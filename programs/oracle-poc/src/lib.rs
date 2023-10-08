use anchor_lang::prelude::*;

declare_id!("A2h16ZekNmvuFzJCS4MdU1Pe1AwZE2pyFtFDBeaRJQES");

#[program]
pub mod oracle_poc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_init()?;
        oracle.bump = *ctx.bumps.get("oracle").unwrap();
        oracle.price = 0;
        Ok(())
    }

    pub fn update_oracle(ctx: Context<UpdateOracle>, params: UpdateOracleParams) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_mut()?;
        oracle.price = params.price_raw as i128;
        oracle.oracle_timestamp = params.publish_time;
        msg!("updated oracle: {:?}", params);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<OracleData>(),
        payer = payer,
        seeds = [b"oracle"],
        bump
    )]
    pub oracle: AccountLoader<'info, OracleData>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(params: UpdateOracleParams)] // rpc parameters hint
pub struct UpdateOracle<'info> {
    // ... your required accounts to modify your program's state
    // We use this to derive and verify the functions enclave state
        // We use this to verify the functions enclave state was verified successfully
    #[account(
        constraint =
                switchboard_function.load()?.validate(
                &enclave_signer.to_account_info()
            )? @ OracleError::FunctionValidationFailed
    )]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,

    #[account(
        mut,
        seeds = [b"oracle"],
        bump
    )]
    pub oracle: AccountLoader<'info, OracleData>,

    pub enclave_signer: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateOracleParams {
    pub price_raw: i64, 
    pub publish_time: i64,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum OracleError {
    #[msg("FunctionAccount was not validated successfully")]
    FunctionValidationFailed,
}


use anchor_lang::{account, prelude::Pubkey, zero_copy};
use switchboard_solana::FunctionAccountData;
#[repr(packed)]
#[account(zero_copy(unsafe))]
pub struct OracleData {
    pub oracle_timestamp: i64,
    pub price: i128,
    pub bump: u8,
}
