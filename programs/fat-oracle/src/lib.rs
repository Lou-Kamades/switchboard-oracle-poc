use anchor_lang::prelude::*;

declare_id!("835WRKhFSAppy7p4QnFBkXJ6Mec3hAx3Jw2X15JKccyi");

#[program]
pub mod fat_oracle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_init()?;
        oracle.bump = *ctx.bumps.get("oracle").unwrap();
        Ok(())
    }

    pub fn save_data(ctx: Context<UpdateOracle>) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_init()?;
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
pub struct UpdateOracle<'info> {
    // ... your required accounts to modify your program's state
    // We use this to derive and verify the functions enclave state
    #[account(
        constraint =
            function.load()?.validate(
              &enclave_signer.to_account_info()
            )?
    )]
    pub function: AccountLoader<'info, FunctionAccountData>,

    #[account(
        mut,
        seeds = [b"oracle"],
        bump
    )]
    pub oracle: AccountLoader<'info, OracleData>,

    pub enclave_signer: Signer<'info>,
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
