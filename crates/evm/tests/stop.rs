use evm::{run, step, EvmError, EvmState};

#[test]
fn stop_halts() {
    let mut state = EvmState::new(vec![0x00]);
    run(&mut state).unwrap();
    assert!(state.halted);
    assert_eq!(state.pc, 1);
    assert!(state.stack.is_empty());
}

#[test]
fn only_first_stop_counts() {
    let mut state = EvmState::new(vec![0x00, 0x00, 0x00]);
    run(&mut state).unwrap();
    assert!(state.halted);
    assert_eq!(state.pc, 1);
}

#[test]
fn empty_code_is_implicit_stop() {
    let mut state = EvmState::new(vec![]);
    run(&mut state).unwrap();
    assert!(state.halted);
    assert_eq!(state.pc, 0);
    assert!(state.stack.is_empty());
}

#[test]
fn unknown_opcode_errors() {
    // 0x21 is an unassigned EVM opcode.
    let mut state = EvmState::new(vec![0x21]);
    assert_eq!(step(&mut state), Err(EvmError::UnknownOpcode(0x21)));
}

#[test]
fn step_after_halt_errors() {
    let mut state = EvmState::new(vec![0x00]);
    run(&mut state).unwrap();
    assert_eq!(step(&mut state), Err(EvmError::Halted));
}
