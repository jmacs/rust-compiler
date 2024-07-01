use crate::parser::*;

#[test]
fn test_parser() {
    let program = Parser::parse_source(vec!["let x = 1;"]);
    println!("\n{:?}", program);
    assert_eq!(program.statements.len(), 1);
}
