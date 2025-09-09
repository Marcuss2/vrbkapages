use assembly_compiler::parser;
use rstest::rstest;

#[rstest]
#[case::add("add x1, x2, x3")]
#[case::sub("sub x4,x5, x6")]
#[case::and("and x7, x8,x9")]
#[case::or("or x1 , x11 , x12")]
#[case::xor("xor x19 , x20, x21")]
#[case::sll(" sll x22, x23, x24")]
#[case::srl("srl x25, x26, x27 ")]
#[case::sra("sra x28, x29, x30")]
#[case::slt("slt x31, zero, ra")]
#[case::sltu("sltu sp, gp, tp")]
#[case::addi("addi t0, t1, 5")]
#[case::addi_neg("addi t0, t1, -5")]
#[case::addi_hex("addi t2, s0, 0x123")]
#[case::slli("slli fp, s1, 10")]
#[case::slti("slti a0, a1, 12")]
#[case::sltiu("sltiu a2, a3, 15")]
#[case::xori("xori a4, a5, 18")]
#[case::ori("ori a6, a7, 21")]
#[case::andi("andi s2, s3, 24")]
#[case::srli("srli s4, s5, 27")]
#[case::srai("srai s6, s7, 30")]
#[case::sb("sb s8, 300(s9)")]
#[case::sh("sh s6, 200(s1)")]
#[case::sw("sw t3, 400(t4)")]
#[case::beq("beq x1, x2, 2")]
#[case::bne("bne x3, x4, 0x10")]
#[case::blt("blt x5, x6, -20")]
#[case::bge("bge x7, x8, 200")]
#[case::bltu("bltu t0, t1, 300")]
#[case::bgeu("bgeu s0, s1, 124")]
#[case::lui("lui x1, 5")]
#[case::lui_2("lui x2, -8")]
#[case::auipc("auipc x3, 0x1234")]
fn test_instruction(#[case] instruction: &str) {
    let parsed = parser::parse_riscv(instruction);
    if let Err(err) = parsed {
        for e in err.iter() {
            println!("{}", e);
        }
        panic!("{:?}", err);
    }
}

#[rstest]
#[case::jal("jal ra, label")]
#[case::jal_2("jal x1, 0x3456")]
fn test_pseudo_instruction(#[case] _instruction: &str) {
    // TODO
}

#[rstest]
#[case::add("addx1, x2, x3")]
#[case::swapped_save("sb x0, x1[100]")]
fn test_instruction_parse_fails(#[case] instruction: &str) {
    let parsed = parser::parse_riscv(instruction);
    assert!(parsed.is_err());
}

#[test]
fn test_parse_multiple_instructions() {
    let input = " addi x3, x4, 6\n  sub x5, x6, x7\n  ";
    let result = parser::parse_riscv(input);
    match result {
        Ok(program) => assert_eq!(program.symbols.len(), 2),
        Err(err) => {
            for e in err {
                println!("{}", e);
                assert!(false);
            }
        }
    }
}

#[test]
fn test_parse_empty_string() {
    let input = "";
    let result = parser::parse_riscv(input);
    assert!(result.is_ok());
    let symbols = result.unwrap().symbols;
    assert_eq!(symbols.len(), 0);
}

#[test]
fn test_parse_invalid_instruction() {
    let input = "invalid x1, x2, 3\n";
    let result = parser::parse_riscv(input);
    assert!(result.is_err());
}
