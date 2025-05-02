use anchor_lang::prelude::*;
use crate::state::SubAccount;
use crate::{ProcessPayment, ValidateToken};
use std::str::FromStr;

pub fn process_payment(ctx: Context<ProcessPayment>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.from;
    let to = &mut ctx.accounts.to_account;

    require!(from.balance >= amount, ErrorCode::InsufficientBalance);

    from.balance = from.balance.checked_sub(amount).ok_or(ErrorCode::MathOverflow)?;
    to.balance = to.balance.checked_add(amount).ok_or(ErrorCode::MathOverflow)?;

    msg!(
        "Transferência de {} de {} para {}",
        amount,
        from.owner,
        to.owner
    );

    Ok(())
}

pub fn pay_fee(ctx: Context<ProcessPayment>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.from;

    require!(from.balance >= amount, ErrorCode::InsufficientBalance);
    from.balance -= amount;

    msg!("Taxa de {} paga pelo usuário {}", amount, ctx.accounts.owner.key());
    Ok(())
}

pub fn withdraw(ctx: Context<ProcessPayment>, amount: u64, fee_percent: u64) -> Result<()> {
    let from = &mut ctx.accounts.from;

    require!(from.balance >= amount, ErrorCode::InsufficientBalance);
    let fee = amount.checked_mul(fee_percent).ok_or(ErrorCode::Overflow)?.checked_div(100).ok_or(ErrorCode::Overflow)?;
    let net_amount = amount.checked_sub(fee).ok_or(ErrorCode::Overflow)?;

    from.balance -= amount;

    msg!("Usuário {} sacou {} ({} de taxa)", ctx.accounts.owner.key(), net_amount, fee);
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
    #[msg("Overflow ao calcular valores.")]
    Overflow,
    #[msg("Overflow aritmético")]
    MathOverflow,
    #[msg("Conta de destino não foi fornecida")]
    DestinationAccountNotProvided,
}
