use anchor_lang::prelude::*;
use crate::state::User;
use crate::errors::UserError;
use crate::{RegisterUser, AddGas, RemoveGas, AddPass, Rebalance};

#[event]
pub struct GasAdded {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct GasRemoved {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct PassAdded {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct UserRegistered {
    pub user: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct UserRebalanced {
    pub user: Pubkey,
    pub gas_balance: u64,
    pub pass_balance: u64,
    pub timestamp: i64,
}

pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    user.owner = ctx.accounts.user.key();
    user.gas_balance = 0;
    user.pass_balance = 0;

    emit!(UserRegistered {
        user: user.owner,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Usuário registrado: {}", user.owner);
    Ok(())
}

pub fn add_gas(ctx: Context<AddGas>, amount: u64) -> Result<()> {
    require!(amount > 0, UserError::InvalidAmount);

    let user = &mut ctx.accounts.user_account;
    user.gas_balance = user
        .gas_balance
        .checked_add(amount)
        .ok_or(UserError::Overflow)?;

    emit!(GasAdded {
        user: user.owner,
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Gas adicionado: {}", amount);
    Ok(())
}

pub fn remove_gas(ctx: Context<RemoveGas>, amount: u64) -> Result<()> {
    require!(amount > 0, UserError::InvalidAmount);

    let user = &mut ctx.accounts.user_account;
    require!(user.gas_balance >= amount, UserError::InsufficientBalance);

    user.gas_balance -= amount;

    emit!(GasRemoved {
        user: user.owner,
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Gas removido: {}", amount);
    Ok(())
}

pub fn add_pass(ctx: Context<AddPass>, amount: u64) -> Result<()> {
    require!(amount > 0, UserError::InvalidAmount);

    let user = &mut ctx.accounts.user_account;
    user.pass_balance = user
        .pass_balance
        .checked_add(amount)
        .ok_or(UserError::Overflow)?;

    emit!(PassAdded {
        user: user.owner,
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(" Passes adicionados: {}", amount);
    Ok(())
}

pub fn rebalance(ctx: Context<Rebalance>) -> Result<()> {
    let user = &ctx.accounts.user_account;

    emit!(UserRebalanced {
        user: user.owner,
        gas_balance: user.gas_balance,
        pass_balance: user.pass_balance,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(
        " Rebalanceando usuário {} — GAS: {}, PASS: {}",
        user.owner,
        user.gas_balance,
        user.pass_balance
    );

    Ok(())
}
