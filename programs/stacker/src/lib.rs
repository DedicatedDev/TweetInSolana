use anchor_lang::prelude::*;
use anchor_spl::token;

declare_id!("HQYiW7oWUMX4WMBdHVZM5gZNVwz7Sbg4sv59ZWREGKEY");

pub mod data;

use data::{
    Stake,
    UnStake,
    CreateBeefTokenBag,
    __client_accounts_stake,
    __client_accounts_un_stake,
    __client_accounts_create_beef_token_bag
};
#[program] 
pub mod staker{

    use super::*;

    pub fn create_beef_token_bag(
        ctx: Context<CreateBeefTokenBag>
    ) -> Result<()> {
        Ok(())
    }


    pub fn stake(
        ctx: Context<Stake>,
        stake_mint_authority_bump: u8,
        _program_beef_bag_bump: u8,
        beef_amount: u64
    ) -> Result<()> {


        // ************************************************************
        // 1. Ask SPL Token Program to mint � to the user.
        // ************************************************************

        let stake_amount = beef_amount; // TODO: Change the formula

        // We know that:
        //                                  findPDA(programId + seed)
        // stakeMintPDA, stakeMintPDABump = findPDA(programId + stakeMint.address)

        // -> So signer can be found using:
        // findPDA(programId + seed)              = X + bump
        // findPDA(programId + stakeMintAddress)  = X + bump
        let stake_mint_address= ctx.accounts.stake_mint.key();
        let seeds = &[stake_mint_address.as_ref(), &[stake_mint_authority_bump]];
        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.stake_mint.to_account_info(),
                to: ctx.accounts.user_stake_token_bag.to_account_info(),
                authority: ctx.accounts.stake_mint_authority.to_account_info(),
            },
            &signer
        );
        token::mint_to(cpi_ctx, stake_amount)?;

        // ************************************************************
        // 2. Ask SPL Token Program to transfer � from the user.
        // ************************************************************
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_beef_token_bag.to_account_info(),
                authority: ctx.accounts.user_beef_token_bag_authority.to_account_info(),
                to: ctx.accounts.program_beef_token_bag.to_account_info(),
            }
        );
        token::transfer(cpi_ctx, beef_amount)?;

        Ok(())
    }

    pub fn unstake(
        ctx: Context<UnStake>,
        program_beef_bag_bump: u8,
        stake_amount: u64
    ) -> Result<()> {

        // ************************************************************
        // 1. Ask SPL Token Program to burn user's �.
        // ************************************************************

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.stake_mint.to_account_info(),
                to: ctx.accounts.user_stake_token_bag.to_account_info(),
                authority: ctx.accounts.user_stake_token_bag_authority.to_account_info(),
            },
        );
        token::burn(cpi_ctx, stake_amount)?;



        // ************************************************************
        // 2. Ask SPL Token Program to transfer back � to the user.
        // ************************************************************

        // See why we did this in `fn stake()`
        let beef_mint_address= ctx.accounts.beef_mint.key();
        let seeds = &[beef_mint_address.as_ref(), &[program_beef_bag_bump]];
        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.program_beef_token_bag.to_account_info(),
                authority: ctx.accounts.program_beef_token_bag.to_account_info(),
                to: ctx.accounts.user_beef_token_bag.to_account_info()
            },
            &signer
        );

        let beef_amount = stake_amount; // TODO: Change the formula
        token::transfer(cpi_ctx, beef_amount)?;

        Ok(())
    }
}

