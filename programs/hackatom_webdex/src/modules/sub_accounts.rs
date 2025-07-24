use anchor_lang::prelude::*;
use crate::state::SubAccount;
use crate::errors::SubAccountError;
use crate::{
    RegisterSubAccount, DepositSubAccount, WithdrawSubAccount, GetSubAccountInfo,
};

#[event]
pub struct SubAccountRegistered {
    pub owner: Pubkey,
    pub bot: Pubkey,
    pub token: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct SubAccountDeposited {
    pub owner: Pubkey,
    pub amount: u64,
    pub new_balance: u64,
    pub timestamp: i64,
}

#[event]
pub struct SubAccountWithdrawn {
    pub owner: Pubkey,
    pub amount: u64,
    pub new_balance: u64,
    pub timestamp: i64,
}

#[event]
pub struct SubAccountInfo {
    pub owner: Pubkey,
    pub bot: Pubkey,
    pub token: Pubkey,
    pub balance: u64,
    pub timestamp: i64,
}

pub fn register_subaccount(
    ctx: Context<RegisterSubAccount>,
    bot: Pubkey,
    token: Pubkey,
) -> Result<()> {
    require!(bot != Pubkey::default(), SubAccountError::InvalidBot);
    require!(token != Pubkey::default(), SubAccountError::InvalidToken);

    let sub_account = &mut ctx.accounts.sub_account;
    sub_account.owner = ctx.accounts.owner.key();
    sub_account.bot = bot;
    sub_account.token = token;
    sub_account.balance = 0;

    emit!(SubAccountRegistered {
        owner: sub_account.owner,
        bot,
        token,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Subconta registrada para usuário: {}", sub_account.owner);
    Ok(())
}

pub fn deposit_to_subaccount(ctx: Context<DepositSubAccount>, amount: u64) -> Result<()> {
    require!(amount > 0, SubAccountError::InvalidAmount);

    let sub_account = &mut ctx.accounts.sub_account;
    sub_account.balance = sub_account
        .balance
        .checked_add(amount)
        .ok_or(SubAccountError::Overflow)?;

    emit!(SubAccountDeposited {
        owner: sub_account.owner,
        amount,
        new_balance: sub_account.balance,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Depósito de {} tokens realizado. Novo saldo: {}", amount, sub_account.balance);
    Ok(())
}

pub fn withdraw_from_subaccount(ctx: Context<WithdrawSubAccount>, amount: u64) -> Result<()> {
    require!(amount > 0, SubAccountError::InvalidAmount);

    let sub_account = &mut ctx.accounts.sub_account;
    require!(sub_account.balance >= amount, SubAccountError::InsufficientBalance);

    sub_account.balance -= amount;

    emit!(SubAccountWithdrawn {
        owner: sub_account.owner,
        amount,
        new_balance: sub_account.balance,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Saque de {} tokens realizado. Novo saldo: {}", amount, sub_account.balance);
    Ok(())
}

pub fn get_subaccount_info(ctx: Context<GetSubAccountInfo>) -> Result<SubAccount> {
    let s = &ctx.accounts.sub_account;

    emit!(SubAccountInfo {
        owner: s.owner,
        bot: s.bot,
        token: s.token,
        balance: s.balance,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Info da Subconta: Owner: {}, Bot: {}, Token: {}, Saldo: {}",
        s.owner, s.bot, s.token, s.balance
    );

    Ok(SubAccount {
        owner: s.owner,
        bot: s.bot,
        token: s.token,
        balance: s.balance,
    })
}
