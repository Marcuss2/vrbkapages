use assembly_compiler::belt::ast::{ConstantOp, ConstantOrLabel};
use belt_interpreter::{
    BeltConstantOp, BeltMachine, BeltPos, BranchOp, ExecutionResult, ImmediateOp, Instruction,
    RegOp, UnaryOp, ZeroOp,
};

fn machine_with_program(program: &[Instruction]) -> BeltMachine {
    let mut m = BeltMachine::new();
    m.memory
        .iter_mut()
        .zip(
            program
                .iter()
                .map(|instr| {
                    let (first, second_optional) = instr.encode();
                    [Some(first), second_optional]
                })
                .flatten()
                .flatten(),
        )
        .for_each(|(mem, new)| *mem = new);
    m
}

#[test]
fn test_belt_constant_and() {
    let mut m = machine_with_program(&[
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(0x00FF),
        },
        Instruction::BeltConstant {
            op: BeltConstantOp::And,
            pos: BeltPos(0),
            constant: ConstantOrLabel::Constant(0x0F0F),
        },
    ]);
    assert_eq!(m.step(), ExecutionResult::Continue); // load
    assert_eq!(m.step(), ExecutionResult::Continue); // and
    assert_eq!(m.belt.peek_belt(0), &0x000F);
}

#[test]
fn test_register_add() {
    let mut m = machine_with_program(&[
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(5),
        },
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(7),
        },
        Instruction::Register {
            op: RegOp::Add,
            pos1: BeltPos(0),
            pos2: BeltPos(1),
        },
    ]);
    m.step();
    m.step();
    assert_eq!(m.step(), ExecutionResult::Continue);
    assert_eq!(m.belt.peek_belt(0), &12);
}

#[test]
fn test_branch_eq_taken() {
    let mut m = machine_with_program(&[
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(42),
        },
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(42),
        },
        Instruction::Branch {
            op: BranchOp::BranchEq,
            pos1: BeltPos(0),
            pos2: BeltPos(1),
            addr: 0x1234,
        },
    ]);
    m.step();
    m.step();
    assert_eq!(m.step(), ExecutionResult::Jump { dest: 0x1234 });
}

#[test]
fn test_branch_eq_not_taken() {
    let mut m = machine_with_program(&[
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(1),
        },
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(2),
        },
        Instruction::Branch {
            op: BranchOp::BranchEq,
            pos1: BeltPos(0),
            pos2: BeltPos(1),
            addr: 0x1234,
        },
    ]);
    m.step();
    m.step();
    assert_eq!(m.step(), ExecutionResult::Continue);
}

#[test]
fn test_load_memory() {
    let mut m = machine_with_program(&[
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(0x0042),
        },
        Instruction::Unary {
            op: UnaryOp::Load,
            pos: BeltPos(0),
        },
    ]);
    m.memory[0x0042] = 0xABCD;
    m.step();
    assert_eq!(m.step(), ExecutionResult::Continue);
    assert_eq!(m.belt.peek_belt(0), &0xABCD);
}

#[test]
fn test_save_memory() {
    let mut m = machine_with_program(&[
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(0x00FF),
        },
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(0x0042),
        },
        Instruction::Register {
            op: RegOp::Save,
            pos1: BeltPos(1),
            pos2: BeltPos(0),
        },
    ]);
    m.step();
    m.step();
    assert_eq!(m.step(), ExecutionResult::Continue);
    assert_eq!(m.memory[0x0042], 0x00FF);
}

#[test]
fn test_stack_push_pop() {
    let mut m = machine_with_program(&[
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(0xBEEF),
        },
        Instruction::Unary {
            op: UnaryOp::Push,
            pos: BeltPos(0),
        },
        Instruction::Zero { op: ZeroOp::Pop },
    ]);
    m.step();
    m.step();
    assert_eq!(m.step(), ExecutionResult::Continue);
    assert_eq!(m.belt.peek_belt(0), &0xBEEF);
}

#[test]
fn test_div_by_zero_fail() {
    let mut m = machine_with_program(&[
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(10),
        },
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(0),
        },
        Instruction::Register {
            op: RegOp::Div,
            pos1: BeltPos(1),
            pos2: BeltPos(0),
        },
    ]);
    m.step();
    m.step();
    assert_eq!(m.step(), ExecutionResult::Fail);
}

#[test]
fn test_jump() {
    let mut m = machine_with_program(&[
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(0x1234),
        },
        Instruction::Unary {
            op: UnaryOp::Jump,
            pos: BeltPos(0),
        },
    ]);
    m.step();
    assert_eq!(m.step(), ExecutionResult::Jump { dest: 0x1234 });
}

#[test]
fn test_ret_simple() {
    let mut m = machine_with_program(&[
        Instruction::Constant {
            op: ConstantOp::LoadConstant,
            constant: ConstantOrLabel::Constant(0x00FF),
        },
        Instruction::Immediate {
            op: ImmediateOp::Ret,
            pos: BeltPos(0),
            imm: 1,
        },
    ]);
    m.step();

    m.stack_push(0x1111);
    for i in 1..=16 {
        m.stack_push(i);
    }

    assert_eq!(m.step(), ExecutionResult::Jump { dest: 0x1111 });
    assert_eq!(m.belt.peek_belt(0), &0x00FF);
    assert_eq!(m.pc, 0x1111)
}

#[test]
fn test_nop() {
    let mut m = machine_with_program(&[Instruction::Zero { op: ZeroOp::Nop }]);
    assert_eq!(m.step(), ExecutionResult::Continue);
}

#[test]
fn test_break() {
    let mut m = machine_with_program(&[Instruction::Zero { op: ZeroOp::Break }]);
    assert_eq!(m.step(), ExecutionResult::Stop);
}
