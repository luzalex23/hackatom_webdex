use anchor_lang::prelude::*;

declare_id!("B4jpBzqnqaUrESQvyLuGzJqvDDxoBsSHYKtSe1ihPbwV");

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
/*manager module functions*/

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(init, payer = user, space = state::User::SPACE)]
    pub user_account: Account<'info, state::User>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGas<'info> {
    #[account(mut, has_one = owner)]
    pub user_account: Account<'info, state::User>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveGas<'info> {
    #[account(mut, has_one = owner)]
    pub user_account: Account<'info, state::User>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddPass<'info> {
    #[account(mut, has_one = owner)]
    pub user_account: Account<'info, state::User>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Rebalance<'info> {
    #[account(mut, has_one = owner)]
    pub user_account: Account<'info, state::User>,
    pub owner: Signer<'info>,
}
/*end functions manager module*/
#[program]
pub mod hackatom_webdex {
    use super::*;
    // Aqui o programa chama (dispara) a lógica definida no módulo factory.
    use crate::modules::factory::{ create_bot as factory_create_bot, get_bot_info as factory_get_bot_info };
    use crate::modules::manager::*;

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
    pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
        register_user(ctx)
    }
    
    pub fn add_gas(ctx: Context<AddGas>, amount: u64) -> Result<()> {
        add_gas(ctx, amount)
    }
    
    pub fn remove_gas(ctx: Context<RemoveGas>, amount: u64) -> Result<()> {
        remove_gas(ctx, amount)
    }
    
    pub fn add_pass(ctx: Context<AddPass>, amount: u64) -> Result<()> {
        add_pass(ctx, amount)
    }
    
    pub fn rebalance(ctx: Context<Rebalance>) -> Result<()> {
        rebalance(ctx)
    }
}
