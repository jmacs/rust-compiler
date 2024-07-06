extern crate rust_compiler;

use rust_compiler::parser::Parser;
use std::{env, fs};

fn test(test_name: &str) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let test_dir = current_dir.join("tests").join(test_name);

    let sample_foo_path = test_dir.join("sample.foo");
    let sample_foo_path_str = sample_foo_path.to_str().unwrap();
    let program = Parser::parse_file(sample_foo_path_str)
        .expect(&format!("{} -> could not parse sample.foo", test_name));

    let actual_json = serde_json::to_string_pretty(&program)
        .expect(&format!("{} -> could not serialize program", test_name));

    let expect_json_path = test_dir.join("expect.json");
    let expect_json = fs::read_to_string(expect_json_path)
        .expect(&format!("{} -> could not read expect.json", test_name));

    let parse_errors = program
        .errors
        .iter()
        .map(|parse_error| format!("- {:?}", parse_error))
        .collect::<Vec<_>>()
        .join("\n");

    assert_eq!(
        program.errors.len(),
        0,
        "{} parse error:\n{}",
        test_name,
        parse_errors
    );

    assert_eq!(
        actual_json, expect_json,
        "{} program output does not match expected result.\n{}",
        test_name, actual_json
    );
}

#[test]
fn test_variable_statement() {
    test("variable_statement");
}
