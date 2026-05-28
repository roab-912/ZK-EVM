use evm::{step, EvmError, EvmState};

#[test]
fn it_compiles() {
    let mut state = EvmState::new(vec![0x00]);
    assert_eq!(state.pc, 0);
    assert!(!state.halted);
    // No opcode is implemented yet in v0.0.
    assert_eq!(step(&mut state), Err(EvmError::UnknownOpcode(0x00)));
}

#[test]
fn empty_code_is_invalid_pc() {
    let mut state = EvmState::new(vec![]);
    assert_eq!(step(&mut state), Err(EvmError::InvalidPc));
}
