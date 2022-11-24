use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use std::mem::size_of;
use anchor_lang::solana_program::log::{
    sol_log_compute_units
};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("GhjiHH45ea3mUuKWHy5dutYMreBLSotzD82ScdY5ES4H");

// Video and comment text length
const TEXT_LENGTH: usize = 1024;
// Username length
const USER_NAME_LENGTH: usize = 100;
// User profile image url length
const USER_URL_LENGTH: usize = 255;
const VIDEO_URL_LENGTH: usize = 255;

const NUMBER_OF_ALLOWED_LIKES_SPACE: usize = 5;
const NUMBER_OF_ALLOWED_LIKES: u8 = 5;

const NUMBER_OF_ALLOWED_FOLLOWING_SPACE: usize = 5;
const NUMBER_OF_ALLOWED_FOLLOWING: u8 = 5;

#[program]
mod tiktok_clone_program {
    use super::*;

    pub fn setup_platform(
        ctx: Context<TikTokPlatform>
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.signer = ctx.accounts.signer.key();
        // Set video count as 0 at this point
        state.video_count = 0;
        Ok(())
    }

    pub fn sign_up_user(
        ctx: Context<SignUpUser>,
        name: String,
        profile_url: String,
    ) -> Result<()> {
        if name.trim().is_empty() || profile_url.trim().is_empty() {
          return Err(Errors::CannotSignUpUser.into());
        }
        if name.as_bytes().len() > USER_NAME_LENGTH {
            return Err(Errors::ExceededNameMaxLength.into());
        }
        if profile_url.as_bytes().len() > USER_URL_LENGTH {
            return Err(Errors::ExceededUserUrlMaxLength.into());
        }
        let user = &mut ctx.accounts.user;
        user.user_wallet_address = ctx.accounts.signer.key();
        user.user_name = name;
        user.user_profile_image_url = profile_url;
        msg!("New User Added!"); //logging
        sol_log_compute_units(); //Logs how many compute units are left, important for budget
        Ok(())
    }

    pub fn upload_video(
        ctx: Context<UploadVideo>,
        description: String,
        video_url: String,
        uploader_name: String,
        uploader_url: String,
    ) -> Result<()> {
        msg!(&description);  //logging
        if description.trim().is_empty() || video_url.trim().is_empty() || uploader_name.trim().is_empty() || uploader_url.trim().is_empty() {
          return Err(Errors::CannotUploadVideo.into());
        }
        if description.as_bytes().len() > TEXT_LENGTH {
            return Err(Errors::ExceededTextMaxLength.into());
        }
        if video_url.as_bytes().len() > VIDEO_URL_LENGTH {
            return Err(Errors::ExceededVideoUrlMaxLength.into());
        }
        if uploader_name.as_bytes().len() > USER_NAME_LENGTH {
            return Err(Errors::ExceededNameMaxLength.into());
        }
        if uploader_url.as_bytes().len() > USER_URL_LENGTH {
            return Err(Errors::ExceededUserUrlMaxLength.into());
        }
        let state = &mut ctx.accounts.state;
        let video = &mut ctx.accounts.video;
        video.signer = ctx.accounts.signer.key();
        video.description = description;
        video.video_url = video_url;
        video.uploader_name = uploader_name;
        video.uploader_url = uploader_url;
        video.comment_count = 0;
        video.index = state.video_count;
        video.creator_time = ctx.accounts.clock.unix_timestamp;
        video.likes = 0;
        video.remove = 0;
        // Increase state's video count by 1
        state.video_count += 1;
        msg!("New Video Added!");  //logging
        sol_log_compute_units(); //Logs how many compute units are left, important for budget
        Ok(())
    }

