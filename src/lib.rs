use byteorder::{ByteOrder, LittleEndian};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
};

use std::mem;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("You are getting half baked with cluutch.io");
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    // Get the account holding quote data
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("The half baked data account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // The data must be large enough to hold a u32 for the price of an ounce of weed in pennies
    if account.try_data_len()? < mem::size_of::<u32>() {
        msg!("The half baked account data length is too small for u32");
        return Err(ProgramError::InvalidAccountData);
    }

    let mut data = account.try_borrow_mut_data()?;
    // we read the data from the instruction_data into the u32 variable new_price
    let new_price = LittleEndian::read_u32(&instruction_data[0..4]);

    // write the u32 number back to the first 4 bytes
    LittleEndian::write_u32(&mut data[0..4], new_price);

    msg!("Stored a new price {} on half baked!", new_price);

    Ok(())
}

#[cfg(test)]
mod test {
    use {
        super::*,
        // assert_matches::*,
        // solana_program::instruction::{AccountMeta, Instruction},
        solana_program::clock::Epoch,
        solana_program_test::*,
        // solana_sdk::{signature::Signer, transaction::Transaction},
    };

    #[tokio::test]
    async fn test_transaction() {

        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        LittleEndian::write_u32(&mut data, 0);
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let mut instruction_data: Vec<u8> = Vec::new();
        instruction_data.push(28);
        instruction_data.push(62);
        instruction_data.push(0);
        instruction_data.push(0);


        let accounts = vec![account];

        assert_eq!(LittleEndian::read_u32(&accounts[0].data.borrow()), 0);
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(LittleEndian::read_u32(&accounts[0].data.borrow()), 15900);


        // let program_id = Pubkey::new_unique();

        // let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        //     "bpf_program_template",
        //     program_id,
        //     processor!(process_instruction),
        // )
        // .start()
        // .await;

        // let mut transaction = Transaction::new_with_payer(
        //     &[Instruction {
        //         program_id,
        //         accounts: vec![AccountMeta::new(payer.pubkey(), false)],
        //         data: vec![1, 2],
        //     }],
        //     Some(&payer.pubkey()),
        // );
        // transaction.sign(&[&payer], recent_blockhash);

        // assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
}
