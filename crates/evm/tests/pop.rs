use evm::{run, EvmError, EvmState};

#[test]
fn push_then_pop_empties_stack() {
    // PUSH1 5, POP, STOP
    let mut state = EvmState::new(vec![0x60, 0x05, 0x50, 0x00]);
    run(&mut state).unwrap();
    assert!(state.stack.is_empty());
    assert!(state.halted);
    assert_eq!(state.pc, 4);
}

#[test]
fn pop_on_empty_stack_underflows() {
    // POP, STOP
    let mut state = EvmState::new(vec![0x50, 0x00]);
    assert_eq!(run(&mut state), Err(EvmError::StackUnderflow));
}

#[test]
fn pop_removes_only_top() {
    // PUSH1 1, PUSH1 2, POP, STOP -> [1]
    let mut state = EvmState::new(vec![0x60, 0x01, 0x60, 0x02, 0x50, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![evm::U256::from(1)]);
}
