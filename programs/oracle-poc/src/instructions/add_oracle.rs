use anchor_lang::prelude::*;

use crate::{state::OracleContainer, ProgramState, POC_ORACLE_SEED, PROGRAM_SEED};

#[derive(Accounts)]
#[instruction(params: AddOracleParams)]
pub struct AddOracle<'info> {
    #[account(
        mut,
        seeds = [POC_ORACLE_SEED],
        bump
    )]
    pub oracle_container: AccountLoader<'info, OracleContainer>,

    #[account(
        mut,
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
        has_one = authority
    )]
    pub program: AccountLoader<'info, ProgramState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AddOracleParams {
    pub name: String,
}

pub fn add_oracle(ctx: Context<AddOracle>, params: AddOracleParams) -> anchor_lang::Result<()> {
    let oracle_container = &mut ctx.accounts.oracle_container.load_mut()?;
    oracle_container.add_oracle(&params.name)?;
    msg!("added oracle: {:?}", params);
    Ok(())
}
