use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("5dtf5Ce9DSQfkSVteaj9ByRHock49YqjMLuAnCrSkYtQ");

#[program]
pub mod bug_free_contract_1 {
    use super::*;

    pub fn create_timelock(ctx: Context<CreateTimelock>, unlock_time: i64) -> Result<()> {
        require!(unlock_time >= 0, ErrorCode::InvalidUnlockTime);

        let timelock_account = &mut ctx.accounts.timelock_account;

        // This check is implicit in the init_if_needed attribute usage below
        // require!(!timelock_account.is_initialized, ErrorCode::AlreadyInitialized);

        timelock_account.owner = *ctx.accounts.owner.key;
        timelock_account.unlock_time = unlock_time;
        timelock_account.is_initialized = true;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let timelock_account = &ctx.accounts.timelock_account;

        // Ensure timelock_account was initialized with a valid unlock_time
        require!(timelock_account.unlock_time > 0, ErrorCode::TimelockNotInitialized);

        // Check if the owner of the withdrawal is the same as the timelock account owner
        require!(timelock_account.owner == *ctx.accounts.owner.key, ErrorCode::Unauthorized);

        // Ensure that the current time is greater than the unlock time
        require!(Clock::get()?.unix_timestamp > timelock_account.unlock_time, ErrorCode::UnlockTimeNotReached);

        // Transfer tokens from the timelock account to the owner's account
        let cpi_accounts = Transfer {
            from: ctx.accounts.timelock_token_account.to_account_info(),
            to: ctx.accounts.owner_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTimelock<'info> {
    #[account(init_if_needed, payer = owner, space = 8 + 32 + 8 + 1)]
    pub timelock_account: Account<'info, TimelockAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = owner, constraint = timelock_account.unlock_time > 0 && timelock_account.is_initialized)]
    pub timelock_account: Account<'info, TimelockAccount>,
    #[account(mut, constraint = timelock_token_account.owner == timelock_account.owner)]
    pub timelock_token_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = owner_token_account.owner == *ctx.accounts.owner.key)]
    pub owner_token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct TimelockAccount {
    pub owner: Pubkey,
    pub unlock_time: i64,
    pub is_initialized: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The unlock time is invalid.")]
    InvalidUnlockTime,
    #[msg("The timelock account has not been initialized.")]
    TimelockNotInitialized,
    #[msg("The unlock time has not yet been reached.")]
    UnlockTimeNotReached,
    #[msg("This timelock account has already been initialized.")]
    AlreadyInitialized,
    #[msg("Unauthorized action.")]
    Unauthorized,
}