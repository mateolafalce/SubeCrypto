use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sube_crypto {
    use super::*;

    pub fn initialize_admin_account(
        ctx: Context<InitializeAdminAccount>
    ) -> Result<()> {
        let (services_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.user.key().as_ref()], &Pubkey::from_str("Ca8tecWTapYzeGfa8FvAMSo6JCheTRPvQhsjebZm56YE").unwrap());
        let sube: &mut Account<SoLotery> = &mut ctx.accounts.sube;
        sube.authority = ctx.accounts.user.key();
        sube.bump_original = bump;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAdminAccount<'info> {
    #[account(init, seeds = [user.key().as_ref()], bump, payer = user, space = 41)]
    pub sube: Account<'info, SubeAdminAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct SubeAdminAccount {
    pub authority: Pubkey, 
    pub bump_original: u8
}
