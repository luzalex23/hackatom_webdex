use anchor_lang::prelude::*;
use crate::state::{SubAccount, Bot};
use crate::errors::StrategyError;
use crate::ExecuteStrategy;

#[event]
pub struct StrategyExecuted {
    pub sub_owner: Pubkey,
    pub bot: Pubkey,
    pub token: Pubkey,
    pub execution_fee: u64,
    pub payload_size: u64,
    pub timestamp: i64,
}

pub fn execute_strategy(
    ctx: Context<ExecuteStrategy>,
    data: Vec<u8>,
    execution_fee: u64,
) -> Result<()> {
    let sub = &mut ctx.accounts.sub_account;
    let bot = &ctx.accounts.bot;

    // Valida vínculo do bot com a subconta
    require!(sub.bot == bot.key(), StrategyError::BotMismatch);

    // Verifica saldo suficiente
    require!(execution_fee > 0, StrategyError::InvalidFee);
    require!(sub.balance >= execution_fee, StrategyError::InsufficientBalance);

    // Deduz taxa de execução
    sub.balance -= execution_fee;

    //  Emitir evento para rastreabilidade
    emit!(StrategyExecuted {
        sub_owner: sub.owner,
        bot: bot.key(),
        token: sub.token,
        execution_fee,
        payload_size: data.len() as u64,
        timestamp: Clock::get()?.unix_timestamp,
    });

    //  Logs úteis para debug
    msg!(" Estratégia executada para subconta de {}", sub.owner);
    msg!("• Bot vinculado: {}", sub.bot);
    msg!("• Token: {}", sub.token);
    msg!("• Saldo restante: {}", sub.balance);
    msg!("• Payload: {:?} ({} bytes)", data, data.len());

    Ok(())
}
