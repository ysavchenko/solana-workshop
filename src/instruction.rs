use solana_program::pubkey::Pubkey;
use solana_program::program_error::ProgramError;
use solana_program::instruction::Instruction;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum OurProgramInstruction {

    Initialize,

}

pub fn initialize(program_id: &Pubkey,) -> Result<Instruction, ProgramError> {

    let data = OurProgramInstruction::Initialize.try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;

    let accounts = vec![];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}