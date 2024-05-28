use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::next_account_info;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};
use solana_program::{entrypoint, system_instruction};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TransferInstruction {
    pub lamports: u64,
}

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Transter $SQL example program_id {}", program_id);

    let accounts_iter = &mut accounts.iter();
    let from_account = next_account_info(accounts_iter)?;
    let to_account = next_account_info(accounts_iter)?;

    // we don't use given account as we rely on the system account received via system_rogram::id()
    // inside system_instruction::transter however you should check it with solana_program::system_program::check_id()
    let _system_program_account = next_account_info(accounts_iter)?;

    let lamports = TransferInstruction::try_from_slice(instruction_data)?.lamports;
    msg!(
        "transtering {} lamports from {} to {}",
        lamports,
        from_account.key,
        to_account.key
    );

    if from_account.lamports() < lamports {
        return Err(ProgramError::InsufficientFunds);
    }

    msg!(
        "from_account balance {} is enough to transter {}",
        from_account.lamports(),
        lamports,
    );

    let _ = invoke(
        &system_instruction::transfer(from_account.key, to_account.key, lamports),
        &[from_account.clone(), to_account.clone()],
    );
    Ok(())
}
