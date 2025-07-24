use anchor_lang::prelude::*;
use crate::state::SubAccount;
use crate::errors::PaymentError;
use crate::{ProcessPayment, ValidateToken, Withdraw};
use std::str::FromStr;

#[event]
pub struct PaymentProcessed {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct FeePaid {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct Withdrawn {
    pub user: Pubkey,
    pub net_amount: u64,
    pub fee_amount: u64,
    pub fee_percent: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokenValidated {
    pub token: Pubkey,
    pub timestamp: i64,
}

pub fn process_payment(ctx: Context<ProcessPayment>, amount: u64) -> Result<()> {
    require!(amount > 0, PaymentError::InvalidAmount);

    let from = &mut ctx.accounts.from;
    let to = &mut ctx.accounts.to_account;

    require!(from.balance >= amount, PaymentError::InsufficientBalance);

    from.balance = from.balance.checked_sub(amount).ok_or(PaymentError::MathOverflow)?;
    to.balance = to.balance.checked_add(amount).ok_or(PaymentError::MathOverflow)?;

    emit!(PaymentProcessed {
        from: from.owner,
        to: to.owner,
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(
        " Transferência de {} de {} para {} concluída",
        amount,
        from.owner,
        to.owner
    );

    Ok(())
}

pub fn pay_fee(ctx: Context<ProcessPayment>, amount: u64) -> Result<()> {
    require!(amount > 0, PaymentError::InvalidAmount);

    let from = &mut ctx.accounts.from;
    require!(from.balance >= amount, PaymentError::InsufficientBalance);

    from.balance -= amount;

    emit!(FeePaid {
        user: ctx.accounts.owner.key(),
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(" Taxa de {} paga pelo usuário {}", amount, ctx.accounts.owner.key());
    Ok(())
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64, fee_percent: u64) -> Result<()> {
    require!(amount > 0, PaymentError::InvalidAmount);

    let from = &mut ctx.accounts.from;
    require!(from.balance >= amount, PaymentError::InsufficientBalance);

    let fee = amount.checked_mul(fee_percent).ok_or(PaymentError::Overflow)?
        .checked_div(100).ok_or(PaymentError::Overflow)?;
    let net_amount = amount.checked_sub(fee).ok_or(PaymentError::Overflow)?;

    from.balance -= amount;

    emit!(Withdrawn {
        user: ctx.accounts.owner.key(),
        net_amount,
        fee_amount: fee,
        fee_percent,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(
        "Usuário {} sacou {} ({} de taxa - {}%)",
        ctx.accounts.owner.key(),
        net_amount,
        fee,
        fee_percent
    );

    Ok(())
}


pub fn validate_token(token: Pubkey) -> Result<()> {
    let accepted_tokens: Vec<Pubkey> = vec![
        Pubkey::from_str("So11111111111111111111111111111111111111112")
            .map_err(|_| error!(PaymentError::InvalidPubkey))?
    ];

    require!(accepted_tokens.contains(&token), PaymentError::TokenNotAllowed);

    emit!(TokenValidated {
        token,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(" Token validado com sucesso: {}", token);
    Ok(())
}
