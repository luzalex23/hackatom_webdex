use anchor_lang::prelude::*;
use crate::state::{SubAccount, Bot};
use crate::ExecuteStrategy;

pub fn execute_strategy(
    ctx: Context<ExecuteStrategy>,
    data: Vec<u8>,
    execution_fee: u64,
) -> Result<()> {
    let sub = &mut ctx.accounts.sub_account;
    let bot = &ctx.accounts.bot;
    let caller = &ctx.accounts.owner;

    //Verifica se o bot da subconta corresponde ao bot enviado
    require!(
        sub.bot == bot.key(),
        StrategyError::BotMismatch
    );

    //Verifica se a subconta tem saldo suficiente
    require!(
        sub.balance >= execution_fee,
        StrategyError::InsufficientBalance
    );

    // Deduz o custo da execução
    sub.balance -= execution_fee;

    //Lógica de execução simulada
    msg!("Executando estratégia para subconta de {}", sub.owner);
    msg!("Bot: {}", sub.bot);
    msg!("Token: {}", sub.token);
    msg!("Saldo restante: {}", sub.balance);
    msg!("Payload de entrada: {:?} ({} bytes)", data, data.len());
    msg!("Estratégia executada com sucesso.");

    Ok(())
}

#[error_code]
pub enum StrategyError {
    #[msg("O bot associado não corresponde ao da subconta.")]
    BotMismatch,
    #[msg("Saldo insuficiente para execução.")]
    InsufficientBalance,
}
