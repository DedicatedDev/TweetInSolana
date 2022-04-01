
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct  VotePlatform<'info> {
    #[account(init, payer = user, space = 9000)]
    pub candidate: Account<'info, Candidate>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct RegisterCandidate<'info> {
    #[account(mut,has_one = authority)]
    pub candidate: Account<'info, Candidate>,
    pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct  VoteCandidate<'info> {
    #[account(mut)]
    pub candidate: Account<'info, Candidate>
}

#[account]
#[derive(Default)]
pub struct Candidate {
    pub name: String,
    pub age: u8,
    pub des: String,
    pub authority:Pubkey,
    pub people_who_voted: Vec<Pubkey>
}
