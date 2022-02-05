use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, MintTo, self}, associated_token::AssociatedToken,
};

#[derive(Accounts)]
#[instruction(mint_bump: u8, amount: u64)]
pub struct Deposit<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"mango-deposit".as_ref()],
        bump = mint_bump,
        mint::decimals = 6,
        mint::authority = mint 
    )]
    pub mint: Account<'info, Mint>,
    pub payer: Signer<'info>,

    // pub mint_authority: AccountInfo<'info>,
    
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = receiver
    )]
    pub destination: Account<'info, TokenAccount>,
    pub receiver: AccountInfo<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Deposit<'info> {
    fn mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                to: self.destination.to_account_info(),
                mint: self.mint.to_account_info(),
                authority: self.mint.to_account_info()
            }
        )
    }
}

pub fn handler(ctx: Context<Deposit>, mint_bump: u8, amount: u64) -> ProgramResult {
    token::mint_to(
        ctx.accounts.mint_context()
        .with_signer(&[&[&"mango-deposit".as_bytes(), &[mint_bump]]]),
        amount,
    )?;

    Ok(())
}
