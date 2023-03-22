use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::{Keypair, Signature},
        signer::Signer,
        pubkey::Pubkey
    },
    Program,
};
use anyhow::Result;
use sube_crypto;

pub fn take_a_trip(
    program: &Program, 
    sube: Pubkey,
    km: u8
) -> Result<()> {
    let receiver: Keypair = Keypair::new();
    let tx: Signature = program
        .request()
        .accounts(sube_crypto::accounts::TakeATrip {
            sube: sube,
            from: program.payer(),
            to: receiver.pubkey(),
            system_program: system_program::ID,
        })
        .args(sube_crypto::instruction::TakeATrip { 
            km
        })
        .send()?;
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    Ok(())
}