use anchor_lang::prelude::*;

declare_id!("7zNxbvdozQr5zmg6fX3ZpZhWGtoCpUvpSxHXvC25gSWS");

pub mod instructions;

pub use instructions::*;

pub const PROGRAM_SEED: &[u8] = b"ORACLEPOC";

#[program]
pub mod oracle_poc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> anchor_lang::Result<()> {
        instructions::initialize(ctx)?;
        Ok(())
    }

    pub fn set_function(ctx: Context<SetFunction>) -> anchor_lang::Result<()> {
        instructions::set_function(ctx)?;
        Ok(())
    }

    pub fn add_oracle(ctx: Context<AddOracle>, params: AddOracleParams) -> anchor_lang::Result<()> {
        instructions::add_oracle(ctx, params)?;
        Ok(())
    }

    pub fn update_oracle(
        ctx: Context<UpdateOracle>,
        params: UpdateOracleParams,
    ) -> anchor_lang::Result<()> {
        instructions::update_oracle(ctx, params)?;
        Ok(())
    }
}

pub fn name_from_str(name: &str) -> Result<[u8; 16]> {
    let name_bytes = name.as_bytes();
    require!(name_bytes.len() <= 16, OracleError::OracleNameTooLong);
    let mut name_ = [0u8; 16];
    name_[..name_bytes.len()].copy_from_slice(name_bytes);
    Ok(name_)
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum OracleError {
    #[msg("FunctionAccount was not validated successfully")]
    FunctionValidationFailed,

    #[msg("Oracle name must be <= 16 bytes")]
    OracleNameTooLong,
}

#[account(zero_copy(unsafe))]
pub struct ProgramState {
    pub bump: u8,
    pub authority: Pubkey,
    pub switchboard_function: Pubkey,
}

#[repr(packed)]
#[account(zero_copy(unsafe))]
#[derive(Debug)]
pub struct OracleData {
    pub oracle_timestamp: i64,
    pub price: i128,
    // confidence ??
    pub bump: u8,
    pub name: [u8; 16],
}
