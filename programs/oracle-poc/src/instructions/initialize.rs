use anchor_lang::prelude::*;
use switchboard_solana::FunctionAccountData;

use crate::{state::OracleContainer, POC_ORACLE_SEED, PROGRAM_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<ProgramState>(),
        payer = authority,
        seeds = [PROGRAM_SEED],
        bump
    )]
    pub program: AccountLoader<'info, ProgramState>,

    #[account(
        init,
        space = 8 + std::mem::size_of::<OracleContainer>(),
        payer = authority,
        seeds = [POC_ORACLE_SEED],
        bump
    )]
    pub oracle_container: AccountLoader<'info, OracleContainer>,

    pub switchboard_function: Option<AccountLoader<'info, FunctionAccountData>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account(zero_copy(unsafe))]
pub struct ProgramState {
    pub bump: u8,
    pub authority: Pubkey,
    pub switchboard_function: Pubkey,
}

pub fn initialize(ctx: Context<Initialize>) -> anchor_lang::Result<()> {
    let program = &mut ctx.accounts.program.load_init()?;
    program.bump = *ctx.bumps.get("program").unwrap();
    program.authority = ctx.accounts.authority.key();

    let oracle_container = &mut ctx.accounts.oracle_container.load_init()?;
    oracle_container.bump = *ctx.bumps.get("oracle_container").unwrap();

    // Optionally set the switchboard_function if provided
    if let Some(switchboard_function) = ctx.accounts.switchboard_function.as_ref() {
        program.switchboard_function = switchboard_function.key();
    }
    Ok(())
}
