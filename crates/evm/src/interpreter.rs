use crate::errors::EvmError;
use crate::opcodes::Opcode;
use crate::state::EvmState;

/// Execute a single step: decode the opcode at `pc` and apply its effect.
///
/// Running past the end of the code is treated as an implicit `STOP`, matching
/// revm / geth behaviour.
pub fn step(state: &mut EvmState) -> Result<(), EvmError> {
    if state.halted {
        return Err(EvmError::Halted);
    }
    let Some(&byte) = state.code.get(state.pc) else {
        state.halted = true;
        return Ok(());
    };
    match Opcode::try_from(byte)? {
        Opcode::Stop => {
            state.halted = true;
            state.pc += 1;
        }
    }
    Ok(())
}

/// Run the interpreter until it halts.
pub fn run(state: &mut EvmState) -> Result<(), EvmError> {
    while !state.halted {
        step(state)?;
    }
    Ok(())
}
