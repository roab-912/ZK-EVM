use core::fmt;

/// Errors that can occur during EVM execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvmError {
    /// Tried to pop/read more items than the stack holds.
    StackUnderflow,
    /// Pushing would exceed the 1024-element stack limit.
    StackOverflow,
    /// Encountered a byte that does not map to a known opcode.
    UnknownOpcode(u8),
    /// Execution continued after the machine had already halted.
    Halted,
    /// The program counter points outside the code bounds.
    InvalidPc,
}

impl fmt::Display for EvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvmError::StackUnderflow => write!(f, "stack underflow"),
            EvmError::StackOverflow => write!(f, "stack overflow"),
            EvmError::UnknownOpcode(op) => write!(f, "unknown opcode: {op:#04x}"),
            EvmError::Halted => write!(f, "execution halted"),
            EvmError::InvalidPc => write!(f, "invalid program counter"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EvmError {}
