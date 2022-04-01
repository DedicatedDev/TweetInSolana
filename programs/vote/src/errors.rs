use anchor_lang::prelude::*;
#[error_code]
pub enum ErrorCode {
    #[msg("Cannot double vote!")]
    CannotVoteAgain,

    #[msg("Message cannot be empty")]
    CannotRegisterAgain,
}