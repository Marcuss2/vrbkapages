use std::num::NonZeroU32;
use derive_more::From;

#[derive(Debug)]
pub struct Program {
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, From)]
pub enum Symbol {
    Instruction(Instruction),
    Comment,
    Directive(Directive),
}

#[derive(Debug)]
pub struct Alignment {
    pub alignment: usize,
}

#[derive(Debug)]
pub enum Directive {
    Alignment(Alignment),
    Other(String),
}

#[derive(Copy, Clone, Debug)]
pub enum IOpcode {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
}

#[derive(Copy, Clone, Debug)]
pub enum ROpcode {
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
}

#[derive(Copy, Clone, Debug)]
pub enum SOpcode {
    Sb,
    Sh,
    Sw,
}

#[derive(Copy, Clone, Debug)]
pub enum LOpcode {
    Lb,
    Lh,
    Lw,
}

#[derive(Copy, Clone, Debug)]
pub enum BOpcode {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Copy, Clone, Debug)]
pub enum JOpcode {
    Jal,
}

#[derive(Copy, Clone, Debug)]
pub enum UOpcode {
    Lui,
    Auipc,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IImmediate(pub i16);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LSImmediate(pub i16);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UImmediate(pub i32);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BImmediate(pub i16);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JImmediate(pub i32);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Register(u8);

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        Register(value)
    }
}

#[derive(Debug)]
pub enum Instruction {
    IType {
        opcode: IOpcode,
        rd: Register,
        rs1: Register,
        imm: IImmediate,
    },
    UType {
        opcode: UOpcode,
        rd: Register,
        imm: UImmediate,
    },
    RType {
        opcode: ROpcode,
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    JType {
        opcode: JOpcode,
        rd: Register,
        imm: JImmediate,
    },
    BType {
        opcode: BOpcode,
        rs1: Register,
        rs2: Register,
        imm: BImmediate,
    },
    SType {
        opcode: SOpcode,
        rs1: Register,
        rs2: Register,
        imm: LSImmediate,
    },
    LType {
        opcode: LOpcode,
        rd: Register,
        rs1: Register,
        imm: LSImmediate,
    },
}

impl IOpcode {
    pub fn immediate_properties(&self) -> (NonZeroU32, bool) {
        match self {
            IOpcode::Addi => (12.try_into().unwrap(), true),
            IOpcode::Slti => (12.try_into().unwrap(), true),
            IOpcode::Sltiu => (12.try_into().unwrap(), false),
            IOpcode::Xori => (12.try_into().unwrap(), true),
            IOpcode::Ori => (12.try_into().unwrap(), true),
            IOpcode::Andi => (12.try_into().unwrap(), true),
            IOpcode::Slli => (5.try_into().unwrap(), false),
            IOpcode::Srli => (5.try_into().unwrap(), false),
            IOpcode::Srai => (5.try_into().unwrap(), false),
        }
    }
}