    pub fn create_comment(
        ctx: Context<CreateComment>,
        text: String,
        commenter_name: String,
        commenter_url: String,
    ) -> Result<()> {
        if text.trim().is_empty() || commenter_name.trim().is_empty() || commenter_url.trim().is_empty() {
          return Err(Errors::CannotCreateComment.into());
        }
        if text.as_bytes().len() > TEXT_LENGTH {
            return Err(Errors::ExceededTextMaxLength.into());
        }
        if commenter_name.as_bytes().len() > USER_NAME_LENGTH {
            return Err(Errors::ExceededNameMaxLength.into());
        }
        if commenter_url.as_bytes().len() > USER_URL_LENGTH {
            return Err(Errors::ExceededUserUrlMaxLength.into());
        }
        let video = &mut ctx.accounts.video;
        if video.remove <= -500 {
            return Err(Errors::UserCensoredVideo.into());
        }
        let comment = &mut ctx.accounts.comment;
        comment.signer = ctx.accounts.signer.key();
        comment.text = text;
        comment.commenter_name = commenter_name;
        comment.commenter_url = commenter_url;
        comment.index = video.comment_count;
        comment.video_time = ctx.accounts.clock.unix_timestamp;
        // Increase video's comment count by 1
        video.comment_count += 1;
        Ok(())
    }

    pub fn approve_video(
        ctx: Context<CreateComment>,
    ) -> Result<()> {
        let video = &mut ctx.accounts.video;
        // Increase video's comment count by 1
        video.remove += 1;
        Ok(())
    }

    pub fn disapprove_video(
        ctx: Context<CreateComment>,
    ) -> Result<()> {
        let video = &mut ctx.accounts.video;
        // Increase video's comment count by 1
        video.remove -= 1;
        Ok(())
    }

    pub fn like_video(ctx: Context<LikeVideo>) -> Result<()> {
        let video = &mut ctx.accounts.video;
        if video.likes == NUMBER_OF_ALLOWED_LIKES {
            return Err(Errors::ReachedMaxLikes.into());
        }
        if video.remove == -500 {
            return Err(Errors::UserCensoredVideo.into());
        }
        let mut iter = video.people_who_liked.iter();
        let user_liking_video = ctx.accounts.signer.key();
        if iter.any(|&v| v == user_liking_video) {
            return Err(Errors::UserLikedVideo.into());
        }
        video.likes += 1;
        video.people_who_liked.push(user_liking_video);
        Ok(())
    }

    pub fn follow_one_another(ctx: Context<FollowOneAnother>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        if user.following == NUMBER_OF_ALLOWED_FOLLOWING {
            return Err(Errors::ReachedMaxFollowing.into());
        }
        let mut iter = user.people_i_follow.iter();
        let user_following = ctx.accounts.signer.key();
        if iter.any(|&v| v == user_following) {
            return Err(Errors::UserIsFollowed.into());
        }
        user.following += 1;
        user.people_i_follow.push(user_following);
        Ok(())
    }
}

/// TikTokPlatform context
#[derive(Accounts)]
pub struct TikTokPlatform<'info> {
    // We must specify the space in order to initialize an account.
    #[account(
        init,
        payer = signer,
        space = size_of::<StateAccount>() + 8, seeds = [b"state".as_ref()], bump
    )]
    pub state: Account<'info, StateAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// SignUpUser context
#[derive(Accounts)]
pub struct SignUpUser<'info> {
    #[account(
        init,
        // User account use string "user" and index of user as seeds
        seeds = [b"user".as_ref(), signer.key().as_ref()],
        bump,
        payer = signer,
        space = size_of::<UserAccount>() + USER_NAME_LENGTH + VIDEO_URL_LENGTH + 8 + 32*NUMBER_OF_ALLOWED_FOLLOWING_SPACE 
    )]
    pub user: Account<'info, UserAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// System program
    /// CHECK: Simple test account for tiktok
    //pub system_program: UncheckedAccount<'info>,
    // Token program
    //#[account(constraint = token_program.key == &token::ID)]
    //pub token_program: Program<'info, Token>,
    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

/// UploadVideo context
#[derive(Accounts)]
pub struct UploadVideo<'info> {
    #[account(mut, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, StateAccount>,
    #[account(
        init,
        // Video account use string "video" and index of video as seeds
        seeds = [b"video".as_ref(), state.video_count.to_be_bytes().as_ref()],
        bump,
        payer = signer,
        space = size_of::<VideoAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH+VIDEO_URL_LENGTH+8+32*NUMBER_OF_ALLOWED_LIKES_SPACE // 32 bits in a pubkey and we have 5
    )]
    pub video: Account<'info, VideoAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// System program
    /// CHECK: Simple test account for tiktok
    //pub system_program: UncheckedAccount<'info>,
    // Token program
    //#[account(constraint = token_program.key == &token::ID)]
    //pub token_program: Program<'info, Token>,
    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

