use anchor_lang::prelude::*;
use anchor_lang::prelude::{msg, Context, Pubkey};

declare_id!("3p3W4Vq9Qsb9duYbz1JBhJZUtXBFNTA3J1nn6e4veFuC");

// Módulos internos
pub mod state;
pub mod ix {
    pub mod read;
    pub mod write;
}
pub mod modules {
    pub mod factory;
}
pub mod util {
    pub mod helpers;
    pub mod math;
}

// Conta utilizada na inicialização
#[derive(Accounts)]
pub struct Initialize {}

#[program]
pub mod hackatom_webdex {
    use super::*;
    // Importa os tipos renomeados e as funções do módulo factory com alias
    use crate::modules::factory::{
        BotCreationAccounts, 
        BotInfoAccounts, 
        create_bot as factory_create_bot, 
        get_bot_info as factory_get_bot_info,
    };

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Programa inicializado.");
        Ok(())
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
        factory_create_bot(
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

    pub fn get_bot_info(ctx: Context<BotInfoAccounts>) -> Result<state::Bot> {
        factory_get_bot_info(ctx)
    }
}
