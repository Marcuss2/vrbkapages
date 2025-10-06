use chumsky::{
    Parser,
    input::{Input, Stream},
};
use logos::Logos;

use crate::belt::parser::lexer::Token;

pub mod lexer;
pub mod syntax;

pub fn parse(
    program_text: &str,
) -> Result<crate::belt::ast::Program, Vec<chumsky::prelude::Rich<'_, Token<'_>>>> {
    let token_iter = lexer::Token::lexer(program_text)
        .spanned()
        // Convert logos errors into tokens. We want parsing to be recoverable and not fail at the lexing stage, so
        // we have a dedicated `Token::Error` variant that represents a token error that was previously encountered
        .map(|(tok, span)| match tok {
            // Turn the `Range<usize>` spans logos gives us into chumsky's `SimpleSpan` via `Into`, because it's easier
            // to work with
            Ok(tok) => (tok, span.into()),
            Err(()) => (lexer::Token::Error, span.into()),
        });

    let token_stream = Stream::from_iter(token_iter)
        // Tell chumsky to split the (Token, SimpleSpan) stream into its parts so that it can handle the spans for us
        // This involves giving chumsky an 'end of input' span: we just use a zero-width span at the end of the string
        .map((0..program_text.len()).into(), |(t, s): (_, _)| (t, s));

    syntax::parser().parse(token_stream).into_result()
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::belt::ast::{
        BeltConstantOp, BeltPos, ConstantOp, ImmediateOp, Instruction, RegOp, Symbol, UnaryOp,
        ZeroOp, Alignment, Directive
    };

    #[rstest]
    #[case("and b0 0x1234", Instruction::BeltConstant { op: BeltConstantOp::And, pos: BeltPos(0), constant: 0x1234u16.into() })]
    #[case("or b1 666", Instruction::BeltConstant { op: BeltConstantOp::Or, pos: BeltPos(1), constant: 666u16.into() })]
    #[case("xor b2 0b1100", Instruction::BeltConstant { op: BeltConstantOp::Xor, pos: BeltPos(2), constant: 0b1100u16.into() })]
    #[case("lc 0xDEF0", Instruction::Constant { op: ConstantOp::LoadFromMemory, constant: 0xDEF0u16.into() })]
    #[case("sl b3 4", Instruction::Immediate { op: ImmediateOp::ShiftLeft, pos: BeltPos(3), imm: 4 })]
    #[case("sr b4 5", Instruction::Immediate { op: ImmediateOp::ShiftRight, pos: BeltPos(4), imm: 5 })]
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
    #[case("call label1", Instruction::Constant { op: ConstantOp::Call, constant: String::from("label1").into() })]
    #[case("call 0x1234", Instruction::Constant { op: ConstantOp::Call, constant: 0x1234u16.into() })]
    pub fn test_instruction_ok(#[case] input: &str, #[case] expected: Instruction) {
        println!("Instruction: \"{}\"", input);
        let result = super::parse(input);
        if let Err(errors) = result {
            for error in errors {
                println!("{}", error.reason());
            }
            panic!("Errors found.");
        } else if let Ok(program) = result {
            assert_eq!(program.symbols.len(), 1);
            match &program.symbols[0] {
                Symbol::Instruction(instruction) => assert_eq!(instruction, &expected),
                other => panic!("{:?} is not an instruction!", other),
            }
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
    #[case("div b11 b12a")]
    #[case("save b13 b0a")]
    #[case("sr b14 b0a")]
    #[case("sl b15 b0a")]
    #[case("load b3a")]
    pub fn test_instruction_fail(#[case] input: &str) {
        println!("Instruction: \"{}\"", input);
        let result = super::parse(input);
        if !result.is_err() {
            panic!(
                "No errors found for malformed instruction. Resulting parse: {:?}",
                result.unwrap()
            );
        }
    }

    #[rstest]
    #[case("label1:", Symbol::Label(String::from("label1")))]
    #[case("start:", Symbol::Label(String::from("start")))]
    #[case("my_label:", Symbol::Label(String::from("my_label")))]
    pub fn test_label_ok(#[case] input: &str, #[case] expected: Symbol) {
        println!("Label: \"{}\"", input);
        let result = super::parse(input);
        if let Err(errors) = result {
            for error in errors {
                println!("{}", error.reason());
            }
            panic!("Errors found.");
        } else if let Ok(program) = result {
            assert_eq!(program.symbols.len(), 1);
            match &program.symbols[0] {
                Symbol::Label(label) => {
                    if let Symbol::Label(expected_label) = expected {
                        assert_eq!(label, &expected_label);
                    } else {
                        panic!("Expected label, got {:?}", expected);
                    }
                }
                other => panic!("{:?} is not a label!", other),
            }
        }
    }

    #[rstest]
    #[case(".balign 4")]
    #[case(".balign 8")]
    #[case(".balign 16")]
    pub fn test_directive_ok(#[case] input: &str) {
        println!("Directive: \"{}\"", input);
        let result = super::parse(input);
        if let Err(errors) = result {
            for error in errors {
                println!("{}", error.reason());
            }
            panic!("Errors found.");
        } else if let Ok(program) = result {
            assert_eq!(program.symbols.len(), 1);
            match &program.symbols[0] {
                Symbol::Directive(_) => {}
                other => panic!("{:?} is not a directive!", other),
            }
        }
    }

    #[rstest]
    #[case(".balign 16")]
    #[case(".balign 0")]
    #[case("label1:")]
    #[case("start:")]
    pub fn test_label_directive_combined(#[case] input: &str) {
        println!("Combined: \"{}\"", input);
        let result = super::parse(input);
        if let Err(errors) = result {
            for error in errors {
                println!("{}", error.reason());
            }
            panic!("Errors found.");
        } else if let Ok(program) = result {
            assert_eq!(program.symbols.len(), 1);
        }
    }

    #[rstest]
    #[case(".balign 20")]
    #[case(".balign -1")]
    pub fn test_directive_fail(#[case] input: &str) {
        println!("Directive: \"{}\"", input);
        let result = super::parse(input);
        if !result.is_err() {
            panic!(
                "No errors found for malformed directive. Resulting parse: {:?}",
                result.unwrap()
            );
        }
    }

    #[rstest]
    #[case("label1: \n.balign 4", "label1", Directive::Alignment(Alignment { alignment: 4 }).into())]
    #[case("start: \nnop", "start", Instruction::Zero { op: ZeroOp::Nop }.into())]
    #[case("my_label: \nlc 0x1234", "my_label", Instruction::Constant { op: ConstantOp::LoadFromMemory, constant: 0x1234u16.into()}.into())]
    pub fn test_label_with_symbol(#[case] input: &str, #[case] label: &str, #[case] symbol: Symbol) {
        println!("Label with symbol: \"{}\"", input);
        let result = super::parse(input);
        if let Err(errors) = result {
            for error in errors {
                println!("{}", error.reason());
            }
            panic!("Errors found.");
        } else if let Ok(program) = result {
            assert_eq!(program.symbols.len(), 2);
            assert_eq!(&program.symbols[0], &Symbol::Label(label.into()));
            assert_eq!(&program.symbols[1], &symbol);
        }
    }
}
