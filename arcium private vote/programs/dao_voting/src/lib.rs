     1|use anchor_lang::prelude::*;
     2|use arcium_anchor::{
     3|    comp_def_offset,
     4|    ARCIUM_CLUSTER_ACCOUNT_SEED, ARCIUM_COMP_DEF_ACCOUNT_SEED,
     5|    ARCIUM_EXECPOOL_ACCOUNT_SEED, ARCIUM_MXE_ACCOUNT_SEED,
     6|    ARCIUM_MEMPOOL_ACCOUNT_SEED,
     7|};
     8|
     9|pub mod state;
    10|pub mod handlers;
    11|pub mod errors;
    12|
    13|use state::*;
    14|use errors::*;
    15|use handlers::initialize_dao::InitializeDao;
    16|use handlers::create_proposal::{CreateProposal, CreateProposalCallback};
    17|use handlers::cast_vote::{CastVote, CastVoteCallback};
    18|use handlers::finalize_proposal::{FinalizeProposal, FinalizeProposalCallback};
    19|
    20|declare_id!("DAOVot1ngPr1vateVot1ngArc1umMPCxxxxxxxxxxxxxxx");
    21|
    22|/// Computation definition offsets — deterministically derived from circuit names.
    23|/// These must match the circuit names in encrypted-ixs/src/lib.rs.
    24|pub const COMP_DEF_OFFSET_INIT_TALLY: u64 = comp_def_offset(b"init_tally");
    25|pub const COMP_DEF_OFFSET_CAST_VOTE: u64 = comp_def_offset(b"cast_vote");
    26|pub const COMP_DEF_OFFSET_REVEAL_TALLY: u64 = comp_def_offset(b"reveal_tally");
    27|
    28|#[program]
    29|pub mod dao_voting {
    30|    use super::*;
    31|
    32|    /// Initialize the DAO configuration account.
    33|    /// Must be called once by the DAO authority before any proposals can be created.
    34|    pub fn initialize_dao(ctx: Context<InitializeDao>) -> Result<()> {
    35|        handlers::initialize_dao::handler(ctx)
    36|    }
    37|
    38|    /// Create a new voting proposal and initialize its encrypted tally via Arcium MPC.
    39|    /// Queues an `init_tally` computation — the MPC cluster will return an
    40|    /// Enc<Mxe, VoteTally> initialized to [0, 0, 0] via the callback.
    41|    pub fn create_proposal(
    42|        ctx: Context<CreateProposal>,
    43|        computation_offset: u64,
    44|        title: String,
    45|        description: String,
    46|        duration_seconds: i64,
    47|    ) -> Result<()> {
    48|        handlers::create_proposal::handler(
    49|            ctx,
    50|            computation_offset,
    51|            title,
    52|            description,
    53|            duration_seconds,
    54|        )
    55|    }
    56|
    57|    /// Arcium callback: receives the initialized encrypted tally from MPC nodes.
    58|    /// Stores the Enc<Mxe, VoteTally> ciphertext in the Proposal account.
    59|    pub fn create_proposal_callback(
    60|        ctx: Context<CreateProposalCallback>,
    61|        encrypted_tally: Vec<u8>,
    62|        tally_nonce: [u8; 8],
    63|    ) -> Result<()> {
    64|        handlers::create_proposal::callback(ctx, encrypted_tally, tally_nonce)
    65|    }
    66|
    67|    /// Cast a private vote on a proposal.
    68|    /// The vote choice is encrypted client-side with Enc<Shared, u8>.
    69|    /// Queues a `cast_vote` computation — MPC adds the encrypted vote to the
    70|    /// encrypted running tally WITHOUT ever decrypting either value.
    71|    pub fn cast_vote(
    72|        ctx: Context<CastVote>,
    73|        computation_offset: u64,
    74|        encrypted_choice: Vec<u8>,  // Enc<Shared, u8> — 48 bytes
    75|        choice_nonce: [u8; 16],
    76|    ) -> Result<()> {
    77|        handlers::cast_vote::handler(ctx, computation_offset, encrypted_choice, choice_nonce)
    78|    }
    79|
    80|    /// Arcium callback: receives the updated encrypted tally after a vote is added.
    81|    /// Updates the Proposal's encrypted_tally field with the new ciphertext.
    82|    pub fn cast_vote_callback(
    83|        ctx: Context<CastVoteCallback>,
    84|        updated_tally: Vec<u8>,
    85|        tally_nonce: [u8; 8],
    86|    ) -> Result<()> {
    87|        handlers::cast_vote::callback(ctx, updated_tally, tally_nonce)
    88|    }
    89|
    90|    /// Finalize a proposal after its voting period ends.
    91|    /// Queues a `reveal_tally` computation — MPC decrypts the final tally and
    92|    /// returns the plaintext vote counts via the callback.
    93|    /// Only callable after `end_time` has passed.
    94|    pub fn finalize_proposal(
    95|        ctx: Context<FinalizeProposal>,
    96|        computation_offset: u64,
    97|    ) -> Result<()> {
    98|        handlers::finalize_proposal::handler(ctx, computation_offset)
    99|    }
   100|
   101|    /// Arcium callback: receives the decrypted final vote counts.
   102|    /// This is the ONLY point where vote counts become visible on-chain.
   103|    /// Individual votes are never revealed — only the aggregate.
   104|    pub fn finalize_proposal_callback(
   105|        ctx: Context<FinalizeProposalCallback>,
   106|        final_yes: u32,
   107|        final_no: u32,
   108|        final_abstain: u32,
   109|    ) -> Result<()> {
   110|        handlers::finalize_proposal::callback(ctx, final_yes, final_no, final_abstain)
   111|    }
   112|}
   113|