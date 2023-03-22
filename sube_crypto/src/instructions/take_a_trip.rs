use anchor_lang::{
    prelude::*,
    solana_program::{
        account_info::AccountInfo,
        pubkey::Pubkey,
        system_instruction,
        program
    },
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn take_a_trip(
    ctx: Context<TakeATrip>,
    km: u8
) -> Result<()> {
    require!(km <= 4, ErrorCode::InvalidaKilometer);
    require!(ctx.accounts.sube.key() == ctx.accounts.to.key(), ErrorCode::PubkeyError);
    let amount: u64 = ctx.accounts.sube.prices[km as usize];
    let transfer =
        system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount
        );
    program::invoke(
            &transfer,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info().clone(),
            ],
        )
        .expect("Error");
        msg!("Paid bus ticket");
    Ok(())
}

#[derive(Accounts)]
pub struct TakeATrip<'info> {
    #[account(
        mut,
        seeds = [sube.authority.key().as_ref()],
        bump = sube.bump_original
    )]
    pub sube: Account<'info, SubeAdminAccount>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
