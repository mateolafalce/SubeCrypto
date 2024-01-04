<div align="center">

![sube-crypto](sube-crypto.png)


# SUBE CRYPTO

A public transport system payment 🚍

</div>

---

Inspired by the Argentine public transport payment system, this project aims to solve an ongoing issue in the system, which is the slow loading of physical data (which still exists in Argentina today), as well as allowing for decentralized payment, without the need for payment distributors, which are often far from users living outside of major cities.

The program is designed to scale nationally for any nation willing to accept blockchain technology in its public transport system, so if necessary, a function could be created to attach specific user data, but as a demo, it was not considered necessary. This comment also applies to price modifications by bus lines or specific transportation providers.

The program logic allows for the creation of decentralized bus lines (although it could be focused on any other public/private transportation), incentivizing the widespread adoption of new travel lines around the city. On the user side, the means of payment is a specific key pair that anyone can create and credit without the need to go to an official provider, which charges fees for loading. So every time a trip is taken, it is recorded on the Solana blockchain.

Like any blockchain technology, it is not yet in the bidding or practical application process due to regional legislation, but this project aims to establish the foundations of what could be a decentralized and simplified transport system in favor of the consumer and not in favor of the monopolistic bus lines regulated by law.

## Initialize a bus line

```rust
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
```

This function called initialize_bus_line, is used to initialize a bus line and set the prices to be charged for different travel legs.

It takes as arguments a Context InitializeBusLine structure that contains relevant contextual information, as well as the prices to set for different travel legs. The prices are passed as parameters to the function and are assigned to an array of prices in the SubeAdminAccount that is created or initialized in the function.

The first step of the function is to use the provided signer's public key to calculate a derived public key and a "bump" (an offset value used to protect against key collision attacks). The derived public key is used to seed a SubeAdminAccount, which is set as a seed account. The space argument is used to set the storage space required for the account.

After the account is initialized, the signatory is set as the account authority and the original "bump" is set. The prices are assigned to a price array in the SubeAdminAccount that was just initialized.


## Take a trip

```rust
pub fn take_a_trip(ctx: Context<TakeATrip>, km: u8) -> Result<()> {
    let amount: u64 = ctx.accounts.sube.prices[km as usize];
    let sube: Pubkey = ctx.accounts.sube.key();
    let to: Pubkey = ctx.accounts.to.key();
    let from: Pubkey = ctx.accounts.from.key();
    // validations
    require_gte!(4, km);
    require_keys_eq!(sube, to);
    let transfer = transfer(&from, &to, amount);
    invoke(
        &transfer,
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
```

The first argument is a ctx context object, which contains information about the current transaction. ctx is also expected to have a TakeATrip structure that defines the accounts involved in the transaction.

The second argument is the number of kilometers the user wishes to travel on the bus.

It is verified that the number of kilometers is valid and that the sender of the transaction and the receiver of the transaction are the same accounts that are defined in the TakeATrip structure. The sube account is accessed and the amount of money that must be paid for the ticket is obtained, according to the distance that the user wishes to travel.

An instruction is created to transfer the cryptocurrency or token from the sending account to the receiving account, with the previously calculated amount of money. The function program::invoke is called to execute the transfer statement.
