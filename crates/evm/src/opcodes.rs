use crate::errors::EvmError;

/// EVM opcodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /// `0x00` — halt execution.
    Stop,
    /// `0x60` — push a 1-byte immediate (zero-extended) onto the stack.
    Push1,
}

impl Opcode {
    /// The byte value that encodes this opcode.
    pub const fn as_byte(self) -> u8 {
        match self {
            Opcode::Stop => 0x00,
            Opcode::Push1 => 0x60,
        }
    }

    /// How many bytes the program counter advances past this opcode: `1` for the
    /// opcode itself plus the size of any inline immediate.
    ///
    /// Anticipates `PUSH2`..`PUSH32` (advance = `1 + n`).
    pub const fn advance(self) -> usize {
        match self {
            Opcode::Push1 => 2,
            _ => 1,
        }
    }
}

impl TryFrom<u8> for Opcode {
    type Error = EvmError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Opcode::Stop),
            0x60 => Ok(Opcode::Push1),
            other => Err(EvmError::UnknownOpcode(other)),
        }
    }
}
