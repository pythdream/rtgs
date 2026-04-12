     1|use anchor_lang::prelude::*;
     2|
     3|// ─────────────────────────────────────────────────────────────────────────────
     4|// DaoConfig
     5|// ─────────────────────────────────────────────────────────────────────────────
     6|
     7|/// Global DAO configuration — one per deployment.
     8|/// Stores the Arcium MXE and cluster account addresses so all instructions
     9|/// can derive the correct Arcium PDAs without extra parameters.
    10|#[account]
    11|#[derive(InitSpace)]
    12|pub struct DaoConfig {
    13|    /// DAO admin ��� the only account that can finalize proposals
    14|    pub authority: Pubkey,
    15|    /// Sequential counter used to derive proposal PDAs
    16|    pub proposal_count: u64,
    17|    /// Arcium MXE account public key (identifies our encrypted execution environment)
    18|    pub mxe_account: Pubkey,
    19|    /// Arcium cluster account public key
    20|    pub cluster_account: Pubkey,
    21|    /// PDA bump seed
    22|    pub bump: u8,
    23|}
    24|
    25|impl DaoConfig {
    26|    /// Seed used to derive the DaoConfig PDA
    27|    pub const SEED: &'static [u8] = b"dao_config";
    28|}
    29|
    30|// ─────────────────────────────────────────────────────────────────────────────
    31|// Proposal
    32|// ──────────────────���──────────────────────────────────────────────────────────
    33|
    34|/// A single voting proposal.
    35|///
    36|/// Privacy design:
    37|/// - `encrypted_tally` holds Enc<Mxe, VoteTally> — opaque bytes that only the
    38|///   Arcium MXE cluster can decrypt.  It is updated after every cast_vote callback.
    39|/// - `final_yes/no/abstain` are populated ONLY after `finalize_proposal_callback`
    40|///   runs, which happens after `end_time`.
    41|/// - Individual vote choices are NEVER stored on-chain.
    42|#[account]
    43|#[derive(InitSpace)]
    44|pub struct Proposal {
    45|    /// Sequential proposal ID — used as PDA seed
    46|    pub id: u64,
    47|    /// Account that created this proposal
    48|    pub authority: Pubkey,
    49|    /// Proposal title (UTF-8, max 64 bytes)
    50|    #[max_len(64)]
    51|    pub title: String,
    52|    /// Proposal description (UTF-8, max 256 bytes)
    53|    #[max_len(256)]
    54|    pub description: String,
    55|    /// Unix timestamp when voting opens
    56|    pub start_time: i64,
    57|    /// Unix timestamp when voting closes
    58|    pub end_time: i64,
    59|    /// Current lifecycle status
    60|    pub status: ProposalStatus,
    61|    /// Enc<Mxe, VoteTally> — the running encrypted tally maintained by Arcium MPC.
    62|    /// Stored as raw bytes; updated by cast_vote_callback.
    63|    /// Max 96 bytes (Rescue cipher output for a 3-field struct).
    64|    #[max_len(96)]
    65|    pub encrypted_tally: Vec<u8>,
    66|    /// Nonce associated with the current encrypted_tally ciphertext
    67|    pub tally_nonce: [u8; 8],
    68|    /// Final YES vote count — populated by finalize_proposal_callback
    69|    pub final_yes: u32,
    70|    /// Final NO vote count — populated by finalize_proposal_callback
    71|    pub final_no: u32,
    72|    /// Final ABSTAIN vote count — populated by finalize_proposal_callback
    73|    pub final_abstain: u32,
    74|    /// Total number of voters who cast a vote (public counter)
    75|    pub total_voters: u32,
    76|    /// Arcium computation offset used for the pending tally computation
    77|    pub pending_computation_offset: u64,
    78|    /// PDA bump seed
    79|    pub bump: u8,
    80|}
    81|
    82|impl Proposal {
    83|    /// Seed prefix used to derive Proposal PDAs
    84|    pub const SEED: &'static [u8] = b"proposal";
    85|}
    86|
    87|// ─────────────────────────────────────────────────────────────────────────────
    88|// VoterRecord
    89|// ─────────────────────────────────────────────────────────────────────────────
    90|
    91|/// Proof that a voter has participated in a specific proposal.
    92|///
    93|/// Privacy design:
    94|/// - This account records ONLY the fact of voting — NOT the vote choice.
    95|/// - The PDA is derived from (proposal_pubkey, voter_pubkey).
    96|/// - Attempting to `init` this PDA a second time will fail, preventing double-voting.
    97|#[account]
    98|#[derive(InitSpace)]
    99|pub struct VoterRecord {
   100|    /// The proposal this record belongs to
   101|    pub proposal: Pubkey,
   102|    /// The voter's public key
   103|    pub voter: Pubkey,
   104|    /// Unix timestamp when the vote was cast
   105|    pub voted_at: i64,
   106|    /// PDA bump seed
   107|    pub bump: u8,
   108|}
   109|
   110|impl VoterRecord {
   111|    /// Seed prefix used to derive VoterRecord PDAs
   112|    pub const SEED: &'static [u8] = b"voter_record";
   113|}
   114|
   115|// ─────────────────────────────────────────────────────────────────────────────
   116|// ProposalStatus
   117|// ─────────────────────────────────────────────────────────────────────────────
   118|
   119|#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace, Default)]
   120|pub enum ProposalStatus {
   121|    /// Voting is open (start_time <= now <= end_time)
   122|    #[default]
   123|    Active,
   124|    /// Voting period has ended; awaiting finalize_proposal
   125|    VotingEnded,
   126|    /// Tally has been revealed; final_yes/no/abstain are populated
   127|    Finalized,
   128|    /// Proposal was cancelled by the authority
   129|    Cancelled,
   130|}
   131|