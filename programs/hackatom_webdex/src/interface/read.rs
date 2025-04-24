use anchor_lang::prelude::*;
use crate::state::Bot;

#[derive(Accounts)]
pub struct GetBotInfo<'info> {
    pub bot: Account<'info, Bot>,
}


pub fn get_bot_info(ctx: Context<GetBotInfo>) -> Result<Bot> {
    let bot = &ctx.accounts.bot;
    Ok(bot.clone())
}
