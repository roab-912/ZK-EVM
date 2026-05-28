//! Pure EVM interpreter (no ZK).
//!
//! v0.0 is a skeleton: it wires up the crate structure but contains no opcode
//! logic yet. The interpreter always reports an unknown opcode.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod errors;
pub mod interpreter;
pub mod opcodes;
pub mod state;

pub use errors::EvmError;
pub use interpreter::step;
pub use opcodes::Opcode;
pub use state::{EvmState, STACK_LIMIT};
