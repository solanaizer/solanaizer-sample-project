use anchor_lang::prelude::*;

declare_id!("5h5NPWfoMQBVWoztmvu13QQw1uzx7TSLSkazzzYw3U3h");

#[program]
pub mod bug_free_contract_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
