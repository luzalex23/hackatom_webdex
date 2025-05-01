use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

declare_id!("H4c87pshhbmUwiNzNcuSs1yjqEzetWNsmCo2HfsyqY89");

// Módulos internos
pub mod state;
pub mod modules;
pub use crate::MintToken as FaucetMintToken;

// Declaração dos contextos
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

/*begin sub_accounts module*/
#[derive(Accounts)]
#[instruction(bot: Pubkey, token: Pubkey)]
pub struct RegisterSubAccount<'info> {
    #[account(
        init,
        payer = owner,
        seeds = [b"subaccount", owner.key().as_ref(), bot.as_ref(), token.as_ref()],
        bump,
        space = state::SubAccount::SPACE
    )]
    pub sub_account: Account<'info, state::SubAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSubAccount<'info> {
    #[account(mut, has_one = owner)]
    pub sub_account: Account<'info, state::SubAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawSubAccount<'info> {
    #[account(mut, has_one = owner)]
    pub sub_account: Account<'info, state::SubAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetSubAccountInfo<'info> {
    pub sub_account: Account<'info, state::SubAccount>,
}
/*end sub_accounts module*/

/*begin payaments modules*/
#[derive(Accounts)]
pub struct ProcessPayment<'info> {
    #[account(mut, has_one = owner)]
    pub from: Account<'info, state::SubAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ValidateToken {}
/*end payaments modules*/

/*begin strategy modules*/
#[derive(Accounts)]
pub struct ExecuteStrategy<'info> {
    #[account(mut, has_one = owner)]
    pub sub_account: Account<'info, state::SubAccount>,
    #[account(mut)]
    pub bot: Account<'info, state::Bot>,
    pub owner: Signer<'info>,
}
/*end strategy modules*/

/*begin faucet module*/
#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

/*end faucet module*/

#[program]
pub mod hackatom_webdex {
    use super::*;
    use crate::modules::factory::{ create_bot as factory_create_bot, get_bot_info as factory_get_bot_info };
    use crate::modules::manager::*;
    use crate::modules::sub_accounts::*;
    use crate::modules::payments::{
        validate_token as validate_token_handler,
        pay_fee as pay_fee_handler,
        withdraw as withdraw_handler,
    };
    use crate::modules::strategy::execute_strategy as execute_strategy_handler;
    use crate::modules::faucet::mint_token as mint_token_handler;

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
    /*fn manager*/
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

    /*fn sub_accounts*/
    pub fn register_subaccount(
        ctx: Context<RegisterSubAccount>,
        bot: Pubkey,
        token: Pubkey,
    ) -> Result<()> {
        register_subaccount(ctx, bot, token)
    }
    
    pub fn deposit_to_subaccount(
        ctx: Context<DepositSubAccount>,
        amount: u64,
    ) -> Result<()> {
        deposit_to_subaccount(ctx, amount)
    }
    
    pub fn withdraw_from_subaccount(
        ctx: Context<WithdrawSubAccount>,
        amount: u64,
    ) -> Result<()> {
        withdraw_from_subaccount(ctx, amount)
    }
    
    pub fn get_subaccount_info(ctx: Context<GetSubAccountInfo>) -> Result<state::SubAccount> {
        get_subaccount_info(ctx)
    }
    /*fn payaments*/
    pub fn process_payment(
        ctx: Context<ProcessPayment>,
        amount: u64,
        to: Pubkey,
    ) -> Result<()> {
        process_payment(ctx, amount, to)
    }
    
    pub fn validate_token(_ctx: Context<ValidateToken>, token: Pubkey) -> Result<()> {
        validate_token_handler(token)
    }
    pub fn pay_fee(ctx: Context<ProcessPayment>, amount: u64) -> Result<()> {
        pay_fee_handler(ctx, amount)
    }
    
    pub fn withdraw(ctx: Context<ProcessPayment>, amount: u64, fee_percent: u64) -> Result<()> {
        withdraw_handler(ctx, amount, fee_percent)
    }
     /*fn strategy*/   
     pub fn execute_strategy(
        ctx: Context<ExecuteStrategy>,
        data: Vec<u8>,
        execution_fee: u64,
    ) -> Result<()> {
        execute_strategy_handler(ctx, data, execution_fee)
    }
    /*fn faucet*/
    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        mint_token_handler(ctx, amount)
    }    
}
