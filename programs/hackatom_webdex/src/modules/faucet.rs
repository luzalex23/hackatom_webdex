use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo, Mint, Token, TokenAccount};

use crate::MintToken;

pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    );

    mint_to(cpi_ctx, amount)?;
    msg!("Mintado {} tokens para {}", amount, ctx.accounts.recipient.key());
    Ok(())
}
