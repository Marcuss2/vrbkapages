use derive_more::From;

#[derive(Debug)]
pub struct Program {
    pub symbols: Vec<Symbol>,
}

#[derive(Clone, Debug, From, PartialEq)]
pub enum Symbol {
    Instruction(Instruction),
    Label(String),
    Directive(Directive),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Alignment {
    pub alignment: usize,
}

#[derive(Clone, Debug, From, PartialEq)]
pub enum Directive {
    Alignment(Alignment),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BeltConstantOp {
    And,
    Or,
    Xor,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ConstantOp {
    LoadFromMemory,
    LoadConstant,
    Call,
    Jump,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ImmediateOp {
    ShiftLeft,
    ShiftRight,
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
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BranchOp {
    BranchLower,
    BranchLowerEq,
    BranchEq,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum UnaryOp {
    Load,
    Push,
    Jump,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ZeroOp {
    Nop,
    Pop,
    Break,
}

#[derive(PartialEq, Clone, Copy, Debug, From)]
#[repr(transparent)]
pub struct BeltPos(pub u8);

#[derive(PartialEq, Clone, Debug, From)]
pub enum ConstantOrLabel {
    Constant(u16),
    Label(String),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Instruction {
    BeltConstant {
        op: BeltConstantOp,
        pos: BeltPos,
        constant: ConstantOrLabel,
    },
    Constant {
        op: ConstantOp,
        constant: ConstantOrLabel,
    },
    Immediate {
        op: ImmediateOp,
        pos: BeltPos,
        imm: u8,
    },
    Register {
        op: RegOp,
        pos1: BeltPos,
        pos2: BeltPos,
    },
    Branch {
        op: BranchOp,
        pos1: BeltPos,
        pos2: BeltPos,
        addr: u16,
    },
    Unary {
        op: UnaryOp,
        pos: BeltPos,
    },
    Zero {
        op: ZeroOp,
    },
}

impl Instruction {
    pub fn decode(buf: &[u16]) -> Option<(Instruction, usize)> {
        if buf.is_empty() {
            return None;
        }
        let word = buf[0];
        let main_op = (word >> 12) & 0xF;
        let sub_op = (word >> 8) & 0xF;
        let pos1 = ((word >> 4) & 0xF) as u8;
        let pos2 = (word & 0xF) as u8;

        let instr = match main_op {
            1 => {
                if buf.len() < 2 {
                    return None;
                }
                if pos2 != 0 {
                    return None;
                }
                let op = match sub_op {
                    0 => BeltConstantOp::And,
                    1 => BeltConstantOp::Or,
                    2 => BeltConstantOp::Xor,
                    _ => return None,
                };
                Instruction::BeltConstant {
                    op,
                    pos: BeltPos(pos1),
                    constant: ConstantOrLabel::Constant(buf[1]),
                }
            }
            2 => {
                if buf.len() < 2 {
                    return None;
                }
                if pos1 != 0 || pos2 != 0 {
                    return None;
                }
                let op = match sub_op {
                    0 => ConstantOp::LoadFromMemory,
                    1 => ConstantOp::LoadConstant,
                    2 => ConstantOp::Call,
                    3 => ConstantOp::Jump,
                    _ => return None,
                };
                Instruction::Constant {
                    op,
                    constant: ConstantOrLabel::Constant(buf[1]),
                }
            }
            3 => {
                let op = match sub_op {
                    0 => ImmediateOp::ShiftLeft,
                    1 => ImmediateOp::ShiftRight,
                    2 => ImmediateOp::Ret,
                    _ => return None,
                };
                Instruction::Immediate {
                    op,
                    pos: BeltPos(pos1),
                    imm: pos2,
                }
            }
            4 => {
                let op = match sub_op {
                    0 => RegOp::Add,
                    1 => RegOp::Sub,
                    2 => RegOp::And,
                    3 => RegOp::Or,
                    4 => RegOp::Xor,
                    5 => RegOp::Mul,
                    6 => RegOp::Div,
                    7 => RegOp::Save,
                    8 => RegOp::ShiftLeft,
                    9 => RegOp::ShiftRight,
                    _ => return None,
                };
                Instruction::Register {
                    op,
                    pos1: BeltPos(pos1),
                    pos2: BeltPos(pos2),
                }
            }
            5 => {
                if buf.len() < 2 {
                    return None;
                }
                let op = match sub_op {
                    0 => BranchOp::BranchLower,
                    1 => BranchOp::BranchLowerEq,
                    2 => BranchOp::BranchEq,
                    _ => return None,
                };
                Instruction::Branch {
                    op,
                    pos1: BeltPos(pos1),
                    pos2: BeltPos(pos2),
                    addr: buf[1],
                }
            }
            6 => {
                if pos2 != 0 {
                    return None;
                }
                let op = match sub_op {
                    0 => UnaryOp::Load,
                    1 => UnaryOp::Push,
                    2 => UnaryOp::Jump,
                    _ => return None,
                };
                Instruction::Unary {
                    op,
                    pos: BeltPos(pos1),
                }
            }
            7 => {
                if pos1 != 0 || pos2 != 0 {
                    return None;
                }
                let op = match sub_op {
                    0 => ZeroOp::Nop,
                    1 => ZeroOp::Pop,
                    2 => ZeroOp::Break,
                    _ => return None,
                };
                Instruction::Zero { op }
            }
            _ => return None,
        };

        let words_consumed = if main_op == 1 || main_op == 2 || main_op == 5 {
            2
        } else {
            1
        };
        Some((instr, words_consumed))
    }

    pub fn encode(&self) -> (u16, Option<u16>) {
        let (main_op, sub_op, pos1, pos2, constant) = match self.clone() {
            Instruction::BeltConstant { op, pos, constant } => {
                let main_op = 1;
                let sub_op = match op {
                    BeltConstantOp::And => 0,
                    BeltConstantOp::Or => 1,
                    BeltConstantOp::Xor => 2,
                };
                let BeltPos(pos) = pos;
                let constant = match constant {
                    ConstantOrLabel::Constant(c) => c,
                    ConstantOrLabel::Label(l) => panic!("Unresolved label: {l}!"),
                };
                (main_op, Some(sub_op), Some(pos), None, Some(constant))
            }
            Instruction::Constant { op, constant } => {
                let main_op = 2;
                let sub_op = match op {
                    ConstantOp::LoadFromMemory => 0,
                    ConstantOp::LoadConstant => 1,
                    ConstantOp::Call => 2,
                    ConstantOp::Jump => 3,
                };
                let constant = match constant {
                    ConstantOrLabel::Constant(c) => c,
                    ConstantOrLabel::Label(l) => panic!("Unresolved label: {l}!"),
                };
                (main_op, Some(sub_op), None, None, Some(constant))
            }
            Instruction::Immediate { op, pos, imm } => {
                let main_op = 3;
                let sub_op = match op {
                    ImmediateOp::ShiftLeft => 0,
                    ImmediateOp::ShiftRight => 1,
                    ImmediateOp::Ret => 2,
                };
                let BeltPos(pos) = pos;
                (main_op, Some(sub_op), Some(pos), Some(imm), None)
            }
            Instruction::Register { op, pos1, pos2 } => {
                let main_op = 4;
                let sub_op = match op {
                    RegOp::Add => 0,
                    RegOp::Sub => 1,
                    RegOp::And => 2,
                    RegOp::Or => 3,
                    RegOp::Xor => 4,
                    RegOp::Mul => 5,
                    RegOp::Div => 6,
                    RegOp::Save => 7,
                    RegOp::ShiftLeft => 8,
                    RegOp::ShiftRight => 9,
                };
                let BeltPos(pos1) = pos1;
                let BeltPos(pos2) = pos2;
                (main_op, Some(sub_op), Some(pos1), Some(pos2), None)
            }
            Instruction::Branch {
                op,
                pos1,
                pos2,
                addr,
            } => {
                let main_op = 5;
                let sub_op = match op {
                    BranchOp::BranchLower => 0,
                    BranchOp::BranchLowerEq => 1,
                    BranchOp::BranchEq => 2,
                };
                let BeltPos(pos1) = pos1;
                let BeltPos(pos2) = pos2;
                (main_op, Some(sub_op), Some(pos1), Some(pos2), Some(addr))
            }
            Instruction::Unary { op, pos } => {
                let main_op = 6;
                let sub_op = match op {
                    UnaryOp::Load => 0,
                    UnaryOp::Push => 1,
                    UnaryOp::Jump => 2,
                };
                let BeltPos(pos) = pos;
                (main_op, Some(sub_op), Some(pos), None, None)
            }
            Instruction::Zero { op } => {
                let main_op = 7;
                let sub_op = match op {
                    ZeroOp::Nop => 0,
                    ZeroOp::Pop => 1,
                    ZeroOp::Break => 2,
                };
                (main_op, Some(sub_op), None, None, None)
            }
        };

        let sub_op = sub_op.unwrap_or(0);
        let pos1 = pos1.unwrap_or(0) as u16;
        let pos2 = pos2.unwrap_or(0) as u16;
        let code = main_op << 12 | sub_op << 8 | pos1 << 4 | pos2;

        (code, constant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_encode_all_16bit() {
        // Test all 65536 possible 16-bit instruction words
        for word in 0..=u16::MAX {
            // Create a buffer with the instruction word and a dummy second word
            let buf = [word, 0x1234];

            // Try to decode the instruction
            if let Some((instr, words_consumed)) = Instruction::decode(&buf) {
                // Re-encode the instruction
                let (encoded_word, second_word) = instr.encode();

                // Compare the first word
                assert_eq!(
                    encoded_word, word,
                    "Mismatch for word 0x{:04X}, decoded instruction: {:?}",
                    word, instr
                );

                // For instructions that consume 2 words, also check the second word
                if words_consumed == 2 {
                    assert_eq!(
                        second_word,
                        Some(0x1234),
                        "Second word mismatch for word 0x{:04X}, decoded instruction {:?}",
                        word,
                        instr
                    );
                } else {
                    assert_eq!(
                        second_word, None,
                        "Unexpected second word for word 0x{:04X}, decoded instruction {:?}",
                        word, instr
                    );
                }
            }
        }
    }
}
