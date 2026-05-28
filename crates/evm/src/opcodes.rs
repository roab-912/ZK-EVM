use crate::errors::EvmError;

/// EVM opcodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /// `0x00` — halt execution.
    Stop,
}

impl Opcode {
    /// The byte value that encodes this opcode.
    pub const fn as_byte(self) -> u8 {
        match self {
            Opcode::Stop => 0x00,
        }
    }
}

impl TryFrom<u8> for Opcode {
    type Error = EvmError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Opcode::Stop),
            other => Err(EvmError::UnknownOpcode(other)),
        }
    }
}
