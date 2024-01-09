use crate::state::accounts::*;
use anchor_lang::{
    prelude::*,
    solana_program::{
        account_info::AccountInfo, program::invoke, pubkey::Pubkey, system_instruction::transfer,
    },
};

pub fn take_a_trip(ctx: Context<TakeATrip>, km: u8) -> Result<()> {
    let authority: Pubkey = ctx.accounts.sube.authority.key();
    let to: Pubkey = ctx.accounts.to.key();
    let from: Pubkey = ctx.accounts.from.key();
    // validations
    require_gte!(MAX_KM, km);
    require_keys_eq!(authority, to);
    let amount: u64 = ctx.accounts.sube.prices[km as usize];
    invoke(
        &transfer(&from, &to, amount),
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.to.to_account_info().clone(),
        ],
    )
    .expect("Error");
    msg!("Paid bus ticket!");
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
