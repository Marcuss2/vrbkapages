use pest::error::Error;
use pest::iterators::Pairs;
use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "riscv.pest"]
pub struct RiscvParser;

pub fn parse_riscv(text: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RiscvParser::parse(Rule::program, text)
}