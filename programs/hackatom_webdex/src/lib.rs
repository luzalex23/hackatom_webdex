use anchor_lang::prelude::*;

declare_id!("GzxWyUMibB3HZeyzCRc7GV9zLKnV5pQT43fZMrivbWb2");

pub mod state;
pub mod interfaces;
pub mod util {
    pub mod helpers;
    pub mod math;
}

#[program]
pub mod hackatom_webdex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Programa inicializado: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_bot(
        ctx: Context<interfaces::write::CreateBot>,
        prefix: String,
        name: String,
        manager: Pubkey,
        strategy: Pubkey,
        sub_account: Pubkey,
        payments: Pubkey,
        token_pass: Pubkey,
    ) -> Result<()> {
        interfaces::write::create_bot(
            ctx,
            prefix,
            name,
            manager,
            strategy,
            sub_account,
            payments,
            token_pass,
        )
    }

    pub fn get_bot_info(ctx: Context<interfaces::read::GetBotInfo>) -> Result<state::Bot> {
        interfaces::read::get_bot_info(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
