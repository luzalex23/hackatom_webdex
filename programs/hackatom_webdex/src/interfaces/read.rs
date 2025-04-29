use anchor_lang::prelude::*;
use crate::state::Bot;

#[derive(Accounts)]
pub struct GetBotInfo<'info> {
    pub bot: Account<'info, Bot>,
}

/// Esta função retorna uma cópia dos dados do Bot.
/// Como o Account wrapper não é exatamente o mesmo que a struct Bot, 
/// criamos uma instância nova com os campos clonados.
pub fn get_bot_info(ctx: Context<GetBotInfo>) -> Result<Bot> {
    let bot_account = &ctx.accounts.bot;
    let ret_bot = Bot {
        prefix: bot_account.prefix.clone(),
        name: bot_account.name.clone(),
        owner: bot_account.owner,
        manager: bot_account.manager,
        strategy: bot_account.strategy,
        sub_account: bot_account.sub_account,
        payments: bot_account.payments,
        token_pass: bot_account.token_pass,
    };
    Ok(ret_bot)
}
