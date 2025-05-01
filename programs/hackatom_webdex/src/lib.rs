use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

declare_id!("Cj2cdLtPtmQvCo2YHvvxbYgN3Dn6v62FR8tDxoLfBhFF");

// Módulos internos
pub mod state;
pub mod modules;

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

#[derive(Accounts)]
pub struct ProcessPayment<'info> {
    #[account(mut, has_one = owner)]
    pub from: Account<'info, state::SubAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ValidateToken {}

#[derive(Accounts)]
pub struct ExecuteStrategy<'info> {
    #[account(mut, has_one = owner)]
    pub sub_account: Account<'info, state::SubAccount>,
    #[account(mut)]
    pub bot: Account<'info, state::Bot>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[program]
pub mod hackatom_webdex {
    use super::*;
    use crate::modules::factory::{ create_bot as factory_create_bot, get_bot_info as factory_get_bot_info };
    use crate::modules::manager::{
        register_user as register_user_handler,
        add_gas as add_gas_handler,
        remove_gas as remove_gas_handler,
        add_pass as add_pass_handler,
        rebalance as rebalance_handler,
    };
    use crate::modules::sub_accounts::{
        register_subaccount as register_subaccount_handler,
        deposit_to_subaccount as deposit_to_subaccount_handler,
        withdraw_from_subaccount as withdraw_from_subaccount_handler,
        get_subaccount_info as get_subaccount_info_handler,
    };
    use crate::modules::payments::{
        process_payment as process_payment_handler,
        validate_token as validate_token_handler,
        pay_fee as pay_fee_handler,
        withdraw as withdraw_handler,
    };
    use crate::modules::strategy::execute_strategy as execute_strategy_handler;
    use crate::modules::faucet::mint_token as mint_token_handler;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
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
        factory_create_bot(ctx, prefix, name, manager, strategy, sub_account, payments, token_pass)
    }

    pub fn get_bot_info(ctx: Context<GetBotInfo>) -> Result<state::Bot> {
        factory_get_bot_info(ctx)
    }

    pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
        register_user_handler(ctx)
    }

    pub fn add_gas(ctx: Context<AddGas>, amount: u64) -> Result<()> {
        add_gas_handler(ctx, amount)
    }

    pub fn remove_gas(ctx: Context<RemoveGas>, amount: u64) -> Result<()> {
        remove_gas_handler(ctx, amount)
    }

    pub fn add_pass(ctx: Context<AddPass>, amount: u64) -> Result<()> {
        add_pass_handler(ctx, amount)
    }

    pub fn rebalance(ctx: Context<Rebalance>) -> Result<()> {
        rebalance_handler(ctx)
    }

    pub fn register_subaccount(ctx: Context<RegisterSubAccount>, bot: Pubkey, token: Pubkey) -> Result<()> {
        register_subaccount_handler(ctx, bot, token)
    }

    pub fn deposit_to_subaccount(ctx: Context<DepositSubAccount>, amount: u64) -> Result<()> {
        deposit_to_subaccount_handler(ctx, amount)
    }

    pub fn withdraw_from_subaccount(ctx: Context<WithdrawSubAccount>, amount: u64) -> Result<()> {
        withdraw_from_subaccount_handler(ctx, amount)
    }

    pub fn get_subaccount_info(ctx: Context<GetSubAccountInfo>) -> Result<state::SubAccount> {
        get_subaccount_info_handler(ctx)
    }

    pub fn process_payment(ctx: Context<ProcessPayment>, amount: u64, to: Pubkey) -> Result<()> {
        process_payment_handler(ctx, amount, to)
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

    pub fn execute_strategy(ctx: Context<ExecuteStrategy>, data: Vec<u8>, execution_fee: u64) -> Result<()> {
        execute_strategy_handler(ctx, data, execution_fee)
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        mint_token_handler(ctx, amount)
    }
}
