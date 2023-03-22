use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Enter a value corresponding to your route")]
    InvalidaKilometer,
    #[msg("This is not the pubkey of the bus line authority")]
    PubkeyError,
}
