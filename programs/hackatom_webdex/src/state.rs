use anchor_lang::prelude::*;
/// Representa um bot cadastrado no sistema.
#[account]
pub struct Bot {
    pub prefix: String,
    pub name: String,
    pub owner: Pubkey,
    pub manager: Pubkey,
    pub strategy: Pubkey,
    pub sub_account: Pubkey,
    pub payments: Pubkey,
    pub token_pass: Pubkey,
}

impl Bot {
    // O cálculo considera:
    // 8 bytes para o discriminator + 
    // 4 bytes para o tamanho e 32 bytes para o campo prefix +
    // 4 bytes para o tamanho e 32 bytes para o campo name +
    // 32 bytes para cada Pubkey e um extra de 8 bytes de padding.
    pub const SPACE: usize = 8  // Discriminator
                            + 4 + 32  // prefix
                            + 4 + 32  // name
                            + 32      // owner
                            + 32      // manager
                            + 32      // strategy
                            + 32      // sub_account
                            + 32      // payments
                            + 32      // token_pass
                            + 8;      // padding extra
}
/// Representa um usuário registrado no sistema, com saldos de gas e passes.
#[account]
pub struct User {
    pub owner: Pubkey,
    pub gas_balance: u64,
    pub pass_balance: u64,
}

impl User {
    pub const SPACE: usize = 8     // Discriminator
        + 32                       // owner
        + 8                        // gas_balance
        + 8;                       // pass_balance
}