use anchor_lang::prelude::*;
use anchor_spl::token::Token;

pub const STAKE_MINT_ADDRESS: &str =  "9FgzyMYYiQew42BdVjsKNHUeXDpP4CaK1rFLMQndf1xE";
pub const BEEF_MINT_ADDRESS: &str = "AXyTBL1C48WEdpzpY4bcDNsG4B2N918zy2cYsiQFKGBf";

#[derive(Accounts)]
pub struct CreateBeefTokenBag<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [ BEEF_MINT_ADDRESS.parse::<Pubkey>().unwrap().as_ref()],
        bump,
        token::mint = beef_mint,
        token::authority = program_beef_token_bag,
    )]
    pub program_beef_token_bag: Account<'info, TokenAccount>

    #[account]

}