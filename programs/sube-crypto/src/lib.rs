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
    pub fn take_a_trip(
        ctx: Context<Trip>
    ) -> Result<()> {
        //TODO
        let _lamports: u64 = 1000000000;
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;
        let transfer = system_instruction::transfer(
            &from_pubkey, &to_pubkey, _lamports,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer,
            &[from.to_account_info(), to.to_account_info().clone()],
        ).expect("Error");
        msg!("Transfered Lamports");
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
#[derive(Accounts)]
pub struct Trip<'info> {
    #[account(mut, seeds = [sube.authority.key().as_ref()], bump = sube.bump_original)]
    pub sube: Account<'info, SubeAdminAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct SubeAdminAccount {
    pub authority: Pubkey, 
    pub bump_original: u8
}
