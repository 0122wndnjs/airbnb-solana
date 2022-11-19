use anchor_lang::prelude::*;
pub mod states;
pub mod constant;
use crate::{constant::*, states::*};

declare_id!("4HneRwTJQfcZsjbeCuQiDB1BY3igLDnMT2aNMyRBtamb");

// Create a DPA
// What is a PDA?  -> Profile Derived Account
// Accounts created from the solana program

#[program]
pub mod clever_airbnb {

    use super::*;
    // Should create a user account with default data
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        OK(())
    }
}

// Create the pda context
#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 32 + 1 + 1 + 8
    )]

    pub user_profile: Box<Account<'info, userProfile>>,

    pub system_program: Program<'info, System>,
}