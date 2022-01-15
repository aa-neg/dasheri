use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeAccount, InitializeMint, Mint, TokenAccount};

#[derive(Accounts)]
pub struct Reserve<'info> {
    /// The program for interacting with the token.
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    // The authority of the mint, able to mint more token
    pub authority: AccountInfo<'info>,
    // The account of the vault ? maybe not needed
    // #[account(mut)]
    pub vault: AccountInfo<'info>,
}
    /// The mint for the token being stored in this reserve.
    // pub token_mint: Account<'info, Mint>,

    // pub rent: Sysvar<'info, Rent>,

pub fn handler(ctx: Context<Reserve>) -> ProgramResult {
    msg!("starting up our handler");

    let ix = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &ctx.accounts.authority.key(),
        &ctx.accounts.authority.key(),
        Some(&ctx.accounts.authority.key()),
        10
    )?;

    // msg!("finished setting up our instruction {} {}", ctx.accounts.vault, ct);

    let result = solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.authority.clone(),
            ctx.accounts.authority.clone(),
            ctx.accounts.token_program.clone(),
        ],
        &[],
    );

    let done = result.unwrap_or_else(|err| {
        msg!("failure! {}", err);
    });

    Ok(())
}