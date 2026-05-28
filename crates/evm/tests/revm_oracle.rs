//! Validation différentielle contre revm (interpréteur EVM de référence).
//!
//! Pour chaque programme, on exécute notre interpréteur et celui de revm sur le
//! même bytecode, puis on vérifie qu'ils sont d'accord sur le résultat
//! observable. Pour v0.1 (STOP), l'observable est : halt en succès + stack vide.

use evm::{run, EvmState};
use revm::bytecode::Bytecode;
use revm::context_interface::DummyHost;
use revm::interpreter::{gas_table, instruction_table, InstructionResult, Interpreter};
use revm::primitives::{Bytes, U256};

/// Exécute `code` via l'interpréteur standalone de revm et renvoie le résultat
/// d'instruction final ainsi que la stack finale.
fn revm_exec(code: Vec<u8>) -> (InstructionResult, Vec<U256>) {
    let bytecode = Bytecode::new_raw(Bytes::from(code));
    let mut interp = Interpreter::default().with_bytecode(bytecode);
    let table = instruction_table();
    let gas = gas_table();
    let mut host = DummyHost::default();
    let action = interp.run_plain(&table, &gas, &mut host);
    let result = action.instruction_result().expect("revm did not return");
    (result, interp.stack.data().clone())
}

#[test]
fn stop_matches_revm() {
    let code = vec![0x00];

    let (revm_result, revm_stack) = revm_exec(code.clone());
    assert_eq!(revm_result, InstructionResult::Stop);

    let mut state = EvmState::new(code);
    run(&mut state).unwrap();

    assert!(state.halted);
    assert_eq!(state.stack, revm_stack);
    assert!(state.stack.is_empty());
}

#[test]
fn empty_code_matches_revm() {
    let code: Vec<u8> = vec![];

    let (revm_result, revm_stack) = revm_exec(code.clone());
    assert_eq!(revm_result, InstructionResult::Stop);

    let mut state = EvmState::new(code);
    run(&mut state).unwrap();

    assert!(state.halted);
    assert_eq!(state.stack, revm_stack);
}

#[test]
fn push1_matches_revm() {
    let programs: &[&[u8]] = &[
        &[0x60, 0x05, 0x00],             // PUSH1 5, STOP -> [5]
        &[0x60, 0xff, 0x00],             // PUSH1 255    -> [255]
        &[0x60, 0x00, 0x00],             // PUSH1 0      -> [0]
        &[0x60, 0x05],                   // no STOP, implicit halt -> [5]
        &[0x60],                         // missing immediate -> [0]
        &[0x60, 0x01, 0x60, 0x02, 0x00], // two pushes -> [1, 2]
    ];

    for &program in programs {
        let code = program.to_vec();
        let (revm_result, revm_stack) = revm_exec(code.clone());

        let mut state = EvmState::new(code.clone());
        run(&mut state).unwrap();

        assert!(state.halted, "not halted for {program:02x?}");
        assert_eq!(
            revm_result,
            InstructionResult::Stop,
            "revm result for {program:02x?}"
        );
        assert_eq!(state.stack, revm_stack, "stack mismatch for {program:02x?}");
    }
}
