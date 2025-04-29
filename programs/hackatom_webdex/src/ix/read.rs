use anchor_lang::prelude::*;
// Reexporta o account correto renomeado para leitura
pub use crate::modules::factory::BotInfoAccounts;

pub fn get_bot_info(ctx: Context<BotInfoAccounts>) -> Result<crate::state::Bot> {
    crate::modules::factory::get_bot_info(ctx)
}
