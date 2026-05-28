use evm::{run, EvmError, EvmState, U256};

#[test]
fn three_times_four_is_twelve() {
    // PUSH1 3, PUSH1 4, MUL, STOP
    let mut state = EvmState::new(vec![0x60, 0x03, 0x60, 0x04, 0x02, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::from(12)]);
    assert!(state.halted);
}

#[test]
fn times_zero_is_zero() {
    // PUSH1 7, PUSH1 0, MUL -> 0
    let mut state = EvmState::new(vec![0x60, 0x07, 0x60, 0x00, 0x02, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::ZERO]);
}

#[test]
fn times_one_is_identity() {
    // PUSH1 42, PUSH1 1, MUL -> 42
    let mut state = EvmState::new(vec![0x60, 0x2a, 0x60, 0x01, 0x02, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::from(42)]);
}

#[test]
fn mul_wraps_on_overflow() {
    // 2^128 * 2^128 = 2^256 == 0 (mod 2^256). PUSH32 not available yet (v0.10),
    // so build the stack manually.
    let two_pow_128 = U256::from(1) << 128;
    let mut state = EvmState::new(vec![0x02, 0x00]); // MUL, STOP
    state.stack = vec![two_pow_128, two_pow_128];
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::ZERO]);
}

#[test]
fn mul_underflow_single_operand() {
    let mut state = EvmState::new(vec![0x60, 0x05, 0x02, 0x00]);
    assert_eq!(run(&mut state), Err(EvmError::StackUnderflow));
}
