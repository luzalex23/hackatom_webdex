use anchor_lang::prelude::*;
use anchor_lang::prelude::msg;
use crate::state::Bot;

#[derive(Accounts)]
pub struct BotCreationAccounts<'info> {
    #[account(init, payer = admin, space = Bot::SPACE)]
    pub bot: Account<'info, Bot>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_bot(
    ctx: Context<BotCreationAccounts>,
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

#[derive(Accounts)]
pub struct BotInfoAccounts<'info> {
    pub bot: Account<'info, Bot>,
}

pub fn get_bot_info(ctx: Context<BotInfoAccounts>) -> Result<Bot> {
    let bot_account = &ctx.accounts.bot;
    let ret_bot = Bot {
        prefix: bot_account.prefix.clone(),
        name: bot_account.name.clone(),
        owner: bot_account.owner,
        manager: bot_account.manager,
        strategy: bot_account.strategy,
        sub_account: bot_account.sub_account,
        payments: bot_account.payments.clone(),
        token_pass: bot_account.token_pass,
    };
    Ok(ret_bot)
}
