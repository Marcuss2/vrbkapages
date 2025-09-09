use crate::belt::ast::{
    BeltPos, ConstantOp, ImmediateOp, Instruction, Program, RegOp, Symbol, UnaryOp, ZeroOp,
};
use crate::chumsky_utils::integer;

use chumsky::prelude::*;
use chumsky::text::{inline_whitespace, newline};

fn belt_pos<'src>() -> impl Parser<'src, &'src str, BeltPos, extra::Err<Rich<'src, char>>> {
    just("b")
        .ignore_then(integer::<u8>(4.try_into().unwrap(), false).map(|n| BeltPos::from(n)))
        .labelled("belt position")
}

fn nop_instr<'src>() -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>> {
    choice([
        just("nop").to(ZeroOp::Nop),
        just("pop").to(ZeroOp::Pop),
        just("break").to(ZeroOp::Break),
    ])
    .labelled("instruction")
    .map(|op| Instruction::Zero { op })
}

fn constant_instr<'src>() -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>>
{
    choice([
        just("and").to(ConstantOp::And),
        just("or").to(ConstantOp::Or),
        just("xor").to(ConstantOp::Xor),
    ])
    .labelled("instruction")
    .then_ignore(inline_whitespace().at_least(1))
    .then(belt_pos().labelled("src"))
    .padded()
    .then(integer::<u16>(16.try_into().unwrap(), false).labelled("constant"))
    .map(|((operation, belt_pos), constant)| Instruction::Constant {
        op: operation,
        pos: belt_pos,
        constant,
    })
}

fn load_constant_instr<'src>(
) -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>> {
    just("lc")
        .labelled("instruction")
        .ignore_then(inline_whitespace().at_least(1))
        .ignore_then(integer::<u16>(16.try_into().unwrap(), false).labelled("constant"))
        .padded()
        .map(|constant| Instruction::LoadConstant { constant })
}

fn imm_instr<'src>() -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>> {
    choice((
        just("sl").to(ImmediateOp::Left),
        just("sr").to(ImmediateOp::Right),
    ))
    .labelled("instruction")
    .then_ignore(inline_whitespace().at_least(1))
    .then(belt_pos().labelled("src"))
    .then_ignore(inline_whitespace().at_least(1))
    .then(integer::<u8>(4.try_into().unwrap(), false).labelled("constant"))
    .padded()
    .map(|((op, belt_pos), imm)| Instruction::Immediate {
        op,
        pos: belt_pos,
        imm: imm,
    })
}

fn reg_instr<'src>() -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>> {
    choice((
        just("add").to(RegOp::Add),
        just("sub").to(RegOp::Sub),
        just("and").to(RegOp::And),
        just("or").to(RegOp::Or),
        just("xor").to(RegOp::Xor),
        just("mul").to(RegOp::Mul),
        just("div").to(RegOp::Div),
        just("save").to(RegOp::Save),
        just("sr").to(RegOp::ShiftRight),
        just("sl").to(RegOp::ShiftLeft),
    ))
    .labelled("instruction")
    .then_ignore(inline_whitespace().at_least(1))
    .then(belt_pos())
    .then_ignore(inline_whitespace().at_least(1))
    .then(belt_pos())
    .padded()
    .map(|((op, pos1), pos2)| Instruction::Register { op, pos1, pos2 })
}

fn unary_instr<'src>() -> impl Parser<'src, &'src str, Instruction, extra::Err<Rich<'src, char>>> {
    just("load")
        .labelled("instruction")
        .ignore_then(inline_whitespace().at_least(1))
        .ignore_then(belt_pos().labelled("address"))
        .padded()
        .map(|src| Instruction::Unary {
            op: UnaryOp::Load,
            pos: src,
        })
}

fn instruction_parser<'src>() -> impl Parser<'src, &'src str, Symbol, extra::Err<Rich<'src, char>>>
{
    choice((
        constant_instr(),
        load_constant_instr(),
        imm_instr(),
        unary_instr(),
        reg_instr(),
        nop_instr(),
    ))
    .map(Symbol::from)
}

fn comment<'src>() -> impl Parser<'src, &'src str, Symbol, extra::Err<Rich<'src, char>>> {
    just("#")
        .then(any().and_is(newline()).not())
        .repeated()
        .padded_by(inline_whitespace())
        .to(Symbol::Comment)
}

