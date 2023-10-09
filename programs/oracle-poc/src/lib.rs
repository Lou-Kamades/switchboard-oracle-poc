use anchor_lang::prelude::*;
use switchboard_solana::FunctionAccountData;

declare_id!("7zNxbvdozQr5zmg6fX3ZpZhWGtoCpUvpSxHXvC25gSWS");

pub const PROGRAM_SEED: &[u8] = b"ORACLEPOC";

#[program]
pub mod oracle_poc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> anchor_lang::Result<()> {
        let program = &mut ctx.accounts.program.load_init()?;
        program.bump = *ctx.bumps.get("program").unwrap();
        program.authority = ctx.accounts.authority.key();

        // Optionally set the switchboard_function if provided
        if let Some(switchboard_function) = ctx.accounts.switchboard_function.as_ref() {
            program.switchboard_function = switchboard_function.key();
        }
        Ok(())
    }

    pub fn set_function(ctx: Context<SetFunction>) -> anchor_lang::Result<()> {
        let program = &mut ctx.accounts.program.load_mut()?;
        program.switchboard_function = ctx.accounts.switchboard_function.key();

        Ok(())
    }

    pub fn add_oracle(ctx: Context<AddOracle>, params: AddOracleParams) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_init()?;
        oracle.name = name_from_str(&params.name)?;
        msg!("added oracle: {:?}", params);
        Ok(())
    }

    pub fn update_oracle(
        ctx: Context<UpdateOracle>,
        params: UpdateOracleParams,
    ) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_mut()?;
        oracle.price = params.price_raw as i128;
        oracle.oracle_timestamp = params.publish_time;
        msg!("updated oracle: {:?}", params);
        Ok(())
    }
}

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

    pub switchboard_function: Option<AccountLoader<'info, FunctionAccountData>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

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

    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
}

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

    #[account(
        constraint =
                switchboard_function.load()?.validate(
                &enclave_signer.to_account_info()
            )? @ OracleError::FunctionValidationFailed
    )]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,

    #[account(
        mut,
        seeds = [&oracle.load()?.name],
        bump
    )]
    pub oracle: AccountLoader<'info, OracleData>,

    pub enclave_signer: Signer<'info>,
}

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
pub struct UpdateOracleParams {
    pub price_raw: i64,
    pub publish_time: i64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AddOracleParams {
    pub name: String,
    // TODO : staleness and other things
}

pub fn name_from_str(name: &str) -> Result<[u8; 16]> {
    let name_bytes = name.as_bytes();
    require!(name_bytes.len() <= 16, OracleError::OracleNameTooLong);
    let mut name_ = [0u8; 16];
    name_[..name_bytes.len()].copy_from_slice(name_bytes);
    Ok(name_)
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum OracleError {
    #[msg("FunctionAccount was not validated successfully")]
    FunctionValidationFailed,

    #[msg("Oracle name must be <= 16 bytes")]
    OracleNameTooLong,
}

#[account(zero_copy(unsafe))]
pub struct ProgramState {
    pub bump: u8,
    pub authority: Pubkey,
    pub switchboard_function: Pubkey,
}

#[repr(packed)]
#[account(zero_copy(unsafe))]
pub struct OracleData {
    pub oracle_timestamp: i64,
    pub price: i128,
    // confidence ??
    pub bump: u8,
    pub name: [u8; 16],
}
