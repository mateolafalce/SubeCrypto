use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;

use instructions::{
    initialize_bus_line::initialize_bus_line, take_a_trip::take_a_trip, InitializeBusLine,
    TakeATrip,
};

declare_id!("6aARpcVGa9htfRgEgvNe55nFNAkyPykoUXFctpjJ2A1z");

#[program]
pub mod sube_crypto {
    use super::*;

    pub fn initialize_bus_line_(
        ctx: Context<InitializeBusLine>,
        to3km: u64,
        to6km: u64,
        to12km: u64,
        to27km: u64,
        more27km: u64,
    ) -> Result<()> {
        initialize_bus_line(ctx, to3km, to6km, to12km, to27km, more27km)
    }

    pub fn take_a_trip_(ctx: Context<TakeATrip>, km: u8) -> Result<()> {
        take_a_trip(ctx, km)
    }
}
