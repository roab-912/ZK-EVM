use evm::{run, EvmError, EvmState, U256};

#[test]
fn swap1_swaps_top_and_second() {
    // PUSH1 1, PUSH1 2, SWAP1, STOP -> [2, 1] (bottom -> top)
    let mut state = EvmState::new(vec![0x60, 0x01, 0x60, 0x02, 0x90, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(state.stack, vec![U256::from(2), U256::from(1)]);
    assert!(state.halted);
}

#[test]
fn swap1_leaves_deeper_items_untouched() {
    // PUSH1 1, PUSH1 2, PUSH1 3, SWAP1, STOP -> [1, 3, 2]
    let mut state = EvmState::new(vec![0x60, 0x01, 0x60, 0x02, 0x60, 0x03, 0x90, 0x00]);
    run(&mut state).unwrap();
    assert_eq!(
        state.stack,
        vec![U256::from(1), U256::from(3), U256::from(2)]
    );
}

#[test]
fn swap1_on_empty_stack_underflows() {
    let mut state = EvmState::new(vec![0x90, 0x00]);
    assert_eq!(run(&mut state), Err(EvmError::StackUnderflow));
}

#[test]
fn swap1_with_single_item_underflows() {
    // PUSH1 5, SWAP1, STOP -> needs a second item.
    let mut state = EvmState::new(vec![0x60, 0x05, 0x90, 0x00]);
    assert_eq!(run(&mut state), Err(EvmError::StackUnderflow));
}
