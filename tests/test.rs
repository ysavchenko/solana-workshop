
use solana_sdk::transaction::Transaction;
use solana_program_test::*;
use work_shop::*;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Signer};

pub fn program_test()-> ProgramTest {
    ProgramTest::new("work_shop", id(), processor!(processor::Processor::process_instruction))
}

#[tokio::test]
async fn basic_test() {

    let mut program_context = program_test().start_with_context().await;

    let mut transaction = Transaction::new_with_payer(
        &[
            instruction::initialize(&id()).unwrap(),
        ],
        Some(&program_context.payer.pubkey()),
    );

    transaction.sign(&[&program_context.payer], program_context.last_blockhash);

    program_context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}