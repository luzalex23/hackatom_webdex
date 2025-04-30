use anchor_lang::prelude::*;
use crate::state::SubAccount;
use crate::{
    RegisterSubAccount, DepositSubAccount, WithdrawSubAccount, GetSubAccountInfo,
};

pub fn register_subaccount(
    ctx: Context<RegisterSubAccount>,
    bot: Pubkey,
    token: Pubkey,
) -> Result<()> {
    let sub_account = &mut ctx.accounts.sub_account;
    sub_account.owner = ctx.accounts.owner.key();
    sub_account.bot = bot;
    sub_account.token = token;
    sub_account.balance = 0;
    msg!("Subconta registrada.");
    Ok(())
}

pub fn deposit_to_subaccount(ctx: Context<DepositSubAccount>, amount: u64) -> Result<()> {
    let sub_account = &mut ctx.accounts.sub_account;
    sub_account.balance = sub_account
        .balance
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;
    msg!("Deposito realizado: {}", amount);
    Ok(())
}

pub fn withdraw_from_subaccount(ctx: Context<WithdrawSubAccount>, amount: u64) -> Result<()> {
    let sub_account = &mut ctx.accounts.sub_account;
    require!(sub_account.balance >= amount, ErrorCode::InsufficientBalance);
    sub_account.balance -= amount;
    msg!("Saque realizado: {}", amount);
    Ok(())
}

pub fn get_subaccount_info(ctx: Context<GetSubAccountInfo>) -> Result<SubAccount> {
    let s = &ctx.accounts.sub_account;
    Ok(SubAccount {
        owner: s.owner,
        bot: s.bot,
        token: s.token,
        balance: s.balance,
    })
}

#[error_code]
pub enum ErrorCode {
    #[msg("Saldo insuficiente.")]
    InsufficientBalance,
    #[msg("Overflow ao adicionar valor.")]
    Overflow,
}
