use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("CCsuJ3u42ioNjNRf9QXqKaM6EeXdv4TVQmcj8j5MUXmk"); 

#[program]
pub mod buggy_contract_1 {
    use super::*;

    pub fn create_timelock(ctx: Context<CreateTimelock>, unlock_time: i64) -> Result<()> {
        require!(unlock_time >= 0, ErrorCode::InvalidUnlockTime);

        let timelock_account = &mut ctx.accounts.timelock_account;
        
        // Prevent re-initialization of the timelock account
        require!(!timelock_account.is_initialized, ErrorCode::AlreadyInitialized);

        timelock_account.owner = *ctx.accounts.owner.key;
        timelock_account.unlock_time = unlock_time;
        timelock_account.is_initialized = true; // Mark as initialized
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let timelock_account = &ctx.accounts.timelock_account;
    
        // Ensure timelock_account was initialized with a valid unlock_time
        require!(timelock_account.unlock_time > 0, ErrorCode::TimelockNotInitialized);
    
        // Remove the ownership check here to introduce the vulnerability
    
        // Ensure that the current time is greater than the unlock time
        require!(Clock::get()?.unix_timestamp > timelock_account.unlock_time, ErrorCode::UnlockTimeNotReached);
    
        // Transfer tokens from the timelock account to the caller's account (assuming caller's account is passed correctly)
        let cpi_accounts = Transfer {
            from: ctx.accounts.timelock_token_account.to_account_info(),
            to: ctx.accounts.caller_token_account.to_account_info(), // Changed to caller's account
            authority: ctx.accounts.caller.to_account_info(), // Changed to caller as authority
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTimelock<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 1)] // Adjusted space for the is_initialized field
    pub timelock_account: Account<'info, TimelockAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = owner, constraint = timelock_account.unlock_time > 0 && timelock_account.is_initialized)] // Ensure the account is initialized
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
    pub is_initialized: bool, // Added to track initialization status
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