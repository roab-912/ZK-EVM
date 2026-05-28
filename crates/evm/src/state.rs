use alloc::vec::Vec;
use ruint::aliases::U256;

/// Maximum number of elements allowed on the EVM stack.
pub const STACK_LIMIT: usize = 1024;

/// Minimal execution state for the interpreter.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EvmState {
    /// Operand stack (max [`STACK_LIMIT`] elements).
    pub stack: Vec<U256>,
    /// Program counter: index of the next opcode in `code`.
    pub pc: usize,
    /// Set once a halting opcode has run.
    pub halted: bool,
    /// Bytecode being executed.
    pub code: Vec<u8>,
}

impl EvmState {
    /// Create a fresh state ready to execute `code`.
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            stack: Vec::new(),
            pc: 0,
            halted: false,
            code,
        }
    }
}
