use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
    },
    Program,
};
use anyhow::Result;
use sube_crypto::{instruction::TakeATrip , accounts::TakeATrip};

pub fn take_a_trip(program: &Program, sube: Pubkey, km: u8) -> Result<()> {
    let receiver: Keypair = Keypair::new();
    let tx: Signature = program
        .request()
        .accounts(TakeATrip {
            sube: sube,
            from: program.payer(),
            to: receiver.pubkey(),
            system_program: system_program::ID,
        })
        .args(TakeATrip { km })
        .send()?;
    println!("Tx: {}", tx);
    Ok(())
}
