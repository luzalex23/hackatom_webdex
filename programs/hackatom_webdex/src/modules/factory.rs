use anchor_lang::prelude::*;
use crate::state::Bot;
use crate::errors::FactoryError;
use crate::{ CreateBot, GetBotInfo };
#[event]
pub struct BotCreated {
    pub owner: Pubkey,
    pub name: String,
    pub prefix: String,
    pub timestamp: i64,
}

pub fn create_bot(
    ctx: Context<CreateBot>,
    prefix: String,
    name: String,
    manager: Pubkey,
    strategy: Pubkey,
    sub_account: Pubkey,
    payments: Pubkey,
    token_pass: Pubkey,
) -> Result<()> {
    //  Validações obrigatórias
    require!(name.len() > 0, FactoryError::EmptyName);
    require!(prefix.len() > 0, FactoryError::EmptyPrefix);
    require!(manager != Pubkey::default(), FactoryError::InvalidManager);
    require!(strategy != Pubkey::default(), FactoryError::InvalidStrategy);
    require!(sub_account != Pubkey::default(), FactoryError::InvalidSubAccount);
    require!(token_pass != Pubkey::default(), FactoryError::InvalidTokenPass);

    // Preenchimento da conta
    let bot = &mut ctx.accounts.bot;
    bot.prefix = prefix.clone();
    bot.name = name.clone();
    bot.owner = ctx.accounts.admin.key();
    bot.manager = manager;
    bot.strategy = strategy;
    bot.sub_account = sub_account;
    bot.payments = payments;
    bot.token_pass = token_pass;

    //  Emite evento
    emit!(BotCreated {
        owner: bot.owner,
        name,
        prefix,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(" Bot '{}' cadastrado com prefixo '{}'", bot.name, bot.prefix);
    Ok(())
}

pub fn get_bot_info(ctx: Context<GetBotInfo>) -> Result<Bot> {
    let bot_account = &ctx.accounts.bot;

    msg!("Informações do bot '{}':", bot_account.name);
    msg!("• Prefixo: {}", bot_account.prefix);
    msg!("• Owner: {}", bot_account.owner);
    msg!("• Manager: {}", bot_account.manager);
    msg!("• Strategy: {}", bot_account.strategy);
    msg!("• SubAccount: {}", bot_account.sub_account);
    msg!("• TokenPass: {}", bot_account.token_pass);

    Ok(Bot {
        prefix: bot_account.prefix.clone(),
        name: bot_account.name.clone(),
        owner: bot_account.owner,
        manager: bot_account.manager,
        strategy: bot_account.strategy,
        sub_account: bot_account.sub_account,
        payments: bot_account.payments,
        token_pass: bot_account.token_pass,
    })
}


