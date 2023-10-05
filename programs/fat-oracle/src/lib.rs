use anchor_lang::prelude::*;

declare_id!("b36ENxZ8qYekipdAfqo7LRE1p98xy6cFNJQyM4o3sgy");

#[program]
pub mod fat_oracle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_init()?;
        oracle.bump = *ctx.bumps.get("oracle").unwrap();
        oracle.price = 0;
        Ok(())
    }

    pub fn ping(_ctx: Context<Ping>) -> anchor_lang::Result<()> {
        msg!("ding dong");
        Ok(())
    }

    pub fn pong(_ctx: Context<Pong>) -> anchor_lang::Result<()> {
        msg!("cool");
        Ok(())
    }

    pub fn update_oracle(ctx: Context<UpdateOracle>) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_mut()?;
        oracle.price += 1;
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
pub struct Ping<'info> {
    #[account(
        constraint =
                switchboard_function.load()?.validate(
                &enclave_signer.to_account_info()
            )? @ OracleError::FunctionValidationFailed
    )]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,

    pub enclave_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Pong<'info> {
    pub signer: Signer<'info>,
}


#[derive(Accounts)]
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
