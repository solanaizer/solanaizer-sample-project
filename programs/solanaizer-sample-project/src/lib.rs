use anchor_lang::prelude::*;

declare_id!("5vAdQX1K57tfQeEn1rp7eRLqmo9rG6Yx2bBiqETEoKDZ");

#[program]
pub mod solanaizer_sample_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
