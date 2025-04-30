use anchor_lang::prelude::*;

declare_id!("3p3W4Vq9Qsb9duYbz1JBhJZUtXBFNTA3J1nn6e4veFuC");

// Módulos internos
pub mod state;
pub mod modules; // Aqui será o diretório modules (com mod.rs e os submódulos, por exemplo, factory.rs)
  
// Declaração dos contextos diretamente aqui
#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateBot<'info> {
    #[account(init, payer = admin, space = state::Bot::SPACE)]
    pub bot: Account<'info, state::Bot>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetBotInfo<'info> {
    pub bot: Account<'info, state::Bot>,
}

#[program]
pub mod hackatom_webdex {
    use super::*;
    // Aqui o programa chama (dispara) a lógica definida no módulo factory.
    use crate::modules::factory::{ create_bot as factory_create_bot, get_bot_info as factory_get_bot_info };

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Programa inicializado.");
        Ok(())
    }

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
        // Apenas repassa os dados para o módulo de lógica
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

    pub fn get_bot_info(ctx: Context<GetBotInfo>) -> Result<state::Bot> {
        factory_get_bot_info(ctx)
    }
}
