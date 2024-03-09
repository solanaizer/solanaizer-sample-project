use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("CCsuJ3u42ioNjNRf9QXqKaM6EeXdv4TVQmcj8j5MUXmk"); 

#[program]
pub mod buggy_contract_1 {
    use super::*;

    pub fn create_timelock(ctx: Context<CreateTimelock>, unlock_time: i64) -> Result<()> {
        let timelock_account = &mut ctx.accounts.timelock_account;
        timelock_account.owner = *ctx.accounts.owner.key;
        timelock_account.unlock_time = unlock_time;
        Ok(())
    }

    // Modified withdraw function to introduce a security vulnerability
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // This function now ignores any checks and transfers tokens to the caller's account

        // Transfer tokens from the timelock account to the caller's account
        let cpi_accounts = Transfer {
            from: ctx.accounts.timelock_token_account.to_account_info(),
            to: ctx.accounts.caller_token_account.to_account_info(), // Sending to the caller's account
            authority: ctx.accounts.timelock_token_account.to_account_info(), // Using the timelock account as authority (vulnerability)
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
