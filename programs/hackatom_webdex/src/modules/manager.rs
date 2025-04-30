use anchor_lang::prelude::*;
use crate::state::User;
use crate::{RegisterUser, AddGas, RemoveGas, AddPass, Rebalance};

pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    user.owner = ctx.accounts.user.key();
    user.gas_balance = 0;
    user.pass_balance = 0;
    msg!("Usuário registrado.");
    Ok(())
}

pub fn add_gas(ctx: Context<AddGas>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    user.gas_balance = user.gas_balance.checked_add(amount).ok_or(ErrorCode::Overflow)?;
    msg!("Gas adicionado: {}", amount);
    Ok(())
}

pub fn remove_gas(ctx: Context<RemoveGas>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    require!(user.gas_balance >= amount, ErrorCode::InsufficientBalance);
    user.gas_balance -= amount;
    msg!("Gas removido: {}", amount);
    Ok(())
}

pub fn add_pass(ctx: Context<AddPass>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    user.pass_balance = user.pass_balance.checked_add(amount).ok_or(ErrorCode::Overflow)?;
    msg!("Passes adicionados: {}", amount);
    Ok(())
}

pub fn rebalance(ctx: Context<Rebalance>) -> Result<()> {
    let user = &ctx.accounts.user_account;
    msg!(
        "Rebalanceando usuário {} com GAS {} e PASS {}",
        user.owner,
        user.gas_balance,
        user.pass_balance
    );
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Saldo insuficiente.")]
    InsufficientBalance,
    #[msg("Overflow ao adicionar valor.")]
    Overflow,
}
