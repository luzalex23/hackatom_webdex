use anchor_lang::prelude::*;
use crate::state::SubAccount;
use crate::{ProcessPayment, ValidateToken};
use std::str::FromStr;


pub fn process_payment(ctx: Context<ProcessPayment>, amount: u64, to: Pubkey) -> Result<()> {
    let from = &mut ctx.accounts.from;

    require!(from.balance >= amount, ErrorCode::InsufficientBalance);
    from.balance -= amount;

    // Aqui você pode usar CPI ou lógica off-chain para realmente enviar fundos se necessário
    msg!("Pagamento de {} para {}", amount, to);
    Ok(())
}

pub fn validate_token(token: Pubkey) -> Result<()> {
    let accepted_tokens: Vec<Pubkey> = vec![
        Pubkey::from_str("So11111111111111111111111111111111111111112")
        .map_err(|_| error!(ErrorCode::InvalidPubkey))?
    ];

    require!(accepted_tokens.contains(&token), ErrorCode::TokenNotAllowed);
    msg!("Token validado com sucesso.");
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Saldo insuficiente.")]
    InsufficientBalance,
    #[msg("Token não permitido.")]
    TokenNotAllowed,
    #[msg("Pubkey inválido.")]
    InvalidPubkey,
   
}
