extern crate rust_compiler;

use rust_compiler::lexer::*;

#[test]
fn test_example() {
    let mut lexer = Lexer::new();
    lexer.read_line("");
    assert_eq!(true, true);
}