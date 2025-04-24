use anchor_lang::prelude::*;

///prefixo para facilitar o rastreamento.
pub fn log_helper(message: &str) {
    msg!("[HELPER]: {}", message);
}
