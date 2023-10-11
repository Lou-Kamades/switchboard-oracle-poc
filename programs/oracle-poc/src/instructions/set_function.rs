use anchor_lang::prelude::*;
use switchboard_solana::FunctionAccountData;

use crate::{ProgramState, PROGRAM_SEED};

#[derive(Accounts)]
pub struct SetFunction<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
        has_one = authority
    )]
    pub program: AccountLoader<'info, ProgramState>,
    pub authority: Signer<'info>,

    pub switchboard_function: AccountLoader<'info, FunctionAccountData>, // TODO: more constraints
}

pub fn set_function(ctx: Context<SetFunction>) -> anchor_lang::Result<()> {
    let program = &mut ctx.accounts.program.load_mut()?;
    program.switchboard_function = ctx.accounts.switchboard_function.key();

    Ok(())
}
