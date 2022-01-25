use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod dasheri {
use super::*;
    pub fn create_mango_account(
        ctx: Context<CreateMangoAccount>,
        account_num: u64,
    ) -> ProgramResult {
        instructions::create_mango_account::handler(ctx, account_num)
    }

    pub fn init_mint(ctx: Context<Reserve>) -> ProgramResult {
        instructions::init_mint::handler(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, mint_bump: u8, amount: u64) -> ProgramResult {
        msg!("our amount {}", amount);
        instructions::deposit::handler(ctx, mint_bump, amount)
    }
}