pub fn parse_belt<'src>(assembly: &'src str) -> Result<Program, Vec<Rich<'src, char>>> {
    let parser = choice((instruction_parser(), comment()))
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
mod test {
    use chumsky::Parser;
    use rstest::rstest;

    use crate::belt::{
        ast::{BeltPos, ConstantOp, ImmediateOp, Instruction, RegOp, Symbol, UnaryOp, ZeroOp},
        parser::instruction_parser,
    };

    #[rstest]
    #[case("and b0 0x1234", Instruction::Constant { op: ConstantOp::And, pos: BeltPos(0), constant: 0x1234 })]
    #[case("or b1 666", Instruction::Constant { op: ConstantOp::Or, pos: BeltPos(1), constant: 666 })]
    #[case("xor b2 0b1100", Instruction::Constant { op: ConstantOp::Xor, pos: BeltPos(2), constant: 0b1100 })]
    #[case("lc 0xDEF0", Instruction::LoadConstant { constant: 0xDEF0 })]
    #[case("sl b3 4", Instruction::Immediate { op: ImmediateOp::Left, pos: BeltPos(3), imm: 4 })]
    #[case("sr b4 5", Instruction::Immediate { op: ImmediateOp::Right, pos: BeltPos(4), imm: 5 })]
    #[case("add b5 b6", Instruction::Register { op: RegOp::Add, pos1: BeltPos(5), pos2: BeltPos(6) })]
    #[case("sub b7 b8", Instruction::Register { op: RegOp::Sub, pos1: BeltPos(7), pos2: BeltPos(8) })]
    #[case("mul b9 b10", Instruction::Register { op: RegOp::Mul, pos1: BeltPos(9), pos2: BeltPos(10) })]
    #[case("and b5 b6", Instruction::Register { op: RegOp::And, pos1: BeltPos(5), pos2: BeltPos(6) })]
    #[case("or b7 b8", Instruction::Register { op: RegOp::Or, pos1: BeltPos(7), pos2: BeltPos(8) })]
    #[case("xor b9 b10", Instruction::Register { op: RegOp::Xor, pos1: BeltPos(9), pos2: BeltPos(10) })]
    #[case("div b11 b12", Instruction::Register { op: RegOp::Div, pos1: BeltPos(11), pos2: BeltPos(12) })]
    #[case("save b13 b0", Instruction::Register { op: RegOp::Save, pos1: BeltPos(13), pos2: BeltPos(0) })]
    #[case("sr b14 b0", Instruction::Register { op: RegOp::ShiftRight, pos1: BeltPos(14), pos2: BeltPos(0) })]
    #[case("sl b15 b0", Instruction::Register { op: RegOp::ShiftLeft, pos1: BeltPos(15), pos2: BeltPos(0) })]
    #[case("load b3", Instruction::Unary { op: UnaryOp::Load, pos: BeltPos(3) })]
    #[case("nop", Instruction::Zero { op: ZeroOp::Nop })]
    #[case("pop", Instruction::Zero { op: ZeroOp::Pop })]
    #[case("break", Instruction::Zero { op: ZeroOp::Break })]
    pub fn test_instruction_ok(#[case] input: &str, #[case] expected: Instruction) {
        let result = instruction_parser().parse(input);
        if result.has_errors() {
            for error in result.errors() {
                println!("{}", error);
            }
            panic!("Errors found.");
        }
        let result_symbol = result.unwrap();
        match result_symbol {
            Symbol::Instruction(instruction) => assert_eq!(instruction, expected),
            _ => panic!("{:?} is not an instruction!", result_symbol),
        }
    }

    #[rstest]
    #[case("and b0 0x12345")]
    #[case("or b1 666a")]
    #[case("xor b2 0b1100a")]
    #[case("lc 0xDEF0a")]
    #[case("sl b3 4a")]
    #[case("sr b4 5a")]
    #[case("add b5 b6a")]
    #[case("sub b7 b8a")]
    #[case("mul b9 b10a")]
    #[case("and b5 b6a")]
    #[case("or b7 b8a")]
    #[case("xor b9 b10a")]
    #[case("div b11 b12a")]
    #[case("save b13 b0a")]
    #[case("sr b14 b0a")]
    #[case("sl b15 b0a")]
    #[case("load b3a")]
    pub fn test_instruction_fail(#[case] input: &str) {
        let result = instruction_parser().parse(input);
        if !result.has_errors() {
            panic!("No errors found for malformed instruction.");
        }
    }
}
