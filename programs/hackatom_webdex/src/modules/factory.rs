use anchor_lang::prelude::*;
use crate::state::Bot;
use crate::{ CreateBot, GetBotInfo };

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
    let bot = &mut ctx.accounts.bot;
    bot.prefix = prefix;
    bot.name = name;
    bot.owner = ctx.accounts.admin.key();
    bot.manager = manager;
    bot.strategy = strategy;
    bot.sub_account = sub_account;
    bot.payments = payments;
    bot.token_pass = token_pass;
    msg!("Bot cadastrado com sucesso.");
    Ok(())
}

pub fn get_bot_info(ctx: Context<GetBotInfo>) -> Result<Bot> {
    let bot_account = &ctx.accounts.bot;
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
