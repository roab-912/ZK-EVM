use evm::{run, EvmError, EvmState, STACK_LIMIT, U256};

#[test]
fn push1_pushes_value() {
    let mut state = EvmState::new(vec![0x60, 0x05, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::from(5)]);
    assert!(state.halted);
    assert_eq!(state.pc, 3);
}

#[test]
fn push1_zero_extends_byte() {
    let mut state = EvmState::new(vec![0x60, 0xff, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::from(255)]);
}

#[test]
fn push1_missing_immediate_is_zero() {
    // revm/geth: PUSH operand bytes past the end of code are read as zero.
    let mut state = EvmState::new(vec![0x60]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::ZERO]);
    assert!(state.halted);
}

#[test]
fn push1_without_stop_halts_implicitly() {
    let mut state = EvmState::new(vec![0x60, 0x05]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::from(5)]);
    assert!(state.halted);
}

#[test]
fn push1_stack_overflow() {
    // One push past the 1024-element limit must fail.
    let mut code = Vec::new();
    for _ in 0..(STACK_LIMIT + 1) {
        code.push(0x60);
        code.push(0x00);
    }
    let mut state = EvmState::new(code);
    assert_eq!(run(&mut state), Err(EvmError::StackOverflow));
    assert_eq!(state.stack.len(), STACK_LIMIT);
}
