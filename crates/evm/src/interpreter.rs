use crate::errors::EvmError;
use crate::state::EvmState;

/// Execute a single EVM step.
///
/// v0.0 is a skeleton with no opcode decoding: it fetches the byte at the
/// current program counter and reports it as unknown. Real dispatch arrives in
/// v0.1.
pub fn step(state: &mut EvmState) -> Result<(), EvmError> {
    if state.halted {
        return Err(EvmError::Halted);
    }
    let op = *state.code.get(state.pc).ok_or(EvmError::InvalidPc)?;
    Err(EvmError::UnknownOpcode(op))
}
