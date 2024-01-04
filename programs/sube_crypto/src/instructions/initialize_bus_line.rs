use crate::state::accounts::*;
use anchor_lang::{
    prelude::*,
    solana_program::{account_info::AccountInfo, pubkey::Pubkey},
};

pub fn initialize_bus_line(
    ctx: Context<InitializeBusLine>,
    to3km: u64,
    to6km: u64,
    to12km: u64,
    to27km: u64,
    more27km: u64,
) -> Result<()> {
    let signer: Pubkey = ctx.accounts.signer.key();
    let program_id: &Pubkey = ctx.program_id;
    let (_services_pda, bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[signer.as_ref()], program_id);
    let sube: &mut Account<SubeAdminAccount> = &mut ctx.accounts.sube;
    sube.authority = signer;
    sube.bump_original = bump;
    sube.prices = [to3km, to6km, to12km, to27km, more27km].to_vec();
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeBusLine<'info> {
    #[account(init, seeds = [signer.key().as_ref()], bump, payer = signer, space = 8 + SubeAdminAccount::SIZE)]
    pub sube: Account<'info, SubeAdminAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
