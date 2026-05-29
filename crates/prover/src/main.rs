//! SP1 guest program: run a phase-1 EVM bytecode and commit the top of stack.
//!
//! Input  (via `sp1_zkvm::io::read`): the bytecode as `Vec<u8>`.
//! Output (via `sp1_zkvm::io::commit`): the final top-of-stack value as 32 big-
//! endian bytes (zero if the stack is empty). These become the proof's public
//! values, so the host can check *what* was proven, not just that it ran.

#![no_main]

sp1_zkvm::entrypoint!(main);

use evm::{run, EvmState, U256};

pub fn main() {
    let code: Vec<u8> = sp1_zkvm::io::read();

    let mut state = EvmState::new(code);
    run(&mut state).expect("evm execution failed inside the zkVM");

    let top: U256 = state.stack.last().copied().unwrap_or(U256::ZERO);
    sp1_zkvm::io::commit_slice(&top.to_be_bytes::<32>());
}
