#![cfg(feature = "test-sbf")]

use anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas};
use mollusk_svm::{result::Check, Mollusk};

#[test]
fn test_initialize() {
    let program_id = anchor_template::id();

    let mollusk = Mollusk::new(&program_id, "anchor_template");

    let instruction = Instruction::new_with_bytes(
        program_id,
        &anchor_template::instruction::Initialize {}.data(),
        anchor_template::accounts::Initialize {}.to_account_metas(None),
    );

    mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
}
