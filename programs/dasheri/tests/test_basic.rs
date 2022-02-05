use std::mem::size_of;

use dasheri::instructions::Deposit;
use solana_program::{instruction::Instruction, program_pack::Pack};
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::signer::Signer;

use spl_associated_token_account::*;
use program_test::*;

mod program_test;

#[tokio::test]
async fn test_create_mango_account() {

}

#[allow(unaligned_references)]
#[tokio::test]
async fn test_basic() {
    // Setup
    let mut context = TestContext::new().await;
    let mango_group_cookie = MangoGroupCookie::default(&mut context).await;

    // Create mango account
    const ACCOUNT_NUM: u64 = 0_u64;
    let (mango_account, _) = Pubkey::find_program_address(
        &[
            &mango_group_cookie.address.as_ref(),
            &context.solana.context.borrow_mut().payer.pubkey().as_ref(),
            &ACCOUNT_NUM.to_le_bytes(),
        ],
        &context.mango.program_id,
    );
    let instructions = vec![Instruction {
        program_id: context.dasheri.program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::CreateMangoAccount {
                mango_program_ai: context.mango.program_id,
                mango_group: mango_group_cookie.address,
                mango_account: mango_account,
                owner: context.solana.context.borrow_mut().payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&dasheri::instruction::CreateMangoAccount {
            account_num: ACCOUNT_NUM,
        }),
    }];
    context
        .solana
        .process_transaction(&instructions, Some(&[]))
        .await
        .unwrap();
    // TODO revisit size of this
    let reciever_account = context.create_account(size_of::<Deposit>(), &context.dasheri.program_id.clone()).await;
    // let created_mint = context.create_mint(&context.mango.program_id.clone()).await;
    // println!("our created mint: {}", created_mint);


    // Set authority to deposit program_id
    let (pda, _nonce) = Pubkey::find_program_address(&[b"mango"], &context.dasheri.program_id.clone());
    
    let mint_account = context.create_mint_account(&pda).await;

    // let mint = context.mints
    let token_account = context.create_token_account(&context.mango.program_id.clone(), &mint_account).await;


    println!("our mango program id: {}", context.mango.program_id);
    println!("our dasheri program id: {}", context.dasheri.program_id);

    let (mint_pda, bump) = Pubkey::find_program_address(&[b"mango-deposit"], &context.dasheri.program_id);


    let associated_token_account = spl_associated_token_account::get_associated_token_address(
        &reciever_account, 
       &mint_pda,
    );

    println!("our bump {}", bump);

    let deposit_instruction = vec![Instruction {
        program_id: context.dasheri.program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::Deposit {
                mint: mint_pda,
                destination: associated_token_account,
                receiver: reciever_account,
                associated_token_program: spl_associated_token_account::id(),
                payer: context.solana.context.borrow_mut().payer.pubkey(),
                token_program: spl_token::id(),
                system_program: solana_sdk::system_program::id(),
                rent: solana_sdk::sysvar::rent::id(),
            },
            Some(true),
        ),
        data: anchor_lang::InstructionData::data(&dasheri::instruction::Deposit {
            mint_bump: bump,
            // TODO adjust amount
            amount: 69,
        }),
    }];

    context
        .solana
        .process_transaction(&deposit_instruction, Some(&[]))
        .await
        .unwrap();

    let bar = context.solana.context.borrow_mut().banks_client.get_account(associated_token_account).await.unwrap().unwrap();
    let foo = spl_token::state::Account::unpack(&bar.data).unwrap();

    println!("hello {}", foo.amount);
}
