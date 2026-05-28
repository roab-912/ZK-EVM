//! Pure EVM interpreter (no ZK).
//!
//! Implemented so far: `STOP` (0x00), `POP` (0x50), `PUSH1` (0x60). [`run`]
//! executes bytecode until it halts.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod errors;
pub mod interpreter;
pub mod opcodes;
pub mod state;

pub use ruint::aliases::U256;

pub use errors::EvmError;
pub use interpreter::{run, step};
pub use opcodes::Opcode;
pub use state::{EvmState, STACK_LIMIT};
