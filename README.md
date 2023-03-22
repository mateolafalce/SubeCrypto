<div align="center">

![sube-crypto](sube-crypto.png)


<h1>SUBE CRYPTO</h1>

<h4>A public transport system<h4>

</div>

---

Inspired by the Argentine public transport payment system, this project aims to solve an ongoing issue in the system, which is the slow loading of physical data (which still exists in Argentina today), as well as allowing for decentralized payment, without the need for payment distributors, which are often far from users living outside of major cities.

The program is designed to scale nationally for any nation willing to accept blockchain technology in its public transport system, so if necessary, a function could be created to attach specific user data, but as a demo, it was not considered necessary. This comment also applies to price modifications by bus lines or specific transportation providers.

The program logic allows for the creation of decentralized bus lines (although it could be focused on any other public/private transportation), incentivizing the widespread adoption of new travel lines around the city. On the user side, the means of payment is a specific key pair that anyone can create and credit without the need to go to an official provider, which charges fees for loading. So every time a trip is taken, it is recorded on the Solana blockchain.

Like any blockchain technology, it is not yet in the bidding or practical application process due to regional legislation, but this project aims to establish the foundations of what could be a decentralized and simplified transport system in favor of the consumer and not in favor of the monopolistic bus lines regulated by law.

<h3 align="center">Initialize a bus line</h3>

```rust
pub fn initialize_bus_line(
    ctx: Context<InitializeBusLine>,
    to3km: u64,
    to6km: u64,
    to12km: u64,
    to27km: u64,
    more27km: u64,
) -> Result<()> {
    let (_services_pda, bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[
            ctx.accounts.signer.key().as_ref()
            ],
            ctx.program_id
        );
    let sube: &mut Account<SubeAdminAccount> = &mut ctx.accounts.sube;
    sube.authority = ctx.accounts.signer.key();
    sube.bump_original = bump;
    sube.prices = [
        to3km,
        to6km,
        to12km,
        to27km,
        more27km
    ].to_vec();
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeBusLine<'info> {
    #[account(
        init,
        seeds = [signer.key().as_ref()],
        bump,
        payer = signer,
        space = 8 + SubeAdminAccount::SIZE
    )]
    pub sube: Account<'info, SubeAdminAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

<h3 align="center">Take a trip</h3>

```rust
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
```
