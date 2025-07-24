use anchor_lang::prelude::*;

#[error_code]
pub enum FactoryError {
    #[msg("O nome do bot não pode estar vazio.")]
    EmptyName,
    #[msg("O prefixo do bot não pode estar vazio.")]
    EmptyPrefix,
    #[msg("Manager inválido.")]
    InvalidManager,
    #[msg("Strategy inválida.")]
    InvalidStrategy,
    #[msg("Subconta inválida.")]
    InvalidSubAccount,
    #[msg("Token de passe inválido.")]
    InvalidTokenPass,
}


#[error_code]
pub enum UserError {
    #[msg("Saldo insuficiente.")]
    InsufficientBalance,
    #[msg("Overflow ao adicionar valor.")]
    Overflow,
    #[msg("O valor precisa ser maior que zero.")]
    InvalidAmount,
}
#[error_code]
pub enum PaymentError {
    #[msg("Saldo insuficiente.")]
    InsufficientBalance,
    #[msg("Token não permitido.")]
    TokenNotAllowed,
    #[msg("Pubkey inválido.")]
    InvalidPubkey,
    #[msg("Overflow ao calcular valores.")]
    Overflow,
    #[msg("Overflow aritmético.")]
    MathOverflow,
    #[msg("Conta de destino não foi fornecida.")]
    DestinationAccountNotProvided,
    #[msg("O valor precisa ser maior que zero.")]
    InvalidAmount,
}
#[error_code]
pub enum StrategyError {
    #[msg("O bot associado não corresponde ao da subconta.")]
    BotMismatch,
    #[msg("Saldo insuficiente para execução.")]
    InsufficientBalance,
    #[msg("Taxa de execução inválida.")]
    InvalidFee,
}
#[error_code]
pub enum SubAccountError {
    #[msg("Saldo insuficiente.")]
    InsufficientBalance,
    #[msg("Overflow ao adicionar valor.")]
    Overflow,
    #[msg("Valor inválido.")]
    InvalidAmount,
    #[msg("Bot inválido.")]
    InvalidBot,
    #[msg("Token inválido.")]
    InvalidToken,
}
