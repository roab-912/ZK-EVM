//! Validation différentielle contre revm (interpréteur EVM de référence).
//!
//! Pour chaque programme, on exécute notre interpréteur et celui de revm sur le
//! même bytecode, puis on vérifie qu'ils sont d'accord sur le résultat
//! observable. Pour v0.1 (STOP), l'observable est : halt en succès + stack vide.

use evm::{run, EvmError, EvmState};
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

#[test]
fn pop_matches_revm() {
    let programs: &[&[u8]] = &[
        &[0x60, 0x05, 0x50, 0x00],             // PUSH1 5, POP -> []
        &[0x60, 0x01, 0x60, 0x02, 0x50, 0x00], // PUSH1 1, PUSH1 2, POP -> [1]
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

#[test]
fn pop_underflow_matches_revm() {
    // POP on an empty stack: both revm and our interpreter must fail.
    let code = vec![0x50, 0x00];

    let (revm_result, _) = revm_exec(code.clone());
    assert_eq!(revm_result, InstructionResult::StackUnderflow);

    let mut state = EvmState::new(code);
    assert_eq!(run(&mut state), Err(EvmError::StackUnderflow));
}

#[test]
fn add_matches_revm() {
    let programs: &[&[u8]] = &[
        &[0x60, 0x02, 0x60, 0x03, 0x01, 0x00], // 2 + 3 -> [5]
        &[0x60, 0x00, 0x60, 0x00, 0x01, 0x00], // 0 + 0 -> [0]
        &[0x60, 0xff, 0x60, 0x01, 0x01, 0x00], // 255 + 1 -> [256]
        &[0x60, 0x0a, 0x60, 0x14, 0x01, 0x00], // 10 + 20 -> [30]
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

#[test]
fn add_underflow_matches_revm() {
    // ADD with a single operand: both revm and our interpreter must fail.
    let code = vec![0x60, 0x05, 0x01, 0x00]; // PUSH1 5, ADD, STOP

    let (revm_result, _) = revm_exec(code.clone());
    assert_eq!(revm_result, InstructionResult::StackUnderflow);

    let mut state = EvmState::new(code);
    assert_eq!(run(&mut state), Err(EvmError::StackUnderflow));
}

#[test]
fn sub_matches_revm() {
    let programs: &[&[u8]] = &[
        &[0x60, 0x03, 0x60, 0x05, 0x03, 0x00], // 5 - 3 -> [2]
        &[0x60, 0x05, 0x60, 0x03, 0x03, 0x00], // 3 - 5 -> [MAX-1] (wrapping)
        &[0x60, 0x07, 0x60, 0x07, 0x03, 0x00], // 7 - 7 -> [0]
        &[0x60, 0x00, 0x60, 0xff, 0x03, 0x00], // 255 - 0 -> [255]
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

#[test]
fn sub_underflow_matches_revm() {
    let code = vec![0x60, 0x05, 0x03, 0x00]; // PUSH1 5, SUB, STOP

    let (revm_result, _) = revm_exec(code.clone());
    assert_eq!(revm_result, InstructionResult::StackUnderflow);

    let mut state = EvmState::new(code);
    assert_eq!(run(&mut state), Err(EvmError::StackUnderflow));
}
