use anchor_lang::prelude::*;

declare_id!("EJ51jW9nAV2vvfMk3GCqAjhQRiJg6FAugtrXSgXUbmyJ");

#[program]
pub mod buggy_contract_3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
