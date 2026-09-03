use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    msg!("Doomsday Token Program");
    msg!("Instruction: {:?}", instruction_data);
    Ok(())
}
