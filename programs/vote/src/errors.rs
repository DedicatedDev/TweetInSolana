use anchor_lang::prelude::*;
#[error_code]
pub enum VoteErrorCode {
    #[msg("Cannot double vote!")]
    CannotVoteAgain,

    #[msg("Message cannot be empty")]
    CannotRegisterAgain,
}