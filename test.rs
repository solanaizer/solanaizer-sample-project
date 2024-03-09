use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFyLsJ9ZedMJr9bG5PuBuQtDpc8xxxXXXXXX"); // Replace with your actual program ID

#[program]
pub mod solana_timelock {
    use super::*;

    pub fn create_timelock(ctx: Context<CreateTimelock>, unlock_time: i64) -> Result<()> {
        let timelock_account = &mut ctx.accounts.timelock_account;
        timelock_account.owner = *ctx.accounts.owner.key;
        timelock_account.unlock_time = unlock_time;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let timelock_account = &ctx.accounts.timelock_account;

        // Ensure that the current time is greater than the unlock time
        require!(
            Clock::get()?.unix_timestamp > timelock_account.unlock_time,
            ErrorCode::UnlockTimeNotReached
        );

        // Transfer tokens from the timelock account to the owner's account
        let cpi_accounts = Transfer {
            from: ctx.accounts.timelock_token_account.to_account_info(),
            to: ctx.accounts.owner_token_account.to_account_info(),
            authority: ctx.accounts.timelock_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTimelock<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8)]
    pub timelock_account: Account<'info, TimelockAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = owner)]
    pub timelock_account: Account<'info, TimelockAccount>,
    #[account(mut)]
    pub timelock_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct TimelockAccount {
    pub owner: Pubkey,
    pub unlock_time: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The unlock time has not yet been reached.")]
    UnlockTimeNotReached,
}