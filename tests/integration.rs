#![cfg(feature = "test-bpf")]
use solana_program_test::*;
use std::mem;
use {
    assert_matches::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{signature::Signer, transaction::Transaction, account::Account},
    solana_validator::test_validator::*,
};
use byteorder::{ByteOrder, LittleEndian};

#[tokio::test]
async fn test_helloworld() {
    let program_id = Pubkey::new_unique();
    let data_inputter_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new("half_baked", program_id, None);
    program_test.add_account(
        data_inputter_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Verify account has no quotes
    let data_inputter_account = banks_client
        .get_account(data_inputter_pubkey)
        .await
        .expect("get_account")
        .expect("data_inputter_account not found");
    assert_eq!(LittleEndian::read_u32(&data_inputter_account.data), 0);

    // Write new quote
    let mut instruction_data: [u8; 4] = [0; 4];
    LittleEndian::write_u32(&mut instruction_data[0..], 15900);

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new(
            program_id,
            &instruction_data, // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(data_inputter_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has 15900
    let data_inputter_account = banks_client
        .get_account(data_inputter_pubkey)
        .await
        .expect("get_account")
        .expect("data_inputter_account not found");
    assert_eq!(LittleEndian::read_u32(&data_inputter_account.data), 15900);

    // Write new quote
    LittleEndian::write_u32(&mut instruction_data[0..], 24029);
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new(
            program_id,
            &instruction_data, // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(data_inputter_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has new quote data
    let data_inputter_account = banks_client
        .get_account(data_inputter_pubkey)
        .await
        .expect("get_account")
        .expect("data_inputter_account not found");
    assert_eq!(LittleEndian::read_u32(&data_inputter_account.data), 24029);
}