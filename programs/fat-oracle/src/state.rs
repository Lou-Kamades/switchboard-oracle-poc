use anchor_lang::{account, prelude::Pubkey, zero_copy};
#[repr(packed)]
#[account(zero_copy(unsafe))]
pub struct OracleData {
    pub oracle_timestamp: i64,
    pub price: i128,
    pub bump: u8,
}
