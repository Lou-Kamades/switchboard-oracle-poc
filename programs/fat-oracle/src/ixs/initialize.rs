use anchor_lang::prelude::*;

use crate::state::OracleData;

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

pub fn initialize(ctx: Context<Initialize>) -> anchor_lang::Result<()> {
    let oracle = &mut ctx.accounts.oracle.load_init()?;
    oracle.bump = *ctx.bumps.get("oracle").unwrap();
    Ok(())
}
