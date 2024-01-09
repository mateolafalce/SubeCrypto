use anchor_lang::prelude::*;

const ANCHOR_BUFFER: usize = 8;
pub const MAX_KM: u8 = 4; //(0,1,2,3,4)

#[account]
pub struct SubeAdminAccount {
    pub authority: Pubkey, // 32
    pub bump_original: u8, // 1
    pub prices: Vec<u64>,  // 4 + [8 * 5]
}

impl SubeAdminAccount {
    pub const SIZE: usize = 32 + 1 + 4 + (8 * 5) + ANCHOR_BUFFER;

    pub fn set_authority(&mut self, authority: Pubkey) {
        self.authority = authority;
    }

    pub fn set_bump_original(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn set_prices(&mut self, to3km: u64, to6km: u64, to12km: u64, to27km: u64, more27km: u64) {
        self.prices = [to3km, to6km, to12km, to27km, more27km].to_vec();
    }
}
