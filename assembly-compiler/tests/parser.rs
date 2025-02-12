use std::error::Error;
use pest::error::ErrorVariant;
use pest::Parser;
use rstest::rstest;
use assembly_compiler::parser::{RiscvParser, Rule};

#[rstest]
#[case::add("add x1, x2, x3")]
#[case::sub("sub x4, x5, x6")]
#[case::add("add x7, x8, x9")]
#[case::or("or x1, x11, x12")]
#[case::xor("xor x19, x20, x21")]
fn test_r_type_instructions(#[case] instruction: &str) -> Result<(), Box<dyn Error>> {
    let parsed = RiscvParser::parse(Rule::program, instruction);
    if let Err(err) = parsed {
        println!("{}", err);
        panic!("{}", err);
    }
    Ok(())
}
