use anchor_lang::prelude::*;
use mango::instruction;
use solana_program::program::invoke;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"mango-mint".as_ref()],
        bump = 0,
        mint::decimals = 6,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = receiver
    )]
    pub destination: Account<'info, TokenAccount>,
    pub payer: Signer<'info>,
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> ProgramResult {

    msg!("inside our amount: {}", amount);
    // let owner = ctx.accounts.token_accounts.load()?;

    // token::mint_to(ctx.accounts, amount)

    Ok(())
}
