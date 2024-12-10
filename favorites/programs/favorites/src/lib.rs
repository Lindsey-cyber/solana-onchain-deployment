use anchor_lang::prelude::*;

declare_id!("7dJBsgUQ65wfq9rsShSwSSFFTzBhtyyN3FdRDtxRNNKG");
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorite(context: Context<SetFavorite>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        let user_public_key = context.accounts.user.key();

        // This works
        msg!("Greeting from {}", user_public_key);

        // This fails
        msg!("Greeting from {user_public_key}");


        // msg!("User {user_public_key}'s favorite number is {number}, color is {color}");
        msg!("User's hobbies are {:?}", hobbies);

        
        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,
    
    #[max_len(4, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorite<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,
    pub system_program: Program<'info, System>,
}
pub struct Initialize {}
