use anchor_lang::prelude::*;

declare_id!("GknYjbiQABncTa8JwStdHRX1t1UZArjdAoaRTrccfhdR");

pub mod instructions;
pub mod state;

#[allow(ambiguous_glob_reexports)]
pub use instructions::*;

pub const PROGRAM_SEED: &[u8] = b"ORACLEPOC";
pub const POC_ORACLE_SEED: &[u8] = b"ORACLE";

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

    pub fn set_oracle_quote_size(
        ctx: Context<SetOracleQuoteSize>,
        params: SetOracleQuoteSizeParams,
    ) -> anchor_lang::Result<()> {
        instructions::set_oracle_quote_size(ctx, params)?;
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

    #[msg("Oracle container is full")]
    OracleContainerFull,

    #[msg("Oracle already added")]
    OracleAlreadyAdded,

    #[msg("Oracle not found")]
    OracleNotFound,

    #[msg("Cannot update oracle with a price that is <= 0")]
    InvalidOraclePrice,

    #[msg("Cannot set oracle quote size to be less than $1")]
    InvalidOracleQuoteSize,
}
