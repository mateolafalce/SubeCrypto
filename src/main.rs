use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
    Client, Cluster,
};
use anyhow::Result;
use std::rc::Rc;
use std::str::FromStr;
pub mod functions;

fn main() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] //cargo test initialize_bus_line -- --show-output
    fn initialize_bus_line() {
        use anchor_client::{
            solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
            Client, Cluster,
        };
        use std::rc::Rc;
        use std::str::FromStr;
        let program = Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("6aARpcVGa9htfRgEgvNe55nFNAkyPykoUXFctpjJ2A1z").unwrap());
        functions::initialize_bus_line::initialize_bus_line(
            &program, 5000, 10000, 15000, 20000, 25000,
        )
        .unwrap();
    }
    #[test] //cargo test take_a_trip -- --show-output
    fn take_a_trip() {
        let program = Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("6aARpcVGa9htfRgEgvNe55nFNAkyPykoUXFctpjJ2A1z").unwrap());
        let sube: Pubkey =
            Pubkey::from_str("GJKNUg1Gm48WCCUbUiDevN5Eddoo382bPAkFbmcq2JNy").unwrap();
        functions::take_a_trip::take_a_trip(&program, sube, 3).unwrap();
    }
}
