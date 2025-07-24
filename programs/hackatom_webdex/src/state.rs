use anchor_lang::prelude::*;

///  Representa um bot cadastrado no sistema.
#[account]
#[derive(Debug, PartialEq)]
pub struct Bot {
    /// Prefixo do bot, usado como identificador curto.
    pub prefix: String,
    /// Nome completo do bot.
    pub name: String,
    /// Carteira que criou o bot.
    pub owner: Pubkey,
    /// Carteira responsável pela execução ou gestão.
    pub manager: Pubkey,
    /// Endereço do contrato de estratégia.
    pub strategy: Pubkey,
    /// Subconta associada ao bot.
    pub sub_account: Pubkey,
    /// Conta usada para recebimentos/pagamentos.
    pub payments: Pubkey,
    /// Token usado para acesso ou passe.
    pub token_pass: Pubkey,
}

impl Bot {
    pub const SPACE: usize = 8  // Discriminator
        + 4 + 32  // prefix (string)
        + 4 + 32  // name (string)
        + 32      // owner
        + 32      // manager
        + 32      // strategy
        + 32      // sub_account
        + 32      // payments
        + 32      // token_pass
        + 8;      // extra padding

    /// Construtor auxiliar
    pub fn new(
        prefix: String,
        name: String,
        owner: Pubkey,
        manager: Pubkey,
        strategy: Pubkey,
        sub_account: Pubkey,
        payments: Pubkey,
        token_pass: Pubkey,
    ) -> Self {
        Self {
            prefix,
            name,
            owner,
            manager,
            strategy,
            sub_account,
            payments,
            token_pass,
        }
    }
}

///  Representa um usuário registrado no sistema, com saldos de GAS e PASSES.
#[account]
#[derive(Debug, PartialEq)]
pub struct User {
    /// Carteira do usuário.
    pub owner: Pubkey,
    /// Saldo de "GAS", usado para transações.
    pub gas_balance: u64,
    /// Saldo de "PASS", usado como créditos ou permissão.
    pub pass_balance: u64,
}

impl User {
    pub const SPACE: usize = 8     // Discriminator
        + 32                       // owner
        + 8                        // gas_balance
        + 8;                       // pass_balance

    pub fn has_gas(&self, amount: u64) -> bool {
        self.gas_balance >= amount
    }

    pub fn has_pass(&self, amount: u64) -> bool {
        self.pass_balance >= amount
    }
}

///  Representa uma subconta vinculada a um bot e token.
#[account]
#[derive(Debug, PartialEq)]
pub struct SubAccount {
    /// Dono da subconta.
    pub owner: Pubkey,
    /// Bot ao qual está vinculada.
    pub bot: Pubkey,
    /// Token associado à subconta.
    pub token: Pubkey,
    /// Saldo disponível.
    pub balance: u64,
}

impl SubAccount {
    pub const SPACE: usize = 8     // Discriminator
        + 32                       // owner
        + 32                       // bot
        + 32                       // token
        + 8;                       // balance

    pub fn has_balance(&self, amount: u64) -> bool {
        self.balance >= amount
    }
}
