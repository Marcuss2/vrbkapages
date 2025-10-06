use std::u16;

use assembly_compiler::belt::ast::{
    BeltConstantOp, BeltPos, BranchOp, ConstantOp, ImmediateOp, Instruction, RegOp, UnaryOp, ZeroOp,
};

use crate::belt::Belt;

#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Continue,
    Jump { dest: u16 },
    Stop,
    Fail,
}

const MEM_SIZE: usize = 65536;
const BELT_SIZE: usize = 16;

pub struct BeltMachine {
    pub belt: Belt<u16, BELT_SIZE>,
    pub memory: [u16; MEM_SIZE],
    pub pc: u16,
    pub sp: u16,
}

const PC_START: u16 = 0x0;
const SP_START: u16 = 0xFFFF;

impl BeltMachine {
    pub fn new() -> Self {
        Self {
            belt: Belt::default(),
            memory: [0; MEM_SIZE],
            pc: PC_START,
            sp: SP_START,
        }
    }

    pub fn with_state(belt: [u16; BELT_SIZE], memory: [u16; MEM_SIZE], pc: u16, sp: u16) -> Self {
        Self {
            belt: Belt::with_state(belt),
            memory,
            pc,
            sp,
        }
    }

    pub fn step(&mut self) -> ExecutionResult {
        let first_word = self.memory[self.pc as usize];
        let second_word = self.memory[self.pc.wrapping_add(1) as usize];
        let instruction = Instruction::decode(&[first_word, second_word]);
        if let Some((instruction, size)) = instruction {
            let exec_result = self.execute_instruction(instruction);
            match exec_result {
                ExecutionResult::Continue => {
                    self.pc = self.pc.wrapping_add(size as u16);
                }
                ExecutionResult::Jump { dest } => {
                    self.pc = dest;
                }
                ExecutionResult::Stop | ExecutionResult::Fail => {}
            }
            exec_result
        } else {
            ExecutionResult::Stop
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) -> ExecutionResult {
        match instruction {
            Instruction::BeltConstant { op, pos, constant } => match constant {
                assembly_compiler::belt::ast::ConstantOrLabel::Constant(c) => {
                    self.execute_belt_constant(op, pos, c)
                }
                assembly_compiler::belt::ast::ConstantOrLabel::Label(_) => unreachable!(),
            },
            Instruction::Constant { op, constant } => match constant {
                assembly_compiler::belt::ast::ConstantOrLabel::Constant(c) => {
                    self.execute_constant(op, c)
                }
                assembly_compiler::belt::ast::ConstantOrLabel::Label(_) => unreachable!(),
            },
            Instruction::Immediate { op, pos, imm } => self.execute_immediate(op, pos, imm),
            Instruction::Register { op, pos1, pos2 } => self.execute_register(op, pos1, pos2),
            Instruction::Branch {
                op,
                pos1,
                pos2,
                addr,
            } => self.execute_branch(op, pos1, pos2, addr),
            Instruction::Unary {
                op: UnaryOp::Jump,
                pos,
            } => self.execute_unary(UnaryOp::Jump, pos),
            Instruction::Unary { op, pos } => self.execute_unary(op, pos),
            Instruction::Zero { op } => self.execute_zeroop(op),
        }
    }

    pub fn execute_unary(&mut self, op: UnaryOp, BeltPos(pos): BeltPos) -> ExecutionResult {
        let pos = pos as usize;
        let exec_result = match op {
            UnaryOp::Load => {
                let addr = self.belt.peek_belt(pos);
                let value = self.memory[*addr as usize];
                self.belt.push_belt(value);
                ExecutionResult::Continue
            }
            UnaryOp::Jump => ExecutionResult::Jump {
                dest: *self.belt.peek_belt(pos),
            },
            UnaryOp::Push => {
                let value = self.belt.peek_belt(pos);
                self.stack_push(*value);
                ExecutionResult::Continue
            }
        };

        exec_result
    }

    pub fn execute_zeroop(&mut self, op: ZeroOp) -> ExecutionResult {
        let exec_result = match op {
            ZeroOp::Nop => ExecutionResult::Continue,
            ZeroOp::Pop => {
                let value = self.stack_pop();
                self.belt.push_belt(value);
                ExecutionResult::Continue
            }
            ZeroOp::Break => ExecutionResult::Stop,
        };

        exec_result
    }

    fn execute_belt_constant(
        &mut self,
        op: BeltConstantOp,
        BeltPos(pos): BeltPos,
        constant: u16,
    ) -> ExecutionResult {
        let pos = pos as usize;
        let value = self.belt.peek_belt(pos);

        let (result_value, exec_result) = match op {
            BeltConstantOp::And => (Some(value & constant), ExecutionResult::Continue),
            BeltConstantOp::Or => (Some(value | constant), ExecutionResult::Continue),
            BeltConstantOp::Xor => (Some(value ^ constant), ExecutionResult::Continue),
        };

        if let Some(result) = result_value {
            self.belt.push_belt(result);
        }

        exec_result
    }

    fn execute_register(
        &mut self,
        op: RegOp,
        BeltPos(pos1): BeltPos,
        BeltPos(pos2): BeltPos,
    ) -> ExecutionResult {
        let value1 = self.belt.peek_belt(pos1 as usize);
        let value2 = self.belt.peek_belt(pos2 as usize);

        let result_value = match op {
            RegOp::Add => value1.wrapping_add(*value2),
            RegOp::Sub => value1.wrapping_sub(*value2),
            RegOp::Mul => value1.wrapping_mul(*value2),
            RegOp::Div => {
                if *value2 == 0 {
                    return ExecutionResult::Fail;
                }
                value1 / value2
            }
            RegOp::And => value1 & value2,
            RegOp::Or => value1 | value2,
            RegOp::Xor => value1 ^ value2,
            RegOp::Save => {
                self.memory[*value2 as usize] = *value1;
                return ExecutionResult::Continue;
            }
            RegOp::ShiftLeft => value1 << value2,
            RegOp::ShiftRight => value1 >> value2,
        };

        self.belt.push_belt(result_value);

        ExecutionResult::Continue
    }

    fn execute_immediate(
        &mut self,
        op: ImmediateOp,
        BeltPos(pos): BeltPos,
        imm: u8,
    ) -> ExecutionResult {
        let value = *self.belt.peek_belt(pos as usize);

        match op {
            ImmediateOp::ShiftLeft => {
                let result = value << imm;
                self.belt.push_belt(result);
                ExecutionResult::Continue
            }
            ImmediateOp::ShiftRight => {
                let result = value >> imm;
                self.belt.push_belt(result);
                ExecutionResult::Continue
            }
            ImmediateOp::Ret => {
                assert!(imm <= 16);

                let pre_ret_belt = self.belt.clone();

                for _ in 0..BELT_SIZE {
                    let val = self.stack_pop();
                    self.belt.push_belt(val);
                }

                for i in (0..imm).rev() {
                    self.belt.push_belt(*pre_ret_belt.peek_belt(i as usize));
                }

                ExecutionResult::Jump {
                    dest: self.stack_pop(),
                }
            }
        }
    }

    fn execute_branch(
        &mut self,
        op: BranchOp,
        BeltPos(pos1): BeltPos,
        BeltPos(pos2): BeltPos,
        target_addr: u16,
    ) -> ExecutionResult {
        let value1 = self.belt.peek_belt(pos1 as usize);
        let value2 = self.belt.peek_belt(pos2 as usize);

        let branch_taken = match op {
            BranchOp::BranchLower => value1 < value2,
            BranchOp::BranchLowerEq => value1 <= value2,
            BranchOp::BranchEq => value1 == value2,
        };

        if branch_taken {
            ExecutionResult::Jump { dest: target_addr }
        } else {
            ExecutionResult::Continue
        }
    }

    fn execute_constant(&mut self, op: ConstantOp, constant: u16) -> ExecutionResult {
        match op {
            ConstantOp::LoadFromMemory => {
                self.belt.push_belt(self.memory[constant as usize]);
                ExecutionResult::Continue
            }
            ConstantOp::LoadConstant => {
                self.belt.push_belt(constant);
                ExecutionResult::Continue
            }
            ConstantOp::Call => {
                for i in 0..BELT_SIZE {
                    self.stack_push(*self.belt.peek_belt(i));
                }
                ExecutionResult::Jump { dest: constant }
            }
            ConstantOp::Jump => ExecutionResult::Jump { dest: constant },
        }
    }

    pub fn stack_pop(&mut self) -> u16 {
        self.sp = self.sp.wrapping_add(1);
        self.memory[self.sp as usize]
    }

    pub fn stack_push(&mut self, value: u16) {
        self.memory[self.sp as usize] = value;
        self.sp = self.sp.wrapping_sub(1);
    }
}

impl Default for BeltMachine {
    fn default() -> Self {
        Self::new()
    }
}
