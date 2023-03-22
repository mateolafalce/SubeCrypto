use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;
use sube_crypto::state::SubeAdminAccount;

pub fn initialize_bus_line(
    program: &Program,
    to3km: u64,
    to6km: u64,
    to12km: u64,
    to27km: u64,
    more27km: u64,
) -> Result<()> {
    let (sube, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[program.payer().as_ref()], &program.id());
    let tx: Signature = program
        .request()
        .accounts(sube_crypto::accounts::InitializeBusLine {
            sube: sube,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(sube_crypto::instruction::InitializeBusLine {
            to3km,
            to6km,
            to12km,
            to27km,
            more27km,
        })
        .send()?;
    let account: SubeAdminAccount = program.account(sube)?;
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    println!("PDA: {}", sube);
    println!("------------------------------------------------------------");
    println!("Authority: {:?}", account.authority);
    println!("------------------------------------------------------------");
    println!("Bump: {:?}", account.bump_original);
    println!("------------------------------------------------------------");
    println!("Prices: {:?}", account.prices);
    println!("------------------------------------------------------------");
    Ok(())
}
