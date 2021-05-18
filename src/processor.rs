

use solana_program::{
    pubkey::Pubkey, account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
};
use crate::instruction::OurProgramInstruction;
use borsh::BorshDeserialize;

/// Program state handler.
pub struct Processor {}
impl Processor {

    fn process_initialize(_program_id: &Pubkey,
        _accounts: &[AccountInfo]) -> ProgramResult {
        
        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = OurProgramInstruction::try_from_slice(input)
            .or(Err(ProgramError::InvalidInstructionData))?;
        match instruction {
            OurProgramInstruction::Initialize => {
                msg!("OurProgramInstruction::Initialize");
                msg!("Hello, world!");
                Self::process_initialize(program_id, accounts)
            }
        }
    }
}