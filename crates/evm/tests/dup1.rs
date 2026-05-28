use evm::{run, EvmError, EvmState, STACK_LIMIT, U256};

#[test]
fn dup1_duplicates_top() {
    // PUSH1 5, DUP1, STOP -> [5, 5]
    let mut state = EvmState::new(vec![0x60, 0x05, 0x80, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::from(5), U256::from(5)]);
    assert!(state.halted);
}

#[test]
fn dup1_duplicates_only_top() {
    // PUSH1 1, PUSH1 2, DUP1, STOP -> [1, 2, 2]
    let mut state = EvmState::new(vec![0x60, 0x01, 0x60, 0x02, 0x80, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(
        state.stack,
        vec![U256::from(1), U256::from(2), U256::from(2)]
    );
}

#[test]
fn dup1_on_empty_stack_underflows() {
    let mut state = EvmState::new(vec![0x80, 0x00]);
    assert_eq!(run(&mut state), Err(EvmError::StackUnderflow));
}

#[test]
fn dup1_at_stack_limit_overflows() {
    // A full stack (1024 items) + DUP1 would make 1025 -> StackOverflow.
    let mut state = EvmState::new(vec![0x80, 0x00]); // DUP1, STOP
    state.stack = vec![U256::from(7); STACK_LIMIT];
    assert_eq!(run(&mut state), Err(EvmError::StackOverflow));
    assert_eq!(state.stack.len(), STACK_LIMIT);
}
