use evm::{run, EvmState};

#[test]
fn it_compiles() {
    let mut state = EvmState::new(vec![0x00]);
    run(&mut state).unwrap();
    assert!(state.halted);
}
