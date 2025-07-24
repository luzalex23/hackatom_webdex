use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo, Mint, Token, TokenAccount};

use crate::MintToken;

#[event]
pub struct TokenMinted {
    pub recipient: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum FaucetError {
    #[msg("O valor precisa ser maior que zero.")]
    InvalidAmount,
}

pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
    // Validação
    require!(amount > 0, FaucetError::InvalidAmount);

    //  CPI para mint
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    );

    mint_to(cpi_ctx, amount)?;

    //  Evento
    emit!(TokenMinted {
        recipient: ctx.accounts.recipient.key(),
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("✅ Mintado {} tokens para {}", amount, ctx.accounts.recipient.key());
    Ok(())
}
