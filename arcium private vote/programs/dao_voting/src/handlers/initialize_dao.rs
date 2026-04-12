     1|use anchor_lang::prelude::*;
     2|use arcium_anchor::{
     3|    ARCIUM_CLUSTER_ACCOUNT_SEED, ARCIUM_MXE_ACCOUNT_SEED,
     4|};
     5|
     6|use crate::state::DaoConfig;
     7|use crate::errors::VotingError;
     8|
     9|// ─────────────────────────────────────────────────────────────────────────────
    10|// Accounts
    11|// ─────────────────────────────────────────────────────────────────────────────
    12|
    13|#[derive(Accounts)]
    14|pub struct InitializeDao<'info> {
    15|    /// The DAO authority — pays for account creation and controls admin actions.
    16|    #[account(mut)]
    17|    pub authority: Signer<'info>,
    18|
    19|    /// The global DAO configuration PDA.
    20|    /// Derived from [b"dao_config", authority.key()].
    21|    /// Created once; subsequent calls will fail because `init` requires the
    22|    /// account to not exist yet.
    23|    #[account(
    24|        init,
    25|        payer = authority,
    26|        space = 8 + DaoConfig::INIT_SPACE,
    27|        seeds = [DaoConfig::SEED, authority.key().as_ref()],
    28|        bump,
    29|    )]
    30|    pub dao_config: Account<'info, DaoConfig>,
    31|
    32|    /// Arcium MXE account — identifies the encrypted execution environment
    33|    /// that will process our private vote computations.
    34|    /// CHECK: Validated by address constraint against the Arcium program's PDA.
    35|    #[account(
    36|        seeds = [ARCIUM_MXE_ACCOUNT_SEED],
    37|        bump,
    38|        seeds::program = arcium_anchor::ID,
    39|    )]
    40|    /// CHECK: Arcium MXE account — address verified by seeds constraint
    41|    pub mxe_account: UncheckedAccount<'info>,
    42|
    43|    /// Arcium cluster account — identifies the MPC node cluster.
    44|    /// CHECK: Validated by address constraint against the Arcium program's PDA.
    45|    #[account(
    46|        seeds = [ARCIUM_CLUSTER_ACCOUNT_SEED],
    47|        bump,
    48|        seeds::program = arcium_anchor::ID,
    49|    )]
    50|    /// CHECK: Arcium cluster account — address verified by seeds constraint
    51|    pub cluster_account: UncheckedAccount<'info>,
    52|
    53|    pub system_program: Program<'info, System>,
    54|}
    55|
    56|// ─────────────────────────────────────────────────────────────────────────────
    57|// Handler
    58|// ─────────────────────────────────────────────────────────────────────────────
    59|
    60|/// Initialize the DAO configuration account.
    61|///
    62|/// This must be called exactly once by the DAO authority before any proposals
    63|/// can be created. It records the Arcium MXE and cluster account addresses so
    64|/// that all subsequent instructions can derive the correct Arcium PDAs.
    65|pub fn handler(ctx: Context<InitializeDao>) -> Result<()> {
    66|    let dao_config = &mut ctx.accounts.dao_config;
    67|
    68|    dao_config.authority = ctx.accounts.authority.key();
    69|    dao_config.proposal_count = 0;
    70|    dao_config.mxe_account = ctx.accounts.mxe_account.key();
    71|    dao_config.cluster_account = ctx.accounts.cluster_account.key();
    72|    dao_config.bump = ctx.bumps.dao_config;
    73|
    74|    emit!(DaoInitializedEvent {
    75|        authority: ctx.accounts.authority.key(),
    76|        dao_config: ctx.accounts.dao_config.key(),
    77|        mxe_account: ctx.accounts.mxe_account.key(),
    78|        cluster_account: ctx.accounts.cluster_account.key(),
    79|    });
    80|
    81|    msg!(
    82|        "DAO initialized. Authority: {}, MXE: {}, Cluster: {}",
    83|        ctx.accounts.authority.key(),
    84|        ctx.accounts.mxe_account.key(),
    85|        ctx.accounts.cluster_account.key(),
    86|    );
    87|
    88|    Ok(())
    89|}
    90|
    91|// ─────────────────────────────────────────────────────────────────────────────
    92|// Events
    93|// ─────────────────────────────────────────────────────────────────────────────
    94|
    95|#[event]
    96|pub struct DaoInitializedEvent {
    97|    pub authority: Pubkey,
    98|    pub dao_config: Pubkey,
    99|    pub mxe_account: Pubkey,
   100|    pub cluster_account: Pubkey,
   101|}
   102|