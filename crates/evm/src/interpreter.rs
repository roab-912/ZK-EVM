use ruint::aliases::U256;

use crate::errors::EvmError;
use crate::opcodes::Opcode;
use crate::state::{EvmState, STACK_LIMIT};

/// Push a value onto the stack, enforcing the 1024-element limit.
fn push(state: &mut EvmState, value: U256) -> Result<(), EvmError> {
    if state.stack.len() >= STACK_LIMIT {
        return Err(EvmError::StackOverflow);
    }
    state.stack.push(value);
    Ok(())
}

/// Pop the top stack item, or `StackUnderflow` if the stack is empty.
fn pop1(state: &mut EvmState) -> Result<U256, EvmError> {
    state.stack.pop().ok_or(EvmError::StackUnderflow)
}

/// Pop the top two items as `(top, second)`, or `StackUnderflow`.
fn pop2(state: &mut EvmState) -> Result<(U256, U256), EvmError> {
    let top = pop1(state)?;
    let second = pop1(state)?;
    Ok((top, second))
}

/// Execute a single step: decode the opcode at `pc` and apply its effect.
///
/// Running past the end of the code is treated as an implicit `STOP`, matching
/// revm / geth behaviour. Likewise, immediate bytes that run past the end of the
/// code (e.g. a trailing `PUSH1`) are read as zero.
pub fn step(state: &mut EvmState) -> Result<(), EvmError> {
    if state.halted {
        return Err(EvmError::Halted);
    }
    let Some(&byte) = state.code.get(state.pc) else {
        state.halted = true;
        return Ok(());
    };
    let opcode = Opcode::try_from(byte)?;
    match opcode {
        Opcode::Stop => {
            state.halted = true;
        }
        Opcode::Add => {
            let (a, b) = pop2(state)?;
            push(state, a.wrapping_add(b))?;
        }
        Opcode::Pop => {
            pop1(state)?;
        }
        Opcode::Push1 => {
            let imm = state.code.get(state.pc + 1).copied().unwrap_or(0);
            push(state, U256::from(imm))?;
        }
    }
    state.pc += opcode.advance();
    Ok(())
}

/// Run the interpreter until it halts.
pub fn run(state: &mut EvmState) -> Result<(), EvmError> {
    while !state.halted {
        step(state)?;
    }
    Ok(())
}
