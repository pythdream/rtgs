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
    11|use crate::COMP_DEF_OFFSET_REVEAL_TALLY;
    12|
    13|// ─────────────────────────────────────────────────────────────────────────────
    14|// Accounts — finalize_proposal
    15|// ─────────────────────────────────────────────────────────────────────────────
    16|
    17|#[derive(Accounts)]
    18|#[instruction(computation_offset: u64)]
    19|pub struct FinalizeProposal<'info> {
    20|    /// The DAO authority — only the authority can trigger finalization.
    21|    #[account(mut)]
    22|    pub authority: Signer<'info>,
    23|
    24|    /// DAO configuration — validates the authority.
    25|    #[account(
    26|        seeds = [DaoConfig::SEED, authority.key().as_ref()],
    27|        bump = dao_config.bump,
    28|        constraint = dao_config.authority == authority.key() @ VotingError::Unauthorized,
    29|    )]
    30|    pub dao_config: Account<'info, DaoConfig>,
    31|
    32|    /// The proposal to finalize.
    33|    /// Must be in VotingEnded status (end_time has passed).
    34|    #[account(
    35|        mut,
    36|        seeds = [
    37|            Proposal::SEED,
    38|            dao_config.key().as_ref(),
    39|            &proposal.id.to_le_bytes(),
    40|        ],
    41|        bump = proposal.bump,
    42|        constraint = proposal.status == ProposalStatus::VotingEnded @ VotingError::ProposalNotEnded,
    43|    )]
    44|    pub proposal: Account<'info, Proposal>,
    45|
    46|    // ── Arcium accounts ───────────────────────────────────────────────────────
    47|
    48|    /// Arcium MXE account — the encrypted execution environment.
    49|    /// CHECK: Verified by seeds constraint against the Arcium program.
    50|    #[account(
    51|        mut,
    52|        seeds = [ARCIUM_MXE_ACCOUNT_SEED],
    53|        bump,
    54|        seeds::program = arcium_anchor::ID,
    55|    )]
    56|    pub mxe_account: UncheckedAccount<'info>,
    57|
    58|    /// Arcium mempool — holds pending computation requests.
    59|    /// CHECK: Verified by seeds constraint against the Arcium program.
    60|    #[account(
    61|        mut,
    62|        seeds = [ARCIUM_MEMPOOL_ACCOUNT_SEED],
    63|        bump,
    64|        seeds::program = arcium_anchor::ID,
    65|    )]
    66|    pub mempool_account: UncheckedAccount<'info>,
    67|
    68|    /// Arcium cluster — the MPC node cluster that will execute the circuit.
    69|    /// CHECK: Verified by seeds constraint against the Arcium program.
    70|    #[account(
    71|        seeds = [ARCIUM_CLUSTER_ACCOUNT_SEED],
    72|        bump,
    73|        seeds::program = arcium_anchor::ID,
    74|    )]
    75|    pub cluster_account: UncheckedAccount<'info>,
    76|
    77|    /// Computation definition for `reveal_tally` circuit.
    78|    /// Offset is deterministically derived from the circuit name.
    79|    /// CHECK: Verified by seeds constraint against the Arcium program.
    80|    #[account(
    81|        seeds = [
    82|            ARCIUM_COMP_DEF_ACCOUNT_SEED,
    83|            &COMP_DEF_OFFSET_REVEAL_TALLY.to_le_bytes(),
    84|        ],
    85|        bump,
    86|        seeds::program = arcium_anchor::ID,
    87|    )]
    88|    pub comp_def_account: UncheckedAccount<'info>,
    89|
    90|    /// Arcium execution pool — tracks active computations.
    91|    /// CHECK: Verified by seeds constraint against the Arcium program.
    92|    #[account(
    93|        mut,
    94|        seeds = [ARCIUM_EXECPOOL_ACCOUNT_SEED],
    95|        bump,
    96|        seeds::program = arcium_anchor::ID,
    97|    )]
    98|    pub execpool_account: UncheckedAccount<'info>,
    99|
   100|    /// The Arcium program itself.
   101|    /// CHECK: Verified by address constraint.
   102|    #[account(address = arcium_anchor::ID)]
   103|    pub arcium_program: UncheckedAccount<'info>,
   104|
   105|    pub system_program: Program<'info, System>,
   106|}
   107|
   108|// ─────────────────────────────────────────────────────────────────────────────
   109|// Accounts — finalize_proposal_callback
   110|// ─────────────────────────────────────────────────────────────────────────────
   111|
   112|#[derive(Accounts)]
   113|pub struct FinalizeProposalCallback<'info> {
   114|    /// The Arcium MXE account — only the MXE can invoke callbacks.
   115|    /// CHECK: Verified by seeds constraint; Arcium enforces the signer check.
   116|    #[account(
   117|        seeds = [ARCIUM_MXE_ACCOUNT_SEED],
   118|        bump,
   119|        seeds::program = arcium_anchor::ID,
   120|    )]
   121|    pub mxe_account: UncheckedAccount<'info>,
   122|
   123|    /// The proposal to mark as Finalized with the revealed vote counts.
   124|    #[account(mut)]
   125|    pub proposal: Account<'info, Proposal>,
   126|}
   127|
   128|// ─────────────────────────────────────────────────────────────────────────────
   129|// Handler — finalize_proposal
   130|// ─────────────────────────────────────────────────────────────────────────────
   131|
   132|/// Finalize a proposal after its voting period ends.
   133|///
   134|/// Privacy model:
   135|/// - The encrypted tally (Enc<Mxe, VoteTally>) is passed to the Arcium MPC cluster.
   136|/// - The MPC executes the `reveal_tally` circuit, which decrypts the tally
   137|///   inside the secure enclave and returns only the aggregate counts.
   138|/// - Individual vote choices are NEVER revealed — only the final totals.
   139|///
   140|/// Flow:
   141|/// 1. Validate the proposal is in VotingEnded status.
   142|/// 2. Validate the voting period has actually passed.
   143|/// 3. Queue a `reveal_tally` computation with the encrypted tally.
   144|/// 4. The MPC cluster decrypts and calls `finalize_proposal_callback`
   145|///    with the plaintext (yes, no, abstain) counts.
   146|pub fn handler(ctx: Context<FinalizeProposal>, computation_offset: u64) -> Result<()> {
   147|    let clock = Clock::get()?;
   148|    let now = clock.unix_timestamp;
   149|
   150|    let proposal = &mut ctx.accounts.proposal;
   151|
   152|    // Ensure the voting window has actually closed
   153|    require!(now > proposal.end_time, VotingError::VotingStillActive);
   154|
   155|    // Ensure the proposal has not already been finalized
   156|    require!(
   157|        proposal.status != ProposalStatus::Finalized,
   158|        VotingError::AlreadyFinalized
   159|    );
   160|
   161|    // Update the pending computation offset
   162|    proposal.pending_computation_offset = computation_offset;
   163|
   164|    // ── Build computation inputs ──────────────────────────────────────────────
   165|    // Input: the current encrypted tally (Enc<Mxe, VoteTally>) + its nonce.
   166|    // The `reveal_tally` circuit will decrypt this inside the MXE and return
   167|    // the plaintext (yes, no, abstain) counts via the callback.
   168|    let mut inputs: Vec<u8> = Vec::new();
   169|    inputs.extend_from_slice(&proposal.encrypted_tally);
   170|    inputs.extend_from_slice(&proposal.tally_nonce);
   171|
   172|    // ── Queue Arcium `reveal_tally` computation ───────────────────────────────
   173|    queue_computation(
   174|        ctx.accounts.arcium_program.to_account_info(),
   175|        ctx.accounts.mxe_account.to_account_info(),
   176|        ctx.accounts.mempool_account.to_account_info(),
   177|        ctx.accounts.cluster_account.to_account_info(),
   178|        ctx.accounts.comp_def_account.to_account_info(),
   179|        ctx.accounts.execpool_account.to_account_info(),
   180|        ctx.accounts.authority.to_account_info(),
   181|        ctx.accounts.system_program.to_account_info(),
   182|        computation_offset,
   183|        inputs,
   184|        vec![proposal.key()], // accounts to pass to callback
   185|    )?;
   186|
   187|    emit!(ProposalFinalizingEvent {
   188|        proposal_id: proposal.id,
   189|        proposal: proposal.key(),
   190|        total_voters: proposal.total_voters,
   191|        computation_offset,
   192|    });
   193|
   194|    msg!(
   195|        "Proposal {} finalization queued. Total voters: {}. Computation offset: {}.",
   196|        proposal.id,
   197|        proposal.total_voters,
   198|        computation_offset,
   199|    );
   200|
   201|    Ok(())
   202|}
   203|
   204|// ─────────────────────────────────────────────────────────────────────────────
   205|// Callback — finalize_proposal_callback
   206|// ─────────────────────────────────────────────────────────────────────────────
   207|
   208|/// Arcium MPC callback: receives the decrypted final vote counts.
   209|///
   210|/// Called by the Arcium MXE after the `reveal_tally` circuit completes.
   211|/// This is the ONLY point where vote counts become visible on-chain.
   212|/// Individual votes are never revealed — only the aggregate totals.
   213|///
   214|/// After this callback:
   215|/// - `proposal.status` is set to `Finalized`
   216|/// - `proposal.final_yes/no/abstain` contain the plaintext vote counts
   217|/// - The encrypted tally is cleared (no longer needed)
   218|pub fn callback(
   219|    ctx: Context<FinalizeProposalCallback>,
   220|    final_yes: u32,
   221|    final_no: u32,
   222|    final_abstain: u32,
   223|) -> Result<()> {
   224|    let proposal = &mut ctx.accounts.proposal;
   225|
   226|    // Store the revealed vote counts
   227|    proposal.final_yes = final_yes;
   228|    proposal.final_no = final_no;
   229|    proposal.final_abstain = final_abstain;
   230|
   231|    // Mark the proposal as finalized
   232|    proposal.status = ProposalStatus::Finalized;
   233|
   234|    // Clear the encrypted tally — it's no longer needed and saves space
   235|    proposal.encrypted_tally = Vec::new();
   236|    proposal.tally_nonce = [0u8; 8];
   237|
   238|    emit!(ProposalFinalizedEvent {
   239|        proposal_id: proposal.id,
   240|        proposal: proposal.key(),
   241|        final_yes,
   242|        final_no,
   243|        final_abstain,
   244|        total_voters: proposal.total_voters,
   245|    });
   246|
   247|    msg!(
   248|        "Proposal {} finalized. YES: {}, NO: {}, ABSTAIN: {}. Total voters: {}.",
   249|        proposal.id,
   250|        final_yes,
   251|        final_no,
   252|        final_abstain,
   253|        proposal.total_voters,
   254|    );
   255|
   256|    Ok(())
   257|}
   258|
   259|// ─────────────────────────────────────────────────────────────────────────────
   260|// Events
   261|// ─────────────────────────────────────────────────────────────────────────────
   262|
   263|/// Emitted when finalization is queued (before MPC decryption)
   264|#[event]
   265|pub struct ProposalFinalizingEvent {
   266|    pub proposal_id: u64,
   267|    pub proposal: Pubkey,
   268|    pub total_voters: u32,
   269|    pub computation_offset: u64,
   270|}
   271|
   272|/// Emitted when the final tally is revealed (after MPC decryption)
   273|#[event]
   274|pub struct ProposalFinalizedEvent {
   275|    pub proposal_id: u64,
   276|    pub proposal: Pubkey,
   277|    pub final_yes: u32,
   278|    pub final_no: u32,
   279|    pub final_abstain: u32,
   280|    pub total_voters: u32,
   281|}
   282|