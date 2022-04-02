use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

pub const STAKE_MINT_ADDRESS: &str = "BRqWTz3EaeTQZpZuQtc3CVoQaYPoqRXrTDp2LSccgTLN";
pub const BEEF_MINT_ADDRESS: &str = "BbTwPdtD4GoEdjBdYrpTfQQvQqy23f1H8Pd7UvUon41n";


#[derive(Accounts)]
pub struct CreateBeefTokenBag<'info> {
    // 1. PDA (so pubkey) for the soon-to-be created beef token bag for our program.
    #[account(
        init,
        payer = payer,

        // We use the token mint as a seed for the mapping -> think "HashMap[seeds+bump] = pda"
        seeds = [BEEF_MINT_ADDRESS.parse::<Pubkey>().unwrap().as_ref()],
        bump,

        // Token Program wants to know what kind of token this token bag is for
        token::mint = beef_mint,

        // It's a PDA so the authority is itself!
        token::authority = program_beef_token_bag,
    )]
    pub program_beef_token_bag: Account<'info, TokenAccount>,


    // 2. The mint �� because it's needed from above ⬆️ token::mint = ...
    #[account(
        address = BEEF_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub beef_mint: Account<'info, Mint>,
    // 3. The rent payer
    #[account(mut)]
    pub payer: Signer<'info>,


    // 4. Needed from Anchor for the creation of an Associated Token Account
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}



#[derive(Accounts)]
#[instruction(stake_mint_authority_bump: u8, program_beef_bag_bump: u8)]
pub struct Stake<'info> {
    // SPL Token Program
    pub token_program: Program<'info, Token>,


    // ***********
    // MINTING � TO USERS
    // ***********

    // Address of the stake mint ��
    #[account(
    mut,
    address = STAKE_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub stake_mint: Account<'info, Mint>,

    // The authority allowed to mutate the above ⬆️
    // And Print Stake Tokens
    /// CHECK: only used as a signing PDA
    #[account(
    seeds = [ stake_mint.key().as_ref() ],
    bump = stake_mint_authority_bump,
    )]
    pub stake_mint_authority: UncheckedAccount<'info>,

    // Associated Token Account � for User to receive �
    #[account(mut)]
    pub user_stake_token_bag: Account<'info, TokenAccount>,




    // ***********
    // TRANSFERING � FROM USERS
    // ***********

    // Associated Token Account for User which holds �.
    #[account(mut)]
    pub user_beef_token_bag: Account<'info, TokenAccount>,

    // The authority allowed to mutate the above ⬆️
    pub user_beef_token_bag_authority: Signer<'info>,

    // Used to receive � from users
    #[account(
        mut,
        seeds = [ beef_mint.key().as_ref() ],
        bump = program_beef_bag_bump,
    )]
    pub program_beef_token_bag: Account<'info, TokenAccount>,

    // Require for the PDA above ⬆️
    #[account(
        address = BEEF_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub beef_mint: Account<'info, Mint>,
}


#[derive(Accounts)]
#[instruction(program_beef_bag_bump: u8)]
pub struct UnStake<'info> {
    // SPL Token Program
    pub token_program: Program<'info, Token>,


    // ***********
    // BURNING USER'S �
    // ***********

    // see `token::Burn.mint`
    #[account(
        mut,
        address = STAKE_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub stake_mint: Account<'info, Mint>,

    // see `token::Burn.to`
    #[account(mut)]
    pub user_stake_token_bag: Account<'info, TokenAccount>,

    // The authority allowed to mutate the above ⬆️
    pub user_stake_token_bag_authority: Signer<'info>,



    // ***********
    // TRANSFER � TO USERS
    // ***********

    // see `token::Transfer.from`
    #[account(
        mut,
        seeds = [ beef_mint.key().as_ref() ],
        bump = program_beef_bag_bump,
    )]
    pub program_beef_token_bag: Account<'info, TokenAccount>,

    // see `token::Transfer.to`
    #[account(mut)]
    pub user_beef_token_bag: Account<'info, TokenAccount>,

    // Require for the PDA above ⬆️
    #[account(
        address = BEEF_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub beef_mint: Box<Account<'info, Mint>>,
}