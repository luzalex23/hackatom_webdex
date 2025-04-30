use anchor_lang::prelude::*;
use crate::state::Bot;

#[derive(Accounts)]
pub struct CreateBot<'info> {
    #[account(init, payer = admin, space = Bot::SPACE)]
    pub bot: Account<'info, Bot>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetBotInfo<'info> {
    pub bot: Account<'info, Bot>,
}
