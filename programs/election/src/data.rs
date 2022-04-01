use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct  TweetPlatform<'info> {
    #[account(init, payer = user, space = 9000)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct  WriteTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}

#[derive(Accounts)]
pub struct  LikeTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>
}

#[account]
#[derive(Default)]
pub struct Tweet {
    pub message: String,
    pub likes: u8,
    pub creator: Pubkey,
    pub people_who_liked: Vec<Pubkey>
}