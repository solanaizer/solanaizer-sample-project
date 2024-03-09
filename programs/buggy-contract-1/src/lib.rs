use anchor_lang::prelude::*;

declare_id!("CCsuJ3u42ioNjNRf9QXqKaM6EeXdv4TVQmcj8j5MUXmk");

#[program]
pub mod buggy_contract_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
