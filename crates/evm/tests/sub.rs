use evm::{run, EvmError, EvmState, U256};

#[test]
fn top_minus_second() {
    // PUSH1 3, PUSH1 5, SUB, STOP -> top=5, second=3 -> 5 - 3 = 2
    let mut state = EvmState::new(vec![0x60, 0x03, 0x60, 0x05, 0x03, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::from(2)]);
    assert!(state.halted);
}

#[test]
fn sub_wraps_on_underflow() {
    // PUSH1 5, PUSH1 3, SUB -> top=3, second=5 -> 3 - 5 = 2^256 - 2 = MAX - 1
    let mut state = EvmState::new(vec![0x60, 0x05, 0x60, 0x03, 0x03, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::MAX - U256::from(1)]);
}

#[test]
fn sub_equal_operands_is_zero() {
    // PUSH1 7, PUSH1 7, SUB -> 0
    let mut state = EvmState::new(vec![0x60, 0x07, 0x60, 0x07, 0x03, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::ZERO]);
}

#[test]
fn sub_underflow_single_operand() {
    let mut state = EvmState::new(vec![0x60, 0x05, 0x03, 0x00]);
    assert_eq!(run(&mut state), Err(EvmError::StackUnderflow));
}
