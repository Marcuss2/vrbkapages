use crate::chumsky_utils::integer;
use crate::riscv::ast::{
    BImmediate, BOpcode, IImmediate, IOpcode, Instruction, LOpcode, LSImmediate, Program, ROpcode,
    Register, SOpcode, Symbol, UImmediate, UOpcode,
};
use chumsky::prelude::*;
use chumsky::text::{inline_whitespace, newline, whitespace};

fn register<'src>() -> impl Parser<'src, &'src str, Register, extra::Err<Rich<'src, char>>> {
    choice((
        just("x")
            .ignore_then(integer::<u8>(5.try_into().unwrap(), false).map(|n| Register::from(n))),
        choice([
            just("zero").to(Register::from(0)),
            just("ra").to(Register::from(1)),
            just("sp").to(Register::from(2)),
            just("gp").to(Register::from(3)),
            just("tp").to(Register::from(4)),
            just("t0").to(Register::from(5)),
            just("t1").to(Register::from(6)),
            just("t2").to(Register::from(7)),
            just("s0").to(Register::from(8)),
            just("fp").to(Register::from(8)),
            just("s1").to(Register::from(9)),
            just("a0").to(Register::from(10)),
            just("a1").to(Register::from(11)),
            just("a2").to(Register::from(12)),
            just("a3").to(Register::from(13)),
            just("a4").to(Register::from(14)),
            just("a5").to(Register::from(15)),
            just("a6").to(Register::from(16)),
            just("a7").to(Register::from(17)),
            just("s2").to(Register::from(18)),
            just("s3").to(Register::from(19)),
            just("s4").to(Register::from(20)),
            just("s5").to(Register::from(21)),
            just("s6").to(Register::from(22)),
            just("s7").to(Register::from(23)),
            just("s8").to(Register::from(24)),
            just("s9").to(Register::from(25)),
            just("s10").to(Register::from(26)),
            just("s11").to(Register::from(27)),
            just("t3").to(Register::from(28)),
            just("t4").to(Register::from(29)),
            just("t5").to(Register::from(30)),
            just("t6").to(Register::from(31)),
        ]),
    ))
    .labelled("register")
}

fn i_instruction<'src>() -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>>
{
    let instr = |literal: &'static str, opcode: IOpcode| {
        just(literal)
            .to(opcode)
            .then_ignore(whitespace().at_least(1))
            .then(register().labelled("rd"))
            .padded()
            .then_ignore(just(','))
            .padded()
            .then(register().labelled("rs1"))
            .padded()
            .then_ignore(just(','))
            .padded()
            .then(
                integer::<i16>(
                    opcode.immediate_properties().0,
                    opcode.immediate_properties().1,
                )
                .labelled("immediate"),
            )
    };
    choice([
        instr("addi", IOpcode::Addi),
        instr("slti", IOpcode::Slti),
        instr("sltiu", IOpcode::Sltiu),
        instr("xori", IOpcode::Xori),
        instr("ori", IOpcode::Ori),
        instr("andi", IOpcode::Andi),
        instr("slli", IOpcode::Slli),
        instr("srli", IOpcode::Srli),
        instr("srai", IOpcode::Srai),
    ])
    .labelled("instruction")
    .map(|(((opcode, rd), rs1), imm)| Instruction::IType {
        opcode,
        rd,
        rs1,
        imm: IImmediate(imm),
    })
}

fn r_instruction<'src>() -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>>
{
    choice([
        just("add").to(ROpcode::Add),
        just("sub").to(ROpcode::Sub),
        just("sll").to(ROpcode::Sll),
        just("sltu").to(ROpcode::Sltu),
        just("slt").to(ROpcode::Slt),
        just("xor").to(ROpcode::Xor),
        just("srl").to(ROpcode::Srl),
        just("sra").to(ROpcode::Sra),
        just("or").to(ROpcode::Or),
        just("and").to(ROpcode::And),
    ])
    .labelled("instruction")
    .then_ignore(whitespace().at_least(1))
    .then(register().labelled("rd"))
    .padded()
    .then_ignore(just(','))
    .padded()
    .then(register().labelled("rs1"))
    .padded()
    .then_ignore(just(','))
    .padded()
    .then(register().labelled("rs2"))
    .map(|(((opcode, rd), rs1), rs2)| Instruction::RType {
        opcode,
        rd,
        rs1,
        rs2,
    })
}

