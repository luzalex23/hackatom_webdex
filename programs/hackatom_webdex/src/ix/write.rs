use anchor_lang::prelude::*;
// Reexporta o account correto renomeado para escrita
pub use crate::modules::factory::BotCreationAccounts;

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
    crate::modules::factory::create_bot(ctx, prefix, name, manager, strategy, sub_account, payments, token_pass)
}
