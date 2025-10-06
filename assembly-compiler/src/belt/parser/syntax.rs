use std::fmt::Display;
use std::ops::RangeInclusive;

use crate::belt::ast::{
    Alignment, BeltConstantOp, BeltPos, ConstantOp, ConstantOrLabel, Directive, ImmediateOp,
    Instruction, Program, RegOp, Symbol, UnaryOp, ZeroOp,
};
use crate::belt::parser::lexer::Token;
use crate::chumsky_utils::number;

use chumsky::input::ValueInput;
use chumsky::prelude::*;
use num_traits::Num;
use num_traits::bounds::LowerBounded;

fn belt_pos<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, BeltPos, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    select! {
        Token::Register(r) => r,
    }
    .try_map(|reg_str, span| {
        let num_str = reg_str
            .strip_prefix("b")
            .expect("BUG: lexer should have parsed this register part correctly");
        if let Ok(num) = num_str.parse::<u8>()
            && num <= 15
        {
            Ok(BeltPos(num))
        } else {
            Err(Rich::custom(span, "Belt number too large, maximum is 15"))
        }
    })
}

fn unsigned<'tokens, 'src: 'tokens, NUM, I>(
    range: &RangeInclusive<NUM>,
) -> impl Parser<'tokens, I, NUM, extra::Err<Rich<'tokens, Token<'src>>>>
where
    NUM: Num + Display + PartialOrd + LowerBounded,
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    select! {
        Token::Number(num) => num,
    }
    .try_map(|num_str, span| {
        let res = number::<NUM>(range).parse(num_str);
        if let Some(output) = res.into_output() {
            Ok(output)
        } else {
            Err(Rich::custom(
                span,
                format!(
                    "Invalid number, expected one in range from {} to {}",
                    range.start(),
                    range.end()
                ),
            ))
        }
    })
}

fn zero_instr<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Instruction, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    let zero_instr_select = select! {
        Token::Text(s) if s == "nop" => ZeroOp::Nop,
        Token::Text(s) if s == "pop" => ZeroOp::Pop,
        Token::Text(s) if s == "break" => ZeroOp::Break,
    };

    zero_instr_select
        .labelled("instruction")
        .map(|op| Instruction::Zero { op })
}

fn belt_constant_instr<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Instruction, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    let belt_constant_instr_select = select! {
        Token::Text(s) if s == "and" => BeltConstantOp::And,
        Token::Text(s) if s == "or" => BeltConstantOp::Or,
        Token::Text(s) if s == "xor" => BeltConstantOp::Xor,
    };

    let get_text = select! {
        Token::Text(s) => s,
    };

    belt_constant_instr_select
        .labelled("instruction")
        .then(belt_pos())
        .then(choice((
            unsigned(&(u16::MIN..=u16::MAX))
                .labelled("constant")
                .map(ConstantOrLabel::from),
            get_text.map(String::from).map(ConstantOrLabel::from),
        )))
        .map(
            |((operation, belt_pos), constant)| Instruction::BeltConstant {
                op: operation,
                pos: belt_pos,
                constant,
            },
        )
}

fn constant_instr<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Instruction, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    let constant_instr_select = select! {
        Token::Text(s) if s == "lc" => ConstantOp::LoadFromMemory,
        Token::Text(s) if s == "call" => ConstantOp::Call,
        Token::Text(s) if s == "jump" => ConstantOp::Jump,
    };
    let get_text = select! {
        Token::Text(s) => s,
    };
    constant_instr_select
        .labelled("instruction")
        .then(choice((
            unsigned(&(u16::MIN..=u16::MAX))
                .labelled("constant")
                .map(ConstantOrLabel::from),
            get_text.map(String::from).map(ConstantOrLabel::from),
        )))
        .map(|(op, constant)| Instruction::Constant { op, constant })
}

fn imm_instr<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Instruction, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    let imm_instr_select = select! {
        Token::Text(s) if s == "sl" => ImmediateOp::ShiftLeft,
        Token::Text(s) if s == "sr" => ImmediateOp::ShiftRight,
        Token::Text(s) if s == "ret" => ImmediateOp::Ret,
    };

    imm_instr_select
        .labelled("instruction")
        .then(belt_pos())
        .then(unsigned(&(0u8..=15u8)).labelled("constant"))
        .map(|((op, belt_pos), imm)| Instruction::Immediate {
            op,
            pos: belt_pos,
            imm: imm,
        })
}

fn reg_instr<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Instruction, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    let reg_instr_select = select! {
        Token::Text(s) if s == "add" => RegOp::Add,
        Token::Text(s) if s == "sub" => RegOp::Sub,
        Token::Text(s) if s == "and" => RegOp::And,
        Token::Text(s) if s == "or" => RegOp::Or,
        Token::Text(s) if s == "xor" => RegOp::Xor,
        Token::Text(s) if s == "mul" => RegOp::Mul,
        Token::Text(s) if s == "div" => RegOp::Div,
        Token::Text(s) if s == "save" => RegOp::Save,
        Token::Text(s) if s == "sr" => RegOp::ShiftRight,
        Token::Text(s) if s == "sl" => RegOp::ShiftLeft,
    };

    reg_instr_select
        .labelled("instruction")
        .then(belt_pos())
        .then(belt_pos())
        .map(|((op, pos1), pos2)| Instruction::Register { op, pos1, pos2 })
}

fn unary_instr<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Instruction, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    let unary_instr_select = select! {
        Token::Text(s) if s == "load" => UnaryOp::Load,
        Token::Text(s) if s == "push" => UnaryOp::Push,
        Token::Text(s) if s == "jump" => UnaryOp::Jump,
    };
    unary_instr_select
        .labelled("instruction")
        .then(belt_pos())
        .map(|(op, src)| Instruction::Unary { op, pos: src })
}

fn instruction_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Instruction, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    choice((
        belt_constant_instr(),
        constant_instr(),
        imm_instr(),
        unary_instr(),
        reg_instr(),
        zero_instr(),
    ))
}

fn label_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, String, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    select! {
        Token::Label(s) => s.strip_suffix(":").expect("Tokenizer should have kept the ':'")
    }
    .map(String::from)
}

fn directive_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Directive, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    just(Token::Directive(".balign"))
        .ignore_then(unsigned(&(0usize..=16usize)))
        .map(|alignment| Alignment { alignment })
        .map(Directive::from)
}

pub fn parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Program, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    choice((
        instruction_parser().map(Symbol::from),
        label_parser().map(Symbol::from),
        directive_parser().map(Symbol::from),
    ))
    .separated_by(just(Token::NewLine))
    .collect()
    .map(|symbols| Program { symbols })
}
