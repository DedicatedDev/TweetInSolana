use anchor_lang::prelude::*;
use anchor_spl::token::Token;
declare_id!("CxX5JFxfvkeVnCi3MntppR8SYQtmsQXiHYwZqYPEA93s");

#[program] 
pub mod stacking {
    use anchor_spl::token;

    use super::*;

    pub fn stake(ctx: Context<Stake>,amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo{
                mint: ctx.accounts.sta
            },
            &singer
        );
        token::mint_to(cpi_ctx, amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct  Stake<'info> {
    pub token_program: Program<'info, Token>
}