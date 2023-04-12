use anchor_lang::{
    prelude::*,
    solana_program::{
        account_info::AccountInfo,
        pubkey::Pubkey,
        system_instruction,
        program
    },
};
use crate::state::accounts::*; // Import state module with account structs
use crate::errors::ErrorCode; // Import module with error codes

// Define the take_a_trip function which takes a Context object and a km value and returns a Result object
pub fn take_a_trip(
    ctx: Context<TakeATrip>,
    km: u8
) -> Result<()> {
    // Check that the km value is valid (less than or equal to 4)
    require!(km <= 4, ErrorCode::InvalidaKilometer);
    // Check that the sube authority and the "to" account have the same public key
    require!(ctx.accounts.sube.key() == ctx.accounts.to.key(), ErrorCode::PubkeyError);
    // Get the price for the specified kilometer from the sube account
    let amount: u64 = ctx.accounts.sube.prices[km as usize];
    // Create a transfer instruction to send the payment from the "from" account to the "to" account
    let transfer =
        system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount
        );
    // Invoke the transfer instruction using the Solana system program
    program::invoke(
            &transfer,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info().clone(),
            ],
        )
        .expect("Error"); // Handle any errors that may occur during the transfer
    // Print a message to indicate that the bus ticket has been paid
    msg!("Paid bus ticket");
    // Return a success result
    Ok(())
}

// Define a struct to hold the accounts required by the take_a_trip function
#[derive(Accounts)]
pub struct TakeATrip<'info> {
    #[account(
        mut,
        seeds = [sube.authority.key().as_ref()], // Use the sube account's authority key as the seed
        bump = sube.bump_original // Use the sube account's original bump value
    )]
    pub sube: Account<'info, SubeAdminAccount>, // Account representing the sube administrator
    #[account(mut, signer)] // Specify that the "from" account must be mutable and signed
    pub from: AccountInfo<'info>, // Account representing the account that pays for the bus ticket
    #[account(mut)] // Specify that the "to" account must be mutable
    pub to: AccountInfo<'info>, // Account representing the account that receives payment for the bus ticket
    pub system_program: Program<'info, System>, // The Solana system program account
}
