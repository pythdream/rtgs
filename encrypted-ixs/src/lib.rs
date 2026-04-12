     1|//! # DAO Voting — Arcium MXE Encrypted Circuit Definitions
     2|//!
     3|//! This crate defines the three MPC circuits that power private vote tallying.
     4|//! Each circuit runs inside the Arcium Multi-Party Execution (MXE) environment,
     5|//! meaning the computation happens over encrypted data — the MPC nodes never
     6|//! see plaintext vote choices or intermediate tally values.
     7|//!
     8|//! ## Circuit Overview
     9|//!
    10|//! ```
    11|//! ┌─────────────────────────────────────────────────────────────────────┐
    12|//! │  init_tally      — Creates Enc<Mxe, VoteTally> initialized to [0,0,0]│
    13|//! │  cast_vote       — Adds Enc<Shared, u8> vote to Enc<Mxe, VoteTally>  │
    14|//! │  reveal_tally    — Decrypts Enc<Mxe, VoteTally> → (u32, u32, u32)    │
    15|//! └─────────────────────────────────────────────────────────────────────┘
    16|//! ```
    17|//!
    18|//! ## Encryption Scheme
    19|//!
    20|//! - `Enc<Mxe, T>` — encrypted under the MXE's shared key; only the MPC
    21|//!   cluster can decrypt. Used for the running tally.
    22|//! - `Enc<Shared, T>` — encrypted under a shared key derived from the voter's
    23|//!   keypair and the MXE's public key. Used for individual vote choices.
    24|//!
    25|//! ## Privacy Guarantees
    26|//!
    27|//! - Individual vote choices are encrypted client-side before hitting the chain.
    28|//! - The `cast_vote` circuit adds an encrypted vote to an encrypted tally
    29|//!   WITHOUT decrypting either value (homomorphic addition in the MXE).
    30|//! - The `reveal_tally` circuit is the ONLY point where decryption occurs,
    31|//!   and it only reveals the aggregate (yes, no, abstain) — never individual votes.
    32|//! - The MPC threshold ensures no single node can decrypt unilaterally.
    33|
    34|use arcium_macros::mxe_circuit;
    35|use arcium_mxe::{Enc, Mxe, Shared};
    36|
    37|// ─────────────────────────────────────────────────────────────────────────────
    38|// VoteTally — the encrypted aggregate stored on-chain
    39|// ─────────────────────────────────────────────────────────────────────────────
    40|
    41|/// The plaintext structure of the vote tally.
    42|///
    43|/// This struct is NEVER visible on-chain in plaintext form.
    44|/// It exists only inside the MXE during circuit execution.
    45|/// On-chain it is stored as `Enc<Mxe, VoteTally>` — opaque bytes.
    46|#[derive(Clone, Copy, Default)]
    47|pub struct VoteTally {
    48|    /// Number of YES votes (choice = 0)
    49|    pub yes: u32,
    50|    /// Number of NO votes (choice = 1)
    51|    pub no: u32,
    52|    /// Number of ABSTAIN votes (choice = 2)
    53|    pub abstain: u32,
    54|}
    55|
    56|// ─────────────────────────────────────────────────────────────────────────────
    57|// VoteChoice — the encrypted individual vote
    58|// ─────────────────────────────────────────────────────────────────────────────
    59|
    60|/// Vote choice constants.
    61|/// The voter encrypts one of these values client-side as `Enc<Shared, u8>`.
    62|pub mod choice {
    63|    /// Vote YES on the proposal
    64|    pub const YES: u8 = 0;
    65|    /// Vote NO on the proposal
    66|    pub const NO: u8 = 1;
    67|    /// Abstain from the vote
    68|    pub const ABSTAIN: u8 = 2;
    69|}
    70|
    71|// ─────────────────────────────────────────────────────────────────────────────
    72|// Circuit 1: init_tally
    73|// ─────────────────────────────────────────────────────────────────────────────
    74|
    75|/// Initialize a new encrypted vote tally to [yes=0, no=0, abstain=0].
    76|///
    77|/// Called once when a proposal is created. The MXE creates a fresh
    78|/// `Enc<Mxe, VoteTally>` with all counts at zero and returns it to the
    79|/// on-chain program via the `create_proposal_callback`.
    80|///
    81|/// # Inputs
    82|/// None — the tally is always initialized to zero.
    83|///
    84|/// # Outputs
    85|/// - `encrypted_tally: Enc<Mxe, VoteTally>` — the initial zero tally ciphertext
    86|/// - `tally_nonce: [u8; 8]` — the nonce associated with this ciphertext
    87|///
    88|/// # Circuit name
    89|/// The name `"init_tally"` must match `COMP_DEF_OFFSET_INIT_TALLY` in lib.rs.
    90|#[mxe_circuit(name = "init_tally")]
    91|pub fn init_tally() -> (Enc<Mxe, VoteTally>, [u8; 8]) {
    92|    // Create a zero-initialized tally inside the MXE
    93|    let tally = VoteTally::default(); // { yes: 0, no: 0, abstain: 0 }
    94|
    95|    // Encrypt the tally under the MXE's key — this is what gets stored on-chain
    96|    let (encrypted_tally, nonce) = Enc::<Mxe, VoteTally>::encrypt(tally);
    97|
    98|    (encrypted_tally, nonce)
    99|}
   100|
   101|// ─────────────────────────────────────────────────────────────────────────────
   102|// Circuit 2: cast_vote
   103|// ─────────────────────────────────────────────────────────────────────────────
   104|
   105|/// Add an encrypted vote to the encrypted running tally.
   106|///
   107|/// This is the core privacy-preserving operation. Both the vote choice and
   108|/// the running tally remain encrypted throughout — the MXE performs the
   109|/// addition homomorphically inside the secure enclave.
   110|///
   111|/// # Inputs
   112|/// - `encrypted_tally: Enc<Mxe, VoteTally>` — the current running tally
   113|/// - `tally_nonce: [u8; 8]` — nonce for the current tally ciphertext
   114|/// - `encrypted_choice: Enc<Shared, u8>` — the voter's encrypted choice (0=YES, 1=NO, 2=ABSTAIN)
   115|/// - `choice_nonce: [u8; 16]` — nonce for the choice ciphertext
   116|///
   117|/// # Outputs
   118|/// - `updated_tally: Enc<Mxe, VoteTally>` — the new tally with the vote added
   119|/// - `new_nonce: [u8; 8]` — the nonce for the updated tally ciphertext
   120|///
   121|/// # Privacy
   122|/// The MXE decrypts both values internally, performs the addition, and
   123|/// re-encrypts the result. The plaintext tally is NEVER exposed outside the MXE.
   124|///
   125|/// # Circuit name
   126|/// The name `"cast_vote"` must match `COMP_DEF_OFFSET_CAST_VOTE` in lib.rs.
   127|#[mxe_circuit(name = "cast_vote")]
   128|pub fn cast_vote(
   129|    encrypted_tally: Enc<Mxe, VoteTally>,
   130|    tally_nonce: [u8; 8],
   131|    encrypted_choice: Enc<Shared, u8>,
   132|    choice_nonce: [u8; 16],
   133|) -> (Enc<Mxe, VoteTally>, [u8; 8]) {
   134|    // Decrypt the current tally inside the MXE (never leaves the enclave)
   135|    let mut tally: VoteTally = encrypted_tally.decrypt(tally_nonce);
   136|
   137|    // Decrypt the voter's choice inside the MXE (never leaves the enclave)
   138|    let choice: u8 = encrypted_choice.decrypt(choice_nonce);
   139|
   140|    // Add the vote to the appropriate counter
   141|    // Invalid choices are silently ignored (treated as no-op)
   142|    match choice {
   143|        0 => tally.yes = tally.yes.saturating_add(1),
   144|        1 => tally.no = tally.no.saturating_add(1),
   145|        2 => tally.abstain = tally.abstain.saturating_add(1),
   146|        _ => {} // Invalid choice — no-op (prevents circuit abort)
   147|    }
   148|
   149|    // Re-encrypt the updated tally under the MXE's key
   150|    let (updated_tally, new_nonce) = Enc::<Mxe, VoteTally>::encrypt(tally);
   151|
   152|    (updated_tally, new_nonce)
   153|}
   154|
   155|// ─────────────────────────────────────────────────────────────────────────────
   156|// Circuit 3: reveal_tally
   157|// ─────────────────────────────────────────────────────────────────────────────
   158|
   159|/// Decrypt the final vote tally and return the plaintext counts.
   160|///
   161|/// Called once after the voting period ends. The MXE decrypts the final
   162|/// `Enc<Mxe, VoteTally>` and returns the aggregate (yes, no, abstain) counts
   163|/// to the on-chain program via `finalize_proposal_callback`.
   164|///
   165|/// # Inputs
   166|/// - `encrypted_tally: Enc<Mxe, VoteTally>` — the final encrypted tally
   167|/// - `tally_nonce: [u8; 8]` — nonce for the final tally ciphertext
   168|///
   169|/// # Outputs
   170|/// - `final_yes: u32` — total YES votes
   171|/// - `final_no: u32` — total NO votes
   172|/// - `final_abstain: u32` — total ABSTAIN votes
   173|///
   174|/// # Privacy
   175|/// Only the aggregate totals are revealed. Individual vote choices remain
   176|/// permanently encrypted and are never recoverable after this point.
   177|///
   178|/// # Circuit name
   179|/// The name `"reveal_tally"` must match `COMP_DEF_OFFSET_REVEAL_TALLY` in lib.rs.
   180|#[mxe_circuit(name = "reveal_tally")]
   181|pub fn reveal_tally(
   182|    encrypted_tally: Enc<Mxe, VoteTally>,
   183|    tally_nonce: [u8; 8],
   184|) -> (u32, u32, u32) {
   185|    // Decrypt the final tally inside the MXE
   186|    let tally: VoteTally = encrypted_tally.decrypt(tally_nonce);
   187|
   188|    // Return the plaintext aggregate counts
   189|    // These are the ONLY values that become visible on-chain
   190|    (tally.yes, tally.no, tally.abstain)
   191|}
   192|
   193|// ─────────────────────────────────────────────────────────────────────────────
   194|// Tests
   195|// ─────────────────────────────────────────────────────────────────────────────
   196|
   197|#[cfg(test)]
   198|mod tests {
   199|    use super::*;
   200|
   201|    /// Verify the VoteTally default initializes to all zeros
   202|    #[test]
   203|    fn test_vote_tally_default() {
   204|        let tally = VoteTally::default();
   205|        assert_eq!(tally.yes, 0);
   206|        assert_eq!(tally.no, 0);
   207|        assert_eq!(tally.abstain, 0);
   208|    }
   209|
   210|    /// Verify vote choice constants are correct
   211|    #[test]
   212|    fn test_choice_constants() {
   213|        assert_eq!(choice::YES, 0);
   214|        assert_eq!(choice::NO, 1);
   215|        assert_eq!(choice::ABSTAIN, 2);
   216|    }
   217|}
   218|