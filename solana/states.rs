use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey, // 32
    pub last_airbnb: u8, // 1
    pub airbnb_count: u8, // 1
}