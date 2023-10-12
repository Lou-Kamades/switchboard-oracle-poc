use anchor_lang::prelude::*;
use switchboard_solana::FunctionAccountData;

use crate::{state::OracleContainer, OracleError, ProgramState, ORACLE_SEED, PROGRAM_SEED};

#[derive(Accounts)]
#[instruction(params: UpdateOracleParams)] // rpc parameters hint
pub struct UpdateOracle<'info> {
    // We need this to validate that the Switchboard Function passed to our program
    // is the expected one.
    #[account(
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
        has_one = switchboard_function
    )]
    pub program: AccountLoader<'info, ProgramState>,

    #[account()]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,

    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump
    )]
    pub oracle_container: AccountLoader<'info, OracleContainer>,

    pub enclave_signer: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateOracleParams {
    pub price_raw: i64,
    pub oracle_name: String,
}

pub fn update_oracle(
    ctx: Context<UpdateOracle>,
    params: UpdateOracleParams,
) -> anchor_lang::Result<()> {
    #[cfg(not(feature = "test"))]
    require!(
        ctx.accounts
            .switchboard_function
            .load()?
            .validate(&ctx.accounts.enclave_signer.to_account_info())
            == Ok(true),
        OracleError::FunctionValidationFailed
    );

    let oracle_container = &mut ctx.accounts.oracle_container.load_mut()?;
    let slot = Clock::get()?.slot;
    oracle_container.update_oracle(&params.oracle_name, params.price_raw as i128, slot)?;
    msg!("updated oracle: {:?}", params);
    Ok(())
}
