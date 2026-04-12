     1|use anchor_lang::prelude::*;
     2|use arcium_anchor::{
     3|    queue_computation,
     4|    ARCIUM_CLUSTER_ACCOUNT_SEED, ARCIUM_COMP_DEF_ACCOUNT_SEED,
     5|    ARCIUM_EXECPOOL_ACCOUNT_SEED, ARCIUM_MXE_ACCOUNT_SEED,
     6|    ARCIUM_MEMPOOL_ACCOUNT_SEED,
     7|};
     8|
     9|use crate::state::{DaoConfig, Proposal, ProposalStatus};
    10|use crate::errors::VotingError;
    11|use crate::COMP_DEF_OFFSET_INIT_TALLY;
    12|
    13|// ─────────────────────────────────────────────────────────────────────────────
    14|// Accounts — create_proposal
    15|// ─────────────────────────────────────────────────────────────────────────────
    16|
    17|#[derive(Accounts)]
    18|#[instruction(computation_offset: u64, title: String, description: String, duration_seconds: i64)]
    19|pub struct CreateProposal<'info> {
    20|    /// The proposal creator — must be the DAO authority.
    21|    #[account(mut)]
    22|    pub authority: Signer<'info>,
    23|
    24|    /// DAO configuration — validates the authority and provides the proposal counter.
    25|    #[account(
    26|        mut,
    27|        seeds = [DaoConfig::SEED, authority.key().as_ref()],
    28|        bump = dao_config.bump,
    29|        constraint = dao_config.authority == authority.key() @ VotingError::Unauthorized,
    30|    )]
    31|    pub dao_config: Account<'info, DaoConfig>,
    32|
    33|    /// The new Proposal PDA.
    34|    /// Derived from [b"proposal", dao_config.key(), proposal_id_le_bytes].
    35|    /// The proposal_id is the current proposal_count before incrementing.
    36|    #[account(
    37|        init,
    38|        payer = authority,
    39|        space = 8 + Proposal::INIT_SPACE,
    40|        seeds = [
    41|            Proposal::SEED,
    42|            dao_config.key().as_ref(),
    43|            &dao_config.proposal_count.to_le_bytes(),
    44|        ],
    45|        bump,
    46|    )]
    47|    pub proposal: Account<'info, Proposal>,
    48|
    49|    // ── Arcium accounts ───────────────────────────────────────────────────────
    50|
    51|    /// Arcium MXE account — the encrypted execution environment.
    52|    /// CHECK: Verified by seeds constraint against the Arcium program.
    53|    #[account(
    54|        mut,
    55|        seeds = [ARCIUM_MXE_ACCOUNT_SEED],
    56|        bump,
    57|        seeds::program = arcium_anchor::ID,
    58|    )]
    59|    pub mxe_account: UncheckedAccount<'info>,
    60|
    61|    /// Arcium mempool — holds pending computation requests.
    62|    /// CHECK: Verified by seeds constraint against the Arcium program.
    63|    #[account(
    64|        mut,
    65|        seeds = [ARCIUM_MEMPOOL_ACCOUNT_SEED],
    66|        bump,
    67|        seeds::program = arcium_anchor::ID,
    68|    )]
    69|    pub mempool_account: UncheckedAccount<'info>,
    70|
    71|    /// Arcium cluster — the MPC node cluster that will execute the circuit.
    72|    /// CHECK: Verified by seeds constraint against the Arcium program.
    73|    #[account(
    74|        seeds = [ARCIUM_CLUSTER_ACCOUNT_SEED],
    75|        bump,
    76|        seeds::program = arcium_anchor::ID,
    77|    )]
    78|    pub cluster_account: UncheckedAccount<'info>,
    79|
    80|    /// Computation definition for `init_tally` circuit.
    81|    /// Offset is deterministically derived from the circuit name.
    82|    /// CHECK: Verified by seeds constraint against the Arcium program.
    83|    #[account(
    84|        seeds = [
    85|            ARCIUM_COMP_DEF_ACCOUNT_SEED,
    86|            &COMP_DEF_OFFSET_INIT_TALLY.to_le_bytes(),
    87|        ],
    88|        bump,
    89|        seeds::program = arcium_anchor::ID,
    90|    )]
    91|    pub comp_def_account: UncheckedAccount<'info>,
    92|
    93|    /// Arcium execution pool — tracks active computations.
    94|    /// CHECK: Verified by seeds constraint against the Arcium program.
    95|    #[account(
    96|        mut,
    97|        seeds = [ARCIUM_EXECPOOL_ACCOUNT_SEED],
    98|        bump,
    99|        seeds::program = arcium_anchor::ID,
   100|    )]
   101|    pub execpool_account: UncheckedAccount<'info>,
   102|
   103|    /// The Arcium program itself.
   104|    /// CHECK: Verified by address constraint.
   105|    #[account(address = arcium_anchor::ID)]
   106|    pub arcium_program: UncheckedAccount<'info>,
   107|
   108|    pub system_program: Program<'info, System>,
   109|}
   110|
   111|// ─────────────────────────────────────────────────────────────────────────────
   112|// Accounts — create_proposal_callback
   113|// ─────────────────────────────────────────────────────────────────────────────
   114|
   115|#[derive(Accounts)]
   116|pub struct CreateProposalCallback<'info> {
   117|    /// The Arcium MXE account — only the MXE can invoke callbacks.
   118|    /// CHECK: Verified by seeds constraint; Arcium enforces the signer check.
   119|    #[account(
   120|        seeds = [ARCIUM_MXE_ACCOUNT_SEED],
   121|        bump,
   122|        seeds::program = arcium_anchor::ID,
   123|    )]
   124|    pub mxe_account: UncheckedAccount<'info>,
   125|
   126|    /// The proposal to update with the initialized encrypted tally.
   127|    #[account(mut)]
   128|    pub proposal: Account<'info, Proposal>,
   129|}
   130|
   131|// ─────────────────────────────────────────────────────────────────────────────
   132|// Handler — create_proposal
   133|// ─────────────────────────────────────────────────────────────────────────────
   134|
   135|/// Create a new voting proposal and initialize its encrypted tally via Arcium MPC.
   136|///
   137|/// Flow:
   138|/// 1. Validate inputs (title/description length, duration > 0).
   139|/// 2. Initialize the Proposal account with metadata.
   140|/// 3. Queue an `init_tally` computation on the Arcium MPC cluster.
   141|/// 4. The MPC cluster executes the circuit and calls `create_proposal_callback`
   142|///    with the initial Enc<Mxe, VoteTally> = Enc([0, 0, 0]).
   143|pub fn handler(
   144|    ctx: Context<CreateProposal>,
   145|    computation_offset: u64,
   146|    title: String,
   147|    description: String,
   148|    duration_seconds: i64,
   149|) -> Result<()> {
   150|    // ── Input validation ──────────────────────────────────────────────────────
   151|    require!(title.len() <= 64, VotingError::TitleTooLong);
   152|    require!(description.len() <= 256, VotingError::DescriptionTooLong);
   153|    require!(duration_seconds > 0, VotingError::InvalidDuration);
   154|
   155|    let clock = Clock::get()?;
   156|    let now = clock.unix_timestamp;
   157|
   158|    // ── Initialize proposal account ───────────────────────────────────────────
   159|    let dao_config = &mut ctx.accounts.dao_config;
   160|    let proposal = &mut ctx.accounts.proposal;
   161|    let proposal_id = dao_config.proposal_count;
   162|
   163|    proposal.id = proposal_id;
   164|    proposal.authority = ctx.accounts.authority.key();
   165|    proposal.title = title.clone();
   166|    proposal.description = description;
   167|    proposal.start_time = now;
   168|    proposal.end_time = now + duration_seconds;
   169|    proposal.status = ProposalStatus::Active;
   170|    proposal.encrypted_tally = Vec::new(); // populated by callback
   171|    proposal.tally_nonce = [0u8; 8];
   172|    proposal.final_yes = 0;
   173|    proposal.final_no = 0;
   174|    proposal.final_abstain = 0;
   175|    proposal.total_voters = 0;
   176|    proposal.pending_computation_offset = computation_offset;
   177|    proposal.bump = ctx.bumps.proposal;
   178|
   179|    // Increment the DAO's proposal counter
   180|    dao_config.proposal_count = proposal_count_next(proposal_id)?;
   181|
   182|    // ── Queue Arcium `init_tally` computation ─────────────────────────────────
   183|    // The MPC cluster will execute the `init_tally` circuit, which returns
   184|    // Enc<Mxe, VoteTally> initialized to { yes: 0, no: 0, abstain: 0 }.
   185|    // The result is delivered via `create_proposal_callback`.
   186|    queue_computation(
   187|        ctx.accounts.arcium_program.to_account_info(),
   188|        ctx.accounts.mxe_account.to_account_info(),
   189|        ctx.accounts.mempool_account.to_account_info(),
   190|        ctx.accounts.cluster_account.to_account_info(),
   191|        ctx.accounts.comp_def_account.to_account_info(),
   192|        ctx.accounts.execpool_account.to_account_info(),
   193|        ctx.accounts.authority.to_account_info(),
   194|        ctx.accounts.system_program.to_account_info(),
   195|        computation_offset,
   196|        vec![],  // init_tally takes no inputs — it creates a fresh zero tally
   197|        vec![ctx.accounts.proposal.key()],  // accounts to pass to callback
   198|    )?;
   199|
   200|    emit!(ProposalCreatedEvent {
   201|        proposal_id,
   202|        proposal: ctx.accounts.proposal.key(),
   203|        authority: ctx.accounts.authority.key(),
   204|        title,
   205|        start_time: now,
   206|        end_time: now + duration_seconds,
   207|    });
   208|
   209|    msg!(
   210|        "Proposal {} created. Voting ends at {}. Queued init_tally computation (offset {}).",
   211|        proposal_id,
   212|        now + duration_seconds,
   213|        computation_offset,
   214|    );
   215|
   216|    Ok(())
   217|}
   218|
   219|// ─────────────────────────────────────────────────────────────────────────────
   220|// Callback — create_proposal_callback
   221|// ─────────────────────────────────────────────────────────────────────────────
   222|
   223|/// Arcium MPC callback: receives the initialized Enc<Mxe, VoteTally>.
   224|///
   225|/// Called by the Arcium MXE after the `init_tally` circuit completes.
   226|/// Stores the encrypted tally ciphertext in the Proposal account so that
   227|/// subsequent `cast_vote` computations can update it.
   228|///
   229|/// Security: Only the Arcium MXE can invoke this instruction (enforced by
   230|/// the `mxe_account` signer check in the Arcium program).
   231|pub fn callback(
   232|    ctx: Context<CreateProposalCallback>,
   233|    encrypted_tally: Vec<u8>,
   234|    tally_nonce: [u8; 8],
   235|) -> Result<()> {
   236|    require!(!encrypted_tally.is_empty(), VotingError::InvalidCallbackOutput);
   237|    require!(encrypted_tally.len() <= 96, VotingError::InvalidEncryptedTally);
   238|
   239|    let proposal = &mut ctx.accounts.proposal;
   240|    proposal.encrypted_tally = encrypted_tally;
   241|    proposal.tally_nonce = tally_nonce;
   242|
   243|    msg!(
   244|        "Proposal {} encrypted tally initialized ({} bytes).",
   245|        proposal.id,
   246|        proposal.encrypted_tally.len(),
   247|    );
   248|
   249|    Ok(())
   250|}
   251|
   252|// ─────────────────────────────────────────────────────────────────────────────
   253|// Helpers
   254|// ─────────────────────────────────────────────────────────────────────────────
   255|
   256|fn proposal_count_next(current: u64) -> Result<u64> {
   257|    current.checked_add(1).ok_or_else(|| error!(VotingError::Unauthorized))
   258|}
   259|
   260|// ─────────────────────────────────────────────────────────────────────────────
   261|// Events
   262|// ─────────────────────────────────────────────────────────────────────────────
   263|
   264|#[event]
   265|pub struct ProposalCreatedEvent {
   266|    pub proposal_id: u64,
   267|    pub proposal: Pubkey,
   268|    pub authority: Pubkey,
   269|    pub title: String,
   270|    pub start_time: i64,
   271|    pub end_time: i64,
   272|}
   273|