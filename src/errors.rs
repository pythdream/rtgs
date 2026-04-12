     1|use anchor_lang::prelude::*;
     2|
     3|/// Custom error codes for the DAO Voting program.
     4|/// Anchor assigns error codes starting at 6000.
     5|#[error_code]
     6|pub enum VotingError {
     7|    // ── Timing errors ────────────────────────────────────────────────────────
     8|    #[msg("Voting period has not started yet")]
     9|    VotingNotStarted,
    10|
    11|    #[msg("Voting period has already ended")]
    12|    VotingEnded,
    13|
    14|    #[msg("Voting period is still active — cannot finalize yet")]
    15|    VotingStillActive,
    16|
    17|    // ── State errors ─────────────────────────────────────────────────────────
    18|    #[msg("Proposal is not in Active status")]
    19|    ProposalNotActive,
    20|
    21|    #[msg("Proposal has not ended — status must be VotingEnded before finalizing")]
    22|    ProposalNotEnded,
    23|
    24|    #[msg("Proposal has already been finalized")]
    25|    AlreadyFinalized,
    26|
    27|    #[msg("Proposal has been cancelled")]
    28|    ProposalCancelled,
    29|
    30|    // ── Voter errors ─────────────────────────────────────────────────────────
    31|    #[msg("Voter has already cast a vote on this proposal")]
    32|    AlreadyVoted,
    33|
    34|    // ── Input validation errors ───────────────────────────────────────────────
    35|    #[msg("Encrypted vote data is malformed — expected 48 bytes")]
    36|    InvalidEncryptedVote,
    37|
    38|    #[msg("Proposal title too long — maximum 64 bytes")]
    39|    TitleTooLong,
    40|
    41|    #[msg("Proposal description too long — maximum 256 bytes")]
    42|    DescriptionTooLong,
    43|
    44|    #[msg("Voting duration must be positive")]
    45|    InvalidDuration,
    46|
    47|    // ── Authorization errors ──────────────────────────────────────────────────
    48|    #[msg("Unauthorized — only the DAO authority can perform this action")]
    49|    Unauthorized,
    50|
    51|    // ── Arcium errors ─────────────────────────────────────────────────────────
    52|    #[msg("Arcium computation callback received unexpected output length")]
    53|    InvalidCallbackOutput,
    54|
    55|    #[msg("Encrypted tally data is malformed")]
    56|    InvalidEncryptedTally,
    57|}
    58|