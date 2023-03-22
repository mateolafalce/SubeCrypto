use anchor_lang::prelude::*;

#[account]
pub struct SubeAdminAccount {
    pub authority: Pubkey, // 32
    pub bump_original: u8, // 1
    pub prices: Vec<u64>,  // 4 + 8 * 5
}

impl SubeAdminAccount {
    pub const SIZE: usize = 32 + 1 + 4 + (8 * 5);
}