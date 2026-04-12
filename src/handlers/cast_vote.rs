     1|use anchor_lang::prelude::*;
     2|use arcium_anchor::{
     3|    queue_computation,
     4|    ARCIUM_CLUSTER_ACCOUNT_SEED, ARCIUM_COMP_DEF_ACCOUNT_SEED,
     5|    ARCIUM_EXECPOOL_ACCOUNT_SEED, ARCIUM_MXE_ACCOUNT_SEED,
     6|    ARCIUM_MEMPOOL_ACCOUNT_SEED,
     7|};
     8|
     9|use crate::state::{DaoConfig, Proposal, ProposalStatus, VoterRecord};
    10|use crate::errors::VotingError;
    11|use crate::COMP_DEF_OFFSET_CAST_VOTE;
    12|
    13|// ─────────────────────────────────────────────────────────────────────────────
    14|// Accounts — cast_vote
    15|// ─────────────────────────────────────────────────────────────────────────────
    16|
    17|#[derive(Accounts)]
    18|#[instruction(computation_offset: u64, encrypted_choice: Vec<u8>, choice_nonce: [u8; 16])]
    19|pub struct CastVote<'info> {
    20|    /// The voter — signs the transaction and pays for the VoterRecord account.
    21|    #[account(mut)]
    22|    pub voter: Signer<'info>,
    23|
    24|    /// DAO configuration — used to validate the proposal belongs to this DAO.
    25|    #[account(
    26|        seeds = [DaoConfig::SEED, dao_config.authority.as_ref()],
    27|        bump = dao_config.bump,
    28|    )]
    29|    pub dao_config: Account<'info, DaoConfig>,
    30|
    31|    /// The proposal being voted on.
    32|    /// Must be Active and within the voting window.
    33|    #[account(
    34|        mut,
    35|        seeds = [
    36|            Proposal::SEED,
    37|            dao_config.key().as_ref(),
    38|            &proposal.id.to_le_bytes(),
    39|        ],
    40|        bump = proposal.bump,
    41|        constraint = proposal.status == ProposalStatus::Active @ VotingError::ProposalNotActive,
    42|    )]
    43|    pub proposal: Account<'info, Proposal>,
    44|
    45|    /// VoterRecord PDA — proves this voter has not already voted.
    46|    /// Derived from [b"voter_record", proposal.key(), voter.key()].
    47|    /// The `init` constraint ensures this fails if the account already exists,
    48|    /// which is the double-vote prevention mechanism.
    49|    #[account(
    50|        init,
    51|        payer = voter,
    52|        space = 8 + VoterRecord::INIT_SPACE,
    53|        seeds = [
    54|            VoterRecord::SEED,
    55|            proposal.key().as_ref(),
    56|            voter.key().as_ref(),
    57|        ],
    58|        bump,
    59|    )]
    60|    pub voter_record: Account<'info, VoterRecord>,
    61|
    62|    // ── Arcium accounts ───────────────────────────────────────────────────────
    63|
    64|    /// Arcium MXE account — the encrypted execution environment.
    65|    /// CHECK: Verified by seeds constraint against the Arcium program.
    66|    #[account(
    67|        mut,
    68|        seeds = [ARCIUM_MXE_ACCOUNT_SEED],
    69|        bump,
    70|        seeds::program = arcium_anchor::ID,
    71|    )]
    72|    pub mxe_account: UncheckedAccount<'info>,
    73|
    74|    /// Arcium mempool — holds pending computation requests.
    75|    /// CHECK: Verified by seeds constraint against the Arcium program.
    76|    #[account(
    77|        mut,
    78|        seeds = [ARCIUM_MEMPOOL_ACCOUNT_SEED],
    79|        bump,
    80|        seeds::program = arcium_anchor::ID,
    81|    )]
    82|    pub mempool_account: UncheckedAccount<'info>,
    83|
    84|    /// Arcium cluster — the MPC node cluster that will execute the circuit.
    85|    /// CHECK: Verified by seeds constraint against the Arcium program.
    86|    #[account(
    87|        seeds = [ARCIUM_CLUSTER_ACCOUNT_SEED],
    88|        bump,
    89|        seeds::program = arcium_anchor::ID,
    90|    )]
    91|    pub cluster_account: UncheckedAccount<'info>,
    92|
    93|    /// Computation definition for `cast_vote` circuit.
    94|    /// CHECK: Verified by seeds constraint against the Arcium program.
    95|    #[account(
    96|        seeds = [
    97|            ARCIUM_COMP_DEF_ACCOUNT_SEED,
    98|            &COMP_DEF_OFFSET_CAST_VOTE.to_le_bytes(),
    99|        ],
   100|        bump,
   101|        seeds::program = arcium_anchor::ID,
   102|    )]
   103|    pub comp_def_account: UncheckedAccount<'info>,
   104|
   105|    /// Arcium execution pool — tracks active computations.
   106|    /// CHECK: Verified by seeds constraint against the Arcium program.
   107|    #[account(
   108|        mut,
   109|        seeds = [ARCIUM_EXECPOOL_ACCOUNT_SEED],
   110|        bump,
   111|        seeds::program = arcium_anchor::ID,
   112|    )]
   113|    pub execpool_account: UncheckedAccount<'info>,
   114|
   115|    /// The Arcium program itself.
   116|    /// CHECK: Verified by address constraint.
   117|    #[account(address = arcium_anchor::ID)]
   118|    pub arcium_program: UncheckedAccount<'info>,
   119|
   120|    pub system_program: Program<'info, System>,
   121|}
   122|
   123|// ─────────────────────────────────────────────────────────────────────────────
   124|// Accounts — cast_vote_callback
   125|// ─────────────────────────────────────────────────────────────────────────────
   126|
   127|#[derive(Accounts)]
   128|pub struct CastVoteCallback<'info> {
   129|    /// The Arcium MXE account — only the MXE can invoke callbacks.
   130|    /// CHECK: Verified by seeds constraint; Arcium enforces the signer check.
   131|    #[account(
   132|        seeds = [ARCIUM_MXE_ACCOUNT_SEED],
   133|        bump,
   134|        seeds::program = arcium_anchor::ID,
   135|    )]
   136|    pub mxe_account: UncheckedAccount<'info>,
   137|
   138|    /// The proposal whose encrypted tally is being updated.
   139|    #[account(mut)]
   140|    pub proposal: Account<'info, Proposal>,
   141|}
   142|
   143|// ─────────────────────────────────────────────────────────────────────────────
   144|// Handler — cast_vote
   145|// ─────────────────────────────────────────────────────────────────────────────
   146|
   147|/// Cast a private vote on a proposal.
   148|///
   149|/// Privacy model:
   150|/// - The voter encrypts their choice (0=yes, 1=no, 2=abstain) client-side
   151|///   using the Arcium shared key, producing Enc<Shared, u8>.
   152|/// - This instruction passes the encrypted choice to the Arcium MPC cluster.
   153|/// - The MPC executes the `cast_vote` circuit, which adds the encrypted vote
   154|///   to the encrypted running tally WITHOUT decrypting either value.
   155|/// - The updated Enc<Mxe, VoteTally> is returned via `cast_vote_callback`.
   156|///
   157|/// Double-vote prevention:
   158|/// - The VoterRecord PDA is initialized with `init`, which fails if the account
   159|///   already exists. This is an on-chain, trustless double-vote guard.
   160|///
   161|/// Flow:
   162|/// 1. Validate proposal is Active and within the voting window.
   163|/// 2. Initialize the VoterRecord (prevents double-voting).
   164|/// 3. Increment the public voter count.
   165|/// 4. Queue a `cast_vote` computation with the encrypted choice.
   166|pub fn handler(
   167|    ctx: Context<CastVote>,
   168|    computation_offset: u64,
   169|    encrypted_choice: Vec<u8>,
   170|    choice_nonce: [u8; 16],
   171|) -> Result<()> {
   172|    // ── Validate encrypted choice format ──────────────────────────────────────
   173|    // Enc<Shared, u8> is a 48-byte ciphertext (Rescue cipher: 32-byte key + 16-byte nonce).
   174|    require!(
   175|        encrypted_choice.len() == 48,
   176|        VotingError::InvalidEncryptedVote
   177|    );
   178|
   179|    // ── Validate voting window ────────────────────────────────────────────────
   180|    let clock = Clock::get()?;
   181|    let now = clock.unix_timestamp;
   182|
   183|    let proposal = &mut ctx.accounts.proposal;
   184|    require!(now >= proposal.start_time, VotingError::VotingNotStarted);
   185|    require!(now <= proposal.end_time, VotingError::VotingEnded);
   186|
   187|    // ── Initialize VoterRecord ────────────────────────────────────────────────
   188|    // This is the double-vote guard: `init` fails if the PDA already exists.
   189|    let voter_record = &mut ctx.accounts.voter_record;
   190|    voter_record.proposal = proposal.key();
   191|    voter_record.voter = ctx.accounts.voter.key();
   192|    voter_record.voted_at = now;
   193|    voter_record.bump = ctx.bumps.voter_record;
   194|
   195|    // ── Increment public voter count ──────────────────────────────────────────
   196|    proposal.total_voters = proposal
   197|        .total_voters
   198|        .checked_add(1)
   199|        .ok_or_else(|| error!(VotingError::Unauthorized))?;
   200|    proposal.pending_computation_offset = computation_offset;
   201|
   202|    // ── Build computation inputs ──────────────────────────────────────────────
   203|    // Input 0: current encrypted tally (Enc<Mxe, VoteTally>) — from on-chain state
   204|    // Input 1: encrypted vote choice (Enc<Shared, u8>) — from the voter
   205|    //
   206|    // The Arcium SDK serializes these as (data, nonce) pairs.
   207|    let mut inputs: Vec<u8> = Vec::new();
   208|
   209|    // Append current encrypted tally + its nonce
   210|    inputs.extend_from_slice(&proposal.encrypted_tally);
   211|    inputs.extend_from_slice(&proposal.tally_nonce);
   212|
   213|    // Append encrypted vote choice + its nonce
   214|    inputs.extend_from_slice(&encrypted_choice);
   215|    inputs.extend_from_slice(&choice_nonce);
   216|
   217|    // ── Queue Arcium `cast_vote` computation ──────────────────────────────────
   218|    queue_computation(
   219|        ctx.accounts.arcium_program.to_account_info(),
   220|        ctx.accounts.mxe_account.to_account_info(),
   221|        ctx.accounts.mempool_account.to_account_info(),
   222|        ctx.accounts.cluster_account.to_account_info(),
   223|        ctx.accounts.comp_def_account.to_account_info(),
   224|        ctx.accounts.execpool_account.to_account_info(),
   225|        ctx.accounts.voter.to_account_info(),
   226|        ctx.accounts.system_program.to_account_info(),
   227|        computation_offset,
   228|        inputs,
   229|        vec![proposal.key()], // accounts to pass to callback
   230|    )?;
   231|
   232|    emit!(VoteCastEvent {
   233|        proposal: proposal.key(),
   234|        voter: ctx.accounts.voter.key(),
   235|        total_voters: proposal.total_voters,
   236|        voted_at: now,
   237|    });
   238|
   239|    msg!(
   240|        "Vote cast on proposal {}. Total voters: {}. Queued cast_vote computation (offset {}).",
   241|        proposal.id,
   242|        proposal.total_voters,
   243|        computation_offset,
   244|    );
   245|
   246|    Ok(())
   247|}
   248|
   249|// ─────────────────────────────────────────────────────────────────────────────
   250|// Callback — cast_vote_callback
   251|// ─────────────────────────────────────────────────────────────────────────────
   252|
   253|/// Arcium MPC callback: receives the updated Enc<Mxe, VoteTally>.
   254|///
   255|/// Called by the Arcium MXE after the `cast_vote` circuit completes.
   256|/// The circuit has homomorphically added the encrypted vote to the encrypted
   257|/// tally — neither the vote choice nor the running tally was ever decrypted.
   258|///
   259|/// This callback simply stores the new ciphertext in the Proposal account.
   260|pub fn callback(
   261|    ctx: Context<CastVoteCallback>,
   262|    updated_tally: Vec<u8>,
   263|    tally_nonce: [u8; 8],
   264|) -> Result<()> {
   265|    require!(!updated_tally.is_empty(), VotingError::InvalidCallbackOutput);
   266|    require!(updated_tally.len() <= 96, VotingError::InvalidEncryptedTally);
   267|
   268|    let proposal = &mut ctx.accounts.proposal;
   269|    proposal.encrypted_tally = updated_tally;
   270|    proposal.tally_nonce = tally_nonce;
   271|
   272|    msg!(
   273|        "Proposal {} encrypted tally updated ({} bytes). Total voters: {}.",
   274|        proposal.id,
   275|        proposal.encrypted_tally.len(),
   276|        proposal.total_voters,
   277|    );
   278|
   279|    Ok(())
   280|}
   281|
   282|// ─────────────────────────────────────────────────────────────────────────────
   283|// Events
   284|// ─────────────────────────────────────────────────────────────────────────────
   285|
   286|#[event]
   287|pub struct VoteCastEvent {
   288|    pub proposal: Pubkey,
   289|    pub voter: Pubkey,
   290|    /// Public count of how many voters have participated (NOT which way they voted)
   291|    pub total_voters: u32,
   292|    pub voted_at: i64,
   293|}
   294|