use anchor_lang::prelude::*;

use crate::{name_from_str, OracleData, ProgramState, PROGRAM_SEED};

#[derive(Accounts)]
#[instruction(params: AddOracleParams)]
pub struct AddOracle<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<OracleData>(),
        payer = authority,
        seeds = [&name_from_str(&params.name).unwrap()],
        bump
    )]
    pub oracle: AccountLoader<'info, OracleData>,

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
    // TODO : staleness and other things
}

pub fn add_oracle(ctx: Context<AddOracle>, params: AddOracleParams) -> anchor_lang::Result<()> {
    let oracle = &mut ctx.accounts.oracle.load_init()?;
    oracle.bump = *ctx.bumps.get("oracle").unwrap();
    oracle.name = name_from_str(&params.name)?;
    msg!("added oracle: {:?}", params);
    Ok(())
}
