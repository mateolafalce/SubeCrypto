use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::ErrorCode;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("6aARpcVGa9htfRgEgvNe55nFNAkyPykoUXFctpjJ2A1z");

#[program]
pub mod sube_crypto {
    use super::*;

    pub fn initialize_bus_line(
        ctx: Context<InitializeBusLine>,
        to3km: u64,
        to6km: u64,
        to12km: u64,
        to27km: u64,
        more27km: u64,
    ) -> Result<()> {
        instructions::initialize_bus_line::initialize_bus_line(
            ctx,
            to3km,
            to6km,
            to12km,
            to27km,
            more27km
        )
    }
    pub fn take_a_trip(
        ctx: Context<TakeATrip>, 
        km: u8
    ) -> Result<()> {
        instructions::take_a_trip::take_a_trip(
            ctx,
            km
        )
        
    }
}