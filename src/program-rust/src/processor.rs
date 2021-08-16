use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,

};



#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Amount{
    pub amount: u64,

}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TransInstruction{
    pub playerid: u64,

}


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let playerinfo = TransInstruction::try_from_slice(instruction_data).map_err(|err| {
        msg!("Receiving message as string utf8 failed, {:?}", err);
        ProgramError::InvalidInstructionData  
      })?;
    msg!("Greeting passed to program is {:?}", playerinfo);
    msg!("Player info ");

    let data = &mut &mut account.data.borrow_mut();
    msg!("Start save instruction into data");
    data[..instruction_data.len()].copy_from_slice(&instruction_data);

    
    //Create in iterator to safety reference accounts in the slice

    // As part of the program specification the first account is the source
    // account and the second is the destination account
    let message = Amount::try_from_slice(instruction_data).map_err(|err| {
        msg!("Receiving message as string utf8 failed, {:?}", err);
        ProgramError::InvalidInstructionData  
      })?;
    //msg!("Greeting passed to program is {:?}", message);
    //msg!("source account");

   



    let source_info = next_account_info(accounts_iter)?;
    let destination_info = next_account_info(accounts_iter)?;

    // Withdraw five lamports from the source
    **source_info.try_borrow_mut_lamports()? -= message.amount;
    // Deposit five lamports into the destination
    **destination_info.try_borrow_mut_lamports()? += message.amount;

    Ok(())
}