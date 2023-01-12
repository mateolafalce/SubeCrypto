use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Enter a value corresponding to your route")]
    InvalidaKilometer,
}
