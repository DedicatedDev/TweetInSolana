use anchor_lang::prelude::*;
declare_id!("6aYgYjZ3aQFMwiYhUZnmBrGNDcoGnAx3UyebJJpx5Fp");

pub mod data;
pub mod errors;

use errors::ErrorCode;
use data::{
    Candidate,
    VotePlatform,
    RegisterCandidate,
    VoteCandidate,
    __client_accounts_vote_platform,
    __client_accounts_register_candidate,
    __client_accounts_vote_candidate
};

#[program] 
pub mod simple_vote {
    use data::VoteCandidate;

    use super::*;

    pub fn setup_platform(ctx: Context<VotePlatform>) -> Result<()> {
        let vote = &mut ctx.accounts.candidate;
        vote.people_who_voted.clear();
        Ok(())
    }

    pub fn register_candidate(ctx: Context<RegisterCandidate>, name:String, age:u8,des:String) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.age = age;
        candidate.name = name;
        candidate.des = des;
        Ok(())
    }

    pub fn vote_candidate(ctx: Context<VoteCandidate>) -> Result<()> {
        
        Ok(())
    }
}


