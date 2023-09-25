use anchor_lang::prelude::*;
use switchboard_solana::FunctionAccountData;

use crate::state::OracleData;

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

pub fn update_oracle(ctx: Context<UpdateOracle>) -> anchor_lang::Result<()> {
    let oracle = &mut ctx.accounts.oracle.load_init()?;
    oracle.price += 1;
    Ok(())
}
