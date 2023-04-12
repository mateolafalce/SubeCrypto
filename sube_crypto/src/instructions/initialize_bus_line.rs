use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;

// This function initializes a bus line with different prices for different distances.
pub fn initialize_bus_line(
    ctx: Context<InitializeBusLine>, // A context struct that holds accounts and other contextual information.
    to3km: u64, // Price for a distance of up to 3 km.
    to6km: u64, // Price for a distance between 3 km and 6 km.
    to12km: u64, // Price for a distance between 6 km and 12 km.
    to27km: u64, // Price for a distance between 12 km and 27 km.
    more27km: u64, // Price for a distance greater than 27 km.
) -> Result<()> {
    // Generate a program-derived address (PDA) from the signer's key and program ID.
    let (_services_pda, bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[
            ctx.accounts.signer.key().as_ref()
            ],
            ctx.program_id
        );
    // Get a mutable reference to the SubeAdminAccount associated with the Sube PDA.
    let sube: &mut Account<SubeAdminAccount> = &mut ctx.accounts.sube;
    // Set the authority of the SubeAdminAccount to the signer's key.
    sube.authority = ctx.accounts.signer.key();
    // Set the original bump value of the SubeAdminAccount to the generated bump.
    sube.bump_original = bump;
    // Set the prices of the SubeAdminAccount to the provided prices.
    sube.prices = [
        to3km,
        to6km,
        to12km,
        to27km,
        more27km
    ].to_vec();
    // Return a successful result.
    Ok(())
}

// This struct defines the accounts required for the initialize_bus_line function.
#[derive(Accounts)]
pub struct InitializeBusLine<'info> {
    // The SubeAdminAccount to be initialized.
    #[account(
        init,
        seeds = [signer.key().as_ref()], // Seeds used to generate the PDA.
        bump, // Bump value used to generate the PDA.
        payer = signer, // The account that pays for the creation of the SubeAdminAccount.
        space = 8 + SubeAdminAccount::SIZE // Required space for the SubeAdminAccount.
    )]
    pub sube: Account<'info, SubeAdminAccount>,
    // The signer account that authorizes the transaction.
    #[account(mut)]
    pub signer: Signer<'info>,
    // The system program account.
    pub system_program: Program<'info, System>,
}