fn s_instruction<'src>() -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>>
{
    choice([
        just("sb").to(SOpcode::Sb),
        just("sh").to(SOpcode::Sh),
        just("sw").to(SOpcode::Sw),
    ])
    .labelled("instruction")
    .then_ignore(whitespace().at_least(1))
    .then(register().labelled("rs2"))
    .padded()
    .then_ignore(just(','))
    .padded()
    .then(integer(12.try_into().unwrap(), true).labelled("offset"))
    .then_ignore(just('('))
    .then(register().labelled("rs1"))
    .then_ignore(just(')'))
    .map(|(((opcode, rs2), offset), rs1)| Instruction::SType {
        opcode,
        rs1,
        rs2,
        imm: LSImmediate(offset),
    })
}

fn l_instruction<'src>() -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>>
{
    choice([
        just("lb").to(LOpcode::Lb),
        just("lh").to(LOpcode::Lh),
        just("lw").to(LOpcode::Lw),
    ])
    .labelled("instruction")
    .then_ignore(whitespace().at_least(1))
    .then(register().labelled("rd"))
    .padded()
    .then_ignore(just(','))
    .padded()
    .then(integer(12.try_into().unwrap(), true).labelled("offset"))
    .then_ignore(just('('))
    .then(register().labelled("rs1"))
    .then_ignore(just(')'))
    .map(|(((opcode, rd), offset), rs1)| Instruction::LType {
        opcode,
        rd,
        rs1,
        imm: LSImmediate(offset),
    })
}

fn b_instruction<'src>() -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>>
{
    choice([
        just("beq").to(BOpcode::Beq),
        just("bne").to(BOpcode::Bne),
        just("bltu").to(BOpcode::Bltu),
        just("bgeu").to(BOpcode::Bgeu),
        just("blt").to(BOpcode::Blt),
        just("bge").to(BOpcode::Bge),
    ])
    .labelled("instruction")
    .then_ignore(whitespace().at_least(1))
    .then(register().labelled("rs1"))
    .padded()
    .then_ignore(just(','))
    .padded()
    .then(register().labelled("rs2"))
    .padded()
    .then_ignore(just(','))
    .padded()
    .then(
        integer(13.try_into().unwrap(), true)
            .labelled("offset")
            .validate(|int: i16, e, emitter| {
                if int % 2 == 1 {
                    emitter.emit(Rich::custom(
                        e.span(),
                        format!("Invalid offset: {}, cannot be odd in RISC-V!", int),
                    ));
                }
                int
            }),
    )
    .map(|(((opcode, rs1), rs2), imm)| Instruction::BType {
        opcode,
        rs1,
        rs2,
        imm: BImmediate(imm),
    })
}

fn u_instruction<'src>() -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>>
{
    choice([
        just("lui").to(UOpcode::Lui),
        just("auipc").to(UOpcode::Auipc),
    ])
    .labelled("instruction")
    .then_ignore(whitespace().at_least(1))
    .then(register().labelled("rd"))
    .padded()
    .then_ignore(just(','))
    .padded()
    .then(integer::<i32>(20.try_into().unwrap(), true))
    .map(|((opcode, rd), imm)| Instruction::UType {
        opcode,
        rd,
        imm: UImmediate(imm),
    })
}

fn instruction_parser<'src>() -> impl Parser<'src, &'src str, Symbol, extra::Err<Rich<'src, char>>>
{
    choice((
        i_instruction(),
        r_instruction(),
        s_instruction(),
        l_instruction(),
        b_instruction(),
        u_instruction(),
    ))
    .map(Symbol::from)
}

