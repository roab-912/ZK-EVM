use evm::{run, step, EvmError, EvmState, U256};

#[test]
fn two_plus_three_is_five() {
    // PUSH1 2, PUSH1 3, ADD, STOP
    let mut state = EvmState::new(vec![0x60, 0x02, 0x60, 0x03, 0x01, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::from(5)]);
    assert!(state.halted);
}

#[test]
fn add_wraps_on_overflow() {
    // MAX + 1 == 0 (mod 2^256). PUSH32 not available yet (v0.10), so build the
    // stack manually as the spec suggests.
    let mut state = EvmState::new(vec![0x01, 0x00]); // ADD, STOP
    state.stack = vec![U256::from(1), U256::MAX]; // second = 1, top = MAX
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::ZERO]);
}

#[test]
fn add_is_step_wrapping() {
    // (MAX - 2) + 5 == 2 (mod 2^256), checked on a single step.
    let mut state = EvmState::new(vec![0x01]);
    state.stack = vec![U256::from(5), U256::MAX - U256::from(2)];
    step(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::from(2)]);
}

#[test]
fn add_underflow_empty() {
    let mut state = EvmState::new(vec![0x01, 0x00]);
    assert_eq!(run(&mut state), Err(EvmError::StackUnderflow));
}

#[test]
fn add_underflow_single_operand() {
    // PUSH1 5, ADD -> only one operand
    let mut state = EvmState::new(vec![0x60, 0x05, 0x01, 0x00]);
    assert_eq!(run(&mut state), Err(EvmError::StackUnderflow));
}
