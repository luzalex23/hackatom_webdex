/*read interface*/
use anchor_lang::prelude::*;
pub use crate::modules::factory::GetBotInfo;

pub fn get_bot_info(ctx: Context<GetBotInfo>) -> Result<crate::state::Bot> {
    crate::modules::factory::get_bot_info(ctx)
}

