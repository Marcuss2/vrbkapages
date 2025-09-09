use derive_more::From;

#[derive(Debug)]
pub struct Program {
    pub symbols: Vec<Symbol>,
}

#[derive(Clone, Debug, From)]
pub enum Symbol {
    Instruction(Instruction),
    Comment,
    Directive(Directive),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Alignment {
    pub alignment: usize,
}

#[derive(Clone, Debug)]
pub enum Directive {
    Alignment(Alignment),
    Other(String),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ConstantOp {
    And,
    Or,
    Xor,
    Jump,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ImmediateOp {
    Left,
    Right,
    Call,
    Ret,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RegOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Mul,
    Div,
    Save,
    ShiftLeft,
    ShiftRight,
    BranchLower,
    BranchLowerEq,
    BranchEq,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum UnaryOp {
    Load,
    Jump,
    Push,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ZeroOp {
    Nop,
    Pop,
    Break,
}

#[derive(PartialEq, Copy, Clone, Debug, From)]
#[repr(transparent)]
pub struct BeltPos(pub u8);

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Instruction {
    Constant { op: ConstantOp, pos: BeltPos, constant: u16 },
    LoadConstant { constant: u16 },
    Immediate { op: ImmediateOp, pos: BeltPos, imm: u8 },
    Register { op: RegOp, pos1: BeltPos, pos2: BeltPos },
    Unary { op: UnaryOp, pos: BeltPos },
    Zero { op: ZeroOp },
}
