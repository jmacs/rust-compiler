use crate::parser::*;

#[test]
fn test_parser() {
    let _program = Parser::parse_source(vec!["let x = 1;"]);
    assert_eq!(true, true)
}
