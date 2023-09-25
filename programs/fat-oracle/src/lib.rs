use crate::ixs::initialize;
use anchor_lang::prelude::*;

mod ixs;
mod state;

declare_id!("D2H2soSaRnbPqJaq2QHtD5Lu7fGJMa9mAKwQCCCqHffB");

pub mod fat_oracle {
    use crate::ixs::update_oracle;

    use super::*;

    pub fn initialize(ctx: Context<initialize::Initialize>) -> anchor_lang::Result<()> {
        initialize::initialize(ctx)?;
        Ok(())
    }

    pub fn save_data(ctx: Context<update_oracle::UpdateOracle>) -> anchor_lang::Result<()> {
        update_oracle::update_oracle(ctx)?;
        Ok(())
    }
}