pub fn parse_riscv<'src>(assembly: &'src str) -> Result<Program, Vec<Rich<'src, char>>> {
    let parser = choice([instruction_parser()])
        .or_not()
        .padded_by(inline_whitespace())
        .separated_by(newline())
        .allow_leading()
        .allow_trailing()
        // TODO: Once .flatten() is implemented for ItemParser, use that.
        .collect::<Vec<Option<Symbol>>>();

    let symbols = parser
        .parse(assembly)
        .into_result()?
        .into_iter()
        .flatten()
        .collect();

    Ok(Program { symbols })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_instruction() {
        let input = "addi x1, x2, 5\n";
        let result = parse_riscv(input);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.symbols.len(), 1);
        if let Symbol::Instruction(Instruction::IType {
            opcode,
            rd,
            rs1,
            imm,
        }) = &program.symbols[0]
        {
            assert!(matches!(opcode, IOpcode::Addi));
            assert_eq!(*rd, Register::from(1));
            assert_eq!(*rs1, Register::from(2));
            assert_eq!(*imm, IImmediate(5));
        } else {
            panic!("Unexpected instruction.");
        }
    }

    #[test]
    fn test_i_instruction_addi() {
        let input = "addi x1, x2, 5";
        let result = i_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::IType {
            opcode,
            rd,
            rs1,
            imm,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, IOpcode::Addi));
        assert_eq!(*rd, Register::from(1));
        assert_eq!(*rs1, Register::from(2));
        assert_eq!(*imm, IImmediate(5));
    }

    #[test]
    fn test_i_instruction_slli() {
        let input = "slli x3, x4, 2";
        let result = i_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::IType {
            opcode,
            rd,
            rs1,
            imm,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, IOpcode::Slli));
        assert_eq!(*rd, Register::from(3));
        assert_eq!(*rs1, Register::from(4));
        assert_eq!(*imm, IImmediate(2));
    }

    #[test]
    fn test_i_instruction_negative_immediate() {
        let input = "xori x5, x6, -1";
        let result = i_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::IType {
            opcode,
            rd,
            rs1,
            imm,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(!result.has_errors());
        assert!(matches!(opcode, IOpcode::Xori));
        assert_eq!(*rd, Register::from(5));
        assert_eq!(*rs1, Register::from(6));
        assert_eq!(*imm, IImmediate(-1));
    }

    #[test]
    fn test_i_instruction_invalid_format() {
        let input = "addi x1, x2";
        let result = i_instruction().parse(input);
        assert!(result.has_errors());
    }

    #[test]
    fn test_r_instruction_add() {
        let input = "add x5, x1, x2";
        let result = r_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::RType {
            opcode,
            rd,
            rs1,
            rs2,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, ROpcode::Add));
        assert_eq!(*rd, Register::from(5));
        assert_eq!(*rs1, Register::from(1));
        assert_eq!(*rs2, Register::from(2));
    }

    #[test]
    fn test_r_instruction_srl() {
        let input = "srl x7, x3, x4";
        let result = r_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::RType {
            opcode,
            rd,
            rs1,
            rs2,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, ROpcode::Srl));
        assert_eq!(*rd, Register::from(7));
        assert_eq!(*rs1, Register::from(3));
        assert_eq!(*rs2, Register::from(4));
    }

    #[test]
    fn test_r_instruction_invalid_format() {
        let input = "add x1, x2";
        let result = r_instruction().parse(input);
        assert!(result.has_errors());
    }

    #[test]
    fn test_s_instruction_sw() {
        let input = "sw x1, 100(x2)";
        let result = s_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::SType {
            opcode,
            rs1,
            rs2,
            imm,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, SOpcode::Sw));
        assert_eq!(*rs2, Register::from(1));
        assert_eq!(*rs1, Register::from(2));
        assert_eq!(*imm, LSImmediate(100));
    }

    #[test]
    fn test_s_instruction_sh() {
        let input = "sh x3, -20(x4)";
        let result = s_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::SType {
            opcode,
            rs1,
            rs2,
            imm,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, SOpcode::Sh));
        assert_eq!(*rs2, Register::from(3));
        assert_eq!(*rs1, Register::from(4));
        assert_eq!(*imm, LSImmediate(-20));
    }

    #[test]
    fn test_s_instruction_invalid_format() {
        let input = "sw x1, 42";
        let result = s_instruction().parse(input);
        assert!(result.has_errors());
    }

    #[test]
    fn test_u_instruction_lui() {
        let input = "lui x1, 0x1234";
        let result = u_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::UType { opcode, rd, imm } = result.output().unwrap() else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, UOpcode::Lui));
        assert_eq!(*rd, Register::from(1));
        assert_eq!(*imm, UImmediate(0x1234));
    }

    #[test]
    fn test_u_instruction_auipc() {
        let input = "auipc x2, -0x5678";
        let result = u_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::UType { opcode, rd, imm } = result.output().unwrap() else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, UOpcode::Auipc));
        assert_eq!(*rd, Register::from(2));
        assert_eq!(*imm, UImmediate(-0x5678));
    }

    #[test]
    fn test_u_instruction_invalid_format() {
        let input = "lui x1";
        let result = u_instruction().parse(input);
        assert!(result.has_errors());
    }

    #[test]
    fn test_u_instruction_invalid_syntax() {
        let input = "auipc x3, 100(x4)";
        let result = u_instruction().parse(input);
        assert!(result.has_errors());
    }

    #[test]
    fn test_u_instruction_negative_immediate() {
        let input = "lui x5, -100";
        let result = u_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::UType { opcode, rd, imm } = result.output().unwrap() else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, UOpcode::Lui));
        assert_eq!(*rd, Register::from(5));
        assert_eq!(*imm, UImmediate(-100));
    }

    #[test]
    fn test_l_instruction_lw() {
        let input = "lw x1, 100(x2)";
        let result = l_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::LType {
            opcode,
            rd,
            rs1,
            imm,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, LOpcode::Lw));
        assert_eq!(*rd, Register::from(1));
        assert_eq!(*rs1, Register::from(2));
        assert_eq!(*imm, LSImmediate(100));
    }

    #[test]
    fn test_l_instruction_lb() {
        let input = "lb x3, 20(x4)";
        let result = l_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::LType {
            opcode,
            rd,
            rs1,
            imm,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, LOpcode::Lb));
        assert_eq!(*rd, Register::from(3));
        assert_eq!(*rs1, Register::from(4));
        assert_eq!(*imm, LSImmediate(20));
    }

    #[test]
    fn test_l_instruction_lh() {
        let input = "lh x5, 50(x6)";
        let result = l_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::LType {
            opcode,
            rd,
            rs1,
            imm,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, LOpcode::Lh));
        assert_eq!(*rd, Register::from(5));
        assert_eq!(*rs1, Register::from(6));
        assert_eq!(*imm, LSImmediate(50));
    }

    #[test]
    fn test_l_instruction_negative_offset() {
        let input = "lw x7, -30(x8)";
        let result = l_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::LType {
            opcode,
            rd,
            rs1,
            imm,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, LOpcode::Lw));
        assert_eq!(*rd, Register::from(7));
        assert_eq!(*rs1, Register::from(8));
        assert_eq!(*imm, LSImmediate(-30));
    }

    #[test]
    fn test_l_instruction_invalid_format() {
        let input = "lw x1, 42";
        let result = l_instruction().parse(input);
        assert!(result.has_errors());
    }

    #[test]
    fn test_b_instruction_beq() {
        let input = "beq x1, x2, 4";
        let result = b_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::BType {
            opcode,
            rs1,
            rs2,
            imm,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, BOpcode::Beq));
        assert_eq!(*rs1, Register::from(1));
        assert_eq!(*rs2, Register::from(2));
        assert_eq!(*imm, BImmediate(4));
    }

    #[test]
    fn test_b_instruction_bne_negative() {
        let input = "bne x3, x4, -8";
        let result = b_instruction().parse(input);
        for error in result.errors() {
            println!("{}", error);
        }
        assert!(!result.has_errors());
        let Instruction::BType {
            opcode,
            rs1,
            rs2,
            imm,
        } = result.output().unwrap()
        else {
            panic!("Unexpected instruction.")
        };
        assert!(matches!(opcode, BOpcode::Bne));
        assert_eq!(*rs1, Register::from(3));
        assert_eq!(*rs2, Register::from(4));
        assert_eq!(*imm, BImmediate(-8));
    }

    #[test]
    fn test_b_instruction_odd_offset() {
        let input = "beq x5, x6, 3";
        let result = b_instruction().parse(input);
        assert!(result.has_errors());
    }

    #[test]
    fn test_b_instruction_invalid_format() {
        let input = "beq x1, 4";
        let result = b_instruction().parse(input);
        assert!(result.has_errors());
    }
}
