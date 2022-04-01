use anchor_lang::prelude::*;
declare_id!("CxX5JFxfvkeVnCi3MntppR8SYQtmsQXiHYwZqYPEA93s");

pub mod data;
pub mod errors;

use errors::VoteErrorCode;
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

    pub fn setup_vote_platform(ctx: Context<VotePlatform>) -> Result<()> {
        let vote = &mut ctx.accounts.candidate;
        vote.people_who_voted.clear();
        vote.authority = *ctx.accounts.user.key;
        Ok(())
    }

    pub fn register_candidate(ctx: Context<RegisterCandidate>, name:String, age:u8,des:String) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.age = age;
        candidate.name = name;
        candidate.des = des;
        Ok(())
    }

    pub fn vote_candidate(ctx: Context<VoteCandidate>, voter: Pubkey) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        if candidate.people_who_voted.contains(&voter) {
            return Err(VoteErrorCode::CannotVoteAgain.into())
        }else{
            candidate.people_who_voted.push(voter);
        }
        Ok(())
    }


}


