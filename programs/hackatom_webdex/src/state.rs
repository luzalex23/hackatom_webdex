use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub balance: u64,
}

#[account]
#[derive(Clone)]
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

    pub const SPACE: usize = 8 +  
                             4 + 32 +
                             4 + 32 + 
                             32 + 
                             32 + 
                             32 + 
                             32 +  
                             32 + 
                             32; 
}
