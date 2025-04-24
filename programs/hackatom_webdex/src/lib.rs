use anchor_lang::prelude::*;

declare_id!("GzxWyUMibB3HZeyzCRc7GV9zLKnV5pQT43fZMrivbWb2");

pub mod state;
pub mod interface;
pub mod util;

#[program]
pub mod hackatom_webdex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn create_bot(
        ctx: Context<interface::write::CreateBot>,
        prefix: String,
        name: String,
        manager: Pubkey,
        strategy: Pubkey,
        sub_account: Pubkey,
        payments: Pubkey,
        token_pass: Pubkey,
    ) -> Result<()> {
        interface::write::create_bot(ctx, prefix, name, manager, strategy, sub_account, payments, token_pass)
    }
    
    pub fn get_bot_info(ctx: Context<interface::read::GetBotInfo>) -> Result<state::Bot> {
        interface::read::get_bot_info(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