/// CreateComment context
#[derive(Accounts)]
pub struct CreateComment<'info> {
    #[account(mut, seeds = [b"video".as_ref(), video.index.to_be_bytes().as_ref()], bump)]
    pub video: Account<'info, VideoAccount>,
    #[account(
        init,
        // Video account use string "comment", index of video and index of comment per video as seeds
        seeds = [b"comment".as_ref(), video.index.to_be_bytes().as_ref(), video.comment_count.to_be_bytes().as_ref()],
        bump,
        payer = signer,
        space = size_of::<CommentAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH+VIDEO_URL_LENGTH
    )]
    pub comment: Account<'info, CommentAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    // Token program
    //#[account(constraint = token_program.key == &token::ID)]
    //pub token_program: Program<'info, Token>,
    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct LikeVideo<'info> {
    #[account(mut)]
    pub video: Account<'info, VideoAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    // Token program
    //#[account(constraint = token_program.key == &token::ID)]
    //pub token_program: Program<'info, Token>,
    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct FollowOneAnother<'info> {
    #[account(mut)]
    pub user: Account<'info, UserAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    // Token program
    //#[account(constraint = token_program.key == &token::ID)]
    //pub token_program: Program<'info, Token>,
    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut)]
    pub video: Account<'info, VideoAccount>,
}

#[derive(Accounts)]
pub struct DisApprove<'info> {
    #[account(mut)]
    pub video: Account<'info, VideoAccount>,
}

// State Account Structure
#[account]
pub struct StateAccount {
    // Signer address
    pub signer: Pubkey,
    // Video count
    pub video_count: u64,
}

// User Account Structure
#[account]
pub struct UserAccount {
    pub user_name: String,
    pub user_wallet_address: Pubkey,
    // user profile image url
    pub user_profile_image_url: String,
    // following: vector of people who I follow,
    pub people_i_follow: Vec<Pubkey>,
    // number of people I follow
    pub following: u8,
}

// Video Account Structure
#[account]
pub struct VideoAccount {
    // Signer address
    pub signer: Pubkey,
    // description text
    pub description: String,
    // video url
    pub video_url: String,
    // Video uploader name
    pub uploader_name: String,
    // Video uploader url
    pub uploader_url: String,
    // Comment counts of videos
    pub comment_count: u64,
    // Video index
    pub index: u64,
    // Video time
    pub creator_time: i64,
    // likes: vector of people who liked it,
    pub people_who_liked: Vec<Pubkey>,
    // number of likes
    pub likes: u8,
    // number of approvals/disapprovals
    pub remove: i64,
}

// Comment Account Structure
#[account]
pub struct CommentAccount {
    // Signer address
    pub signer: Pubkey,
    // Comment text
    pub text: String,
    // commenter_name
    pub commenter_name: String,
    // commenter_url
    pub commenter_url: String,
    // Comment index
    pub index: u64,
    // Video time
    pub video_time: i64,
}

//#[error]
#[error_code]
pub enum Errors {
    #[msg("User cannot be signed up, missing data")]
    CannotSignUpUser,

    #[msg("Video cannot be created, missing data")]
    CannotUploadVideo,

    #[msg("Comment cannot be created, missing data")]
    CannotCreateComment,

    #[msg("Cannot receive more than 5 likes")]
    ReachedMaxLikes,

    #[msg("User has already liked the tweet")]
    UserLikedVideo,

    #[msg("Cannot follow more than 5 people")]
    ReachedMaxFollowing,

    #[msg("User is already followed")]
    UserIsFollowed,

    #[msg("Video with potentially bad content")]
    UserCensoredVideo,

    #[msg("Exceeded name max length")]
    ExceededNameMaxLength,

    #[msg("Exceeded user url max length")]
    ExceededUserUrlMaxLength,

    #[msg("Exceeded text max length")]
    ExceededTextMaxLength,

    #[msg("Exceeded Video url max length")]
    ExceededVideoUrlMaxLength,
}