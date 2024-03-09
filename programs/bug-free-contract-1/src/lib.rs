use anchor_lang::prelude::*;

declare_id!("5dtf5Ce9DSQfkSVteaj9ByRHock49YqjMLuAnCrSkYtQ");

#[program]
pub mod bug_free_contract_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
