use anchor_lang::prelude::*;

declare_id!("3UxKcHeto7wDea3isc6j33N2ZJJ61J89JyjAdYmQkfWH");

#[program]
pub mod bug_free_contract_3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
