use anchor_lang::prelude::*;

declare_id!("4QFvFTrTbSx1RQoUaWDekhGUbwM54iJwv2kC7YdeVDGu");


pub mod errors;
pub mod data;

use errors::ErrorCode;
use data::{
    TweetPlatform,
    Tweet,
    LikeTweet,
    WriteTweet,
    __client_accounts_like_tweet,
    __client_accounts_tweet_platform,
    __client_accounts_write_tweet,
};

#[program]
pub mod election {
    use data::LikeTweet;

    use super::*;
    pub fn setup_platform(ctx:Context<TweetPlatform>) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet;
        tweet.likes = 0;
        tweet.message = ("").to_string();
        Ok(())
    }

    pub fn write_tweet(
        ctx: Context<WriteTweet>, message: String, user_public_key: Pubkey
    ) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet;
        if !tweet.message.trim().is_empty() {
            return Err(ErrorCode::CannotUpdateTweet.into())
        }
        if message.trim().is_empty() {
            return Err(ErrorCode::EmptyMessage.into())
        }
        tweet.message = message;
        tweet.likes = 0;
        tweet.creator = user_public_key;
        Ok(())
    }

    pub fn like_tweet(ctx: Context<LikeTweet>,user_liking_tweet: Pubkey) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet;

        if tweet.message.trim().is_empty() {
            return Err(ErrorCode::NotValidTweet.into())
        }

        if tweet.likes == 5 {
            return Err(ErrorCode::ReachedMaxLikes.into())
        }

        let mut iter = tweet.people_who_liked.iter();
        if iter.any(|&v| v== user_liking_tweet) {
            return Err(ErrorCode::UserLikedTweet.into())
        }

        tweet.likes += 1;
        tweet.people_who_liked.push(user_liking_tweet);
        Ok(())
    }
    
}

