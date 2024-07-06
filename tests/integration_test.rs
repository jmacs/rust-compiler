extern crate rust_compiler;

use rust_compiler::parser::Parser;
use std::{env, fs};

#[test]
fn test_parser() {
    run_test("variable_statement");
}

fn run_test(test_name: &str) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let test_dir = current_dir.join("tests").join(test_name);

    let sample_foo_path = test_dir.join("sample.foo");
    let sample_foo_path_str = sample_foo_path.to_str().unwrap();
    let program = Parser::parse_file(sample_foo_path_str)
        .expect(&format!("{} -> could not parse sample.foo", test_name));

    // todo check for parse errors

    let actual_json = serde_json::to_string_pretty(&program)
        .expect(&format!("{} -> could not serialize program", test_name));

    let expect_json_path = test_dir.join("expect.json");
    let expect_json = fs::read_to_string(expect_json_path)
        .expect(&format!("{} -> could not read expect.json", test_name));

    let are_equal = actual_json.eq(&expect_json);

    if !are_equal {
        println!("Test {} failed.\n\n{}", test_name, actual_json)
    }
    assert_eq!(actual_json, expect_json, "{}: -> test failure", test_name);
}
