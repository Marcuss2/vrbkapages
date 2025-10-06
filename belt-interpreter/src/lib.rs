//! Belt Machine Interpreter - Mill Computing Style
//!
//! This implements a belt-based virtual machine with a fixed-size rotating belt
//! where items "move" and fall off the end. This is inspired by the
//! Mill Computing architecture where the belt is always full.

mod belt;
mod machine;

pub use machine::{
    BeltMachine, ExecutionResult
};

// Re-export the assembly compiler types for convenience
pub use assembly_compiler::belt::ast::{
    BeltPos, BranchOp, BeltConstantOp, ImmediateOp, Instruction, RegOp, UnaryOp, ZeroOp,
};
