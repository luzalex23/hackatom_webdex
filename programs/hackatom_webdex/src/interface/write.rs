use anchor_lang::prelude::*;
use crate::state::Bot;

#[derive(Accounts)]
pub struct CreateBot<'info> {
    /// A nova conta que armazenará os dados do Bot.
    #[account(init, payer = admin, space = Bot::SPACE)]
    pub bot: Account<'info, Bot>,
    /// Conta do administrador (quem paga a criação da conta)
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Essa função inicializa o estado do Bot com os parâmetros fornecidos.
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
    let bot = &mut ctx.accounts.bot;
    bot.prefix = prefix;
    bot.name = name;
    bot.owner = ctx.accounts.admin.key();
    bot.manager = manager;
    bot.strategy = strategy;
    bot.sub_account = sub_account;
    bot.payments = payments;
    bot.token_pass = token_pass;
    msg!("Bot cadastrado com sucesso.");
    Ok(())
}
