use anchor_lang::prelude::*;

use crate::{state::OracleContainer, ProgramState, POC_ORACLE_SEED, PROGRAM_SEED};

#[derive(Accounts)]
#[instruction(params: SetOracleQuoteSizeParams)]
pub struct SetOracleQuoteSize<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
        has_one = authority
    )]
    pub program: AccountLoader<'info, ProgramState>,
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [POC_ORACLE_SEED],
        bump
    )]
    pub oracle_container: AccountLoader<'info, OracleContainer>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetOracleQuoteSizeParams {
    pub quote_size_usdc_native: u64,
    pub oracle_name: String,
}

pub fn set_oracle_quote_size(
    ctx: Context<SetOracleQuoteSize>,
    params: SetOracleQuoteSizeParams,
) -> anchor_lang::Result<()> {
    let oracle_container = &mut ctx.accounts.oracle_container.load_mut()?;
    oracle_container.set_oracle_quote_size(&params.oracle_name, params.quote_size_usdc_native)?;
    msg!("set_oracle_quote_size: {:?}", params);
    Ok(())
}
