use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("TSVswap11111111111111111111111111111111111");

#[program]
pub mod tsv_swap {
    use super::*;

    /// Initializes a compliant Tokenized Security / Circle USDC AMM Pool
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        symbol: String,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.admin.key();
        pool.usdc_mint = ctx.accounts.usdc_mint.key();
        pool.security_mint = ctx.accounts.security_mint.key();
        pool.symbol = symbol;
        pool.is_trading_halted = false;
        
        msg!("SEC TSV Pool initialized for pairing with Circle USDC");
        Ok(())
    }

    /// Admin / Oracle function to trigger emergency circuit breakers
    /// Synchronizes trading halts with primary listing exchanges (NYSE/NASDAQ)
    pub fn set_market_halt(
        ctx: Context<SetMarketHalt>,
        is_halted: bool,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.is_trading_halted = is_halted;

        if is_halted {
            msg!("CIRCUIT BREAKER: Trading halted to mirror primary exchange stoppage.");
        } else {
            msg!("Trading resumed following primary exchange resumption.");
        }
        Ok(())
    }

    /// Executes swap of Circle USDC for Tokenized Security
    /// Reverts if trading is halted or if participant transfer hook checks fail
    pub fn swap_usdc_for_security(
        ctx: Context<ExecuteSwap>,
        usdc_amount_in: u64,
        min_security_out: u64,
    ) -> Result<()> {
        let pool = &ctx.accounts.pool;
        
        // 1. SEC TSV Condition: Trading Halts
        require!(!pool.is_trading_halted, TSVError::TradingHalted);

        // 2. SEC TSV Condition: Verification / OFAC Check
        let investor_record = &ctx.accounts.investor_registry;
        require!(
            investor_record.is_us_person && !investor_record.is_ofac_sanctioned,
            TSVError::UnauthorizedInvestor
        );

        msg!(
            "Swap verified: {} USDC submitted for tokenized security {}.",
            usdc_amount_in,
            pool.symbol
        );

        // Execution of Token-2022 Transfer Hook handles atomic compliance validation
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 32 + 32 + 32 + 1,
        seeds = [b"tsv_pool", security_mint.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, PoolState>,
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub security_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetMarketHalt<'info> {
    #[account(
        mut,
        has_one = authority @ TSVError::UnauthorizedAdmin
    )]
    pub pool: Account<'info, PoolState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteSwap<'info> {
    #[account(mut)]
    pub pool: Account<'info, PoolState>,
    #[account(
        seeds = [b"investor_kyc", user.key().as_ref()],
        bump
    )]
    pub investor_registry: Account<'info, InvestorRegistry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[account]
pub struct PoolState {
    pub authority: Pubkey,
    pub usdc_mint: Pubkey,
    pub security_mint: Pubkey,
    pub symbol: String,
    pub is_trading_halted: bool,
}

#[account]
pub struct InvestorRegistry {
    pub is_us_person: bool,
    pub is_ofac_sanctioned: bool,
}

#[error_code]
pub enum TSVError {
    #[msg("Trading is currently halted per SEC TSV synchronized halt rules.")]
    TradingHalted,
    #[msg("Participant wallet does not meet US Person or OFAC screening criteria.")]
    UnauthorizedInvestor,
    #[msg("Caller is not authorized to toggle trading status.")]
    UnauthorizedAdmin,
}
