use anchor_lang::{
    prelude::*, solana_program::account_info::AccountInfo, solana_program::pubkey::Pubkey,
    solana_program::system_instruction,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn take_a_trip(
    ctx: Context<TakeATrip>, 
    km: u8
) -> Result<()> {
    require!(km < 5, ErrorCode::InvalidaKilometer);
    let amount: u64 = ctx.accounts.sube.prices[km as usize];
    let transfer =
        system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.to.key(), amount);
    if km == 0 {
        anchor_lang::solana_program::program::invoke(
            &transfer,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info().clone(),
            ],
        )
        .expect("Error");
        msg!("Transfered Lamports");
    }
    Ok(())
}

#[derive(Accounts)]
pub struct TakeATrip<'info> {
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