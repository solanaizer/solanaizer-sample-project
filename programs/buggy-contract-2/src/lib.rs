use anchor_lang::prelude::*;

declare_id!("GjQDguFs71C9q154cd9Cr6DwDF5wz62pTdv3u5FxUtgP");

#[program]
pub mod buggy_contract_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
