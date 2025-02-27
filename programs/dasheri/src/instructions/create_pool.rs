use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::ids::usdc_token;
use crate::state::Pool;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreatePool<'info> {
    #[account(
        init,
        seeds = [b"pool".as_ref(), payer.key().as_ref()],
        bump = bump,
        payer = payer,
        space = 8 + std::mem::size_of::<Pool>(),
    )]
    pub pool: Box<Account<'info, Pool>>,

    #[account(
        init,
        associated_token::authority = pool,
        associated_token::mint = deposit_mint,
        payer = payer
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    // todo: enable
    // #[account(
    //     constraint = deposit_mint.key() == usdc_token::ID
    // )]
    pub deposit_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreatePool>, bump: u8) -> ProgramResult {
    let pool = &mut ctx.accounts.pool;
    pool.bump = bump;
    pool.deposit_mint = ctx.accounts.deposit_mint.key();
    pool.vault = ctx.accounts.vault.key();
    pool.admin = ctx.accounts.payer.key();

    Ok(())
}
