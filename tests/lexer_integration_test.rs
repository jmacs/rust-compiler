extern crate rust_compiler;

use rust_compiler::lexer::*;

fn read_tokens(input: &str) -> Vec<TokenResult> {
    let mut lexer = Lexer::new();
    let mut results: Vec<TokenResult> = Vec::new();
    lexer.read_line(input);

    while let Some(result) = lexer.next_token() {
        results.push(result);
    }

    results
}

fn assert_results(actual: Vec<TokenResult>, expected: Vec<TokenResult>) {
    assert_eq!(expected.len(), actual.len(), "Unexpected number of results");

    for (index, left) in expected.iter().enumerate() {
        let right = &actual[index];
        assert_eq!(
            left.token, right.token,
            "Expected {:?} to be {:?}",
            left.token, right.token
        );
        assert_eq!(
            left.position, right.position,
            "Unexpected position {:?}",
            left.token
        );
    }
}

fn assert_keyword(keyword_str: &str, keyword: Keyword) {
    let mut lexer = Lexer::new();
    lexer.read_line(keyword_str);
    let token = lexer.next_token().unwrap().token;
    assert_eq!(Token::Keyword(keyword), token);
}

#[test]
fn test_illegal_char() {
    let results = read_tokens("☺");
    assert_results(
        results,
        vec![TokenResult {
            token: Token::IllegalChar(String::from("☺")),
            position: 0,
        }],
    );
}

#[test]
fn test_self_terminating_delimiters() {
    let results = read_tokens("{}[]():;.,`");
    assert_results(
        results,
        vec![
            TokenResult {
                token: Token::LBrace,
                position: 0,
            },
            TokenResult {
                token: Token::RBrace,
                position: 1,
            },
            TokenResult {
                token: Token::LBracket,
                position: 2,
            },
            TokenResult {
                token: Token::RBracket,
                position: 3,
            },
            TokenResult {
                token: Token::LParen,
                position: 4,
            },
            TokenResult {
                token: Token::RParen,
                position: 5,
            },
            TokenResult {
                token: Token::Colon,
                position: 6,
            },
            TokenResult {
                token: Token::Semi,
                position: 7,
            },
            TokenResult {
                token: Token::Dot,
                position: 8,
            },
            TokenResult {
                token: Token::Comma,
                position: 9,
            },
            TokenResult {
                token: Token::Backtick,
                position: 10,
            },
        ],
    );
}

#[test]
fn test_single_char_operators() {
    let results = read_tokens("@&*!\\^=/><-%|+?");
    assert_results(
        results,
        vec![
            TokenResult {
                token: Token::At,
                position: 0,
            },
            TokenResult {
                token: Token::Amp,
                position: 1,
            },
            TokenResult {
                token: Token::Asterisk,
                position: 2,
            },
            TokenResult {
                token: Token::Bang,
                position: 3,
            },
            TokenResult {
                token: Token::BSlash,
                position: 4,
            },
            TokenResult {
                token: Token::Caret,
                position: 5,
            },
            TokenResult {
                token: Token::Equal,
                position: 6,
            },
            TokenResult {
                token: Token::FSlash,
                position: 7,
            },
            TokenResult {
                token: Token::GreaterThan,
                position: 8,
            },
            TokenResult {
                token: Token::LessThan,
                position: 9,
            },
            TokenResult {
                token: Token::Minus,
                position: 10,
            },
            TokenResult {
                token: Token::Percent,
                position: 11,
            },
            TokenResult {
                token: Token::Pipe,
                position: 12,
            },
            TokenResult {
                token: Token::Plus,
                position: 13,
            },
            TokenResult {
                token: Token::Question,
                position: 14,
            },
        ],
    );
}

#[test]
fn test_string_literal() {
    let results = read_tokens(r#""Make it so.""#);
    assert_results(
        results,
        vec![
            TokenResult {
                token: Token::DblQuote,
                position: 0,
            },
            TokenResult {
                token: Token::StringLiteral(String::from("Make it so.")),
                position: 1,
            },
            TokenResult {
                token: Token::DblQuote,
                position: 12,
            },
        ],
    );
}

#[test]
fn test_char_literal() {
    let results = read_tokens("'c'");
    assert_results(
        results,
        vec![
            TokenResult {
                token: Token::Quote,
                position: 0,
            },
            TokenResult {
                token: Token::CharLiteral(String::from("c")),
                position: 1,
            },
            TokenResult {
                token: Token::Quote,
                position: 2,
            },
        ],
    );
}

#[test]
fn test_empty_string_literal() {
    let results = read_tokens(r#""""#);
    assert_results(
        results,
        vec![
            TokenResult {
                token: Token::DblQuote,
                position: 0,
            },
            TokenResult {
                token: Token::DblQuote,
                position: 1,
            },
        ],
    );
}

#[test]
fn test_empty_char_literal() {
    let results = read_tokens(r#"''"#);
    assert_results(
        results,
        vec![
            TokenResult {
                token: Token::Quote,
                position: 0,
            },
            TokenResult {
                token: Token::Quote,
                position: 1,
            },
        ],
    );
}

#[test]
fn test_string_literal_with_escaped_dbl_quote() {
    let results = read_tokens(r#""\"Make it so\"""#);
    assert_results(
        results,
        vec![
            TokenResult {
                token: Token::DblQuote,
                position: 0,
            },
            TokenResult {
                token: Token::StringLiteral(String::from(r#"\"Make it so\""#)),
                position: 1,
            },
            TokenResult {
                token: Token::DblQuote,
                position: 15,
            },
        ],
    );
}

#[test]
fn test_char_literal_with_escaped_quote() {
    let results = read_tokens(r#"'\''"#);
    assert_results(
        results,
        vec![
            TokenResult {
                token: Token::Quote,
                position: 0,
            },
            TokenResult {
                token: Token::CharLiteral(String::from(r#"\'"#)),
                position: 1,
            },
            TokenResult {
                token: Token::Quote,
                position: 3,
            },
        ],
    );
}

#[test]
fn test_bool_literal_true() {
    let results = read_tokens("true");
    assert_results(
        results,
        vec![TokenResult {
            token: Token::BoolLiteral(true),
            position: 0,
        }],
    );
}

#[test]
fn test_bool_literal_false() {
    let results = read_tokens("false");
    assert_results(
        results,
        vec![TokenResult {
            token: Token::BoolLiteral(false),
            position: 0,
        }],
    );
}

#[test]
fn test_identifier() {
    let results = read_tokens("foobar");
    assert_results(
        results,
        vec![TokenResult {
            token: Token::Identifier(String::from("foobar")),
            position: 0,
        }],
    );
}

#[test]
fn test_identifier_with_numbers() {
    let results = read_tokens("foobar1");
    assert_results(
        results,
        vec![TokenResult {
            token: Token::Identifier(String::from("foobar1")),
            position: 0,
        }],
    );
}

#[test]
fn test_keywords() {
    assert_keyword("async", Keyword::ASYNC);
    assert_keyword("await", Keyword::AWAIT);
    assert_keyword("break", Keyword::BREAK);
    assert_keyword("const", Keyword::CONST);
    assert_keyword("continue", Keyword::CONTINUE);
    assert_keyword("else", Keyword::ELSE);
    assert_keyword("for", Keyword::FOR);
    assert_keyword("func", Keyword::FUNC);
    assert_keyword("if", Keyword::IF);
    assert_keyword("impl", Keyword::IMPL);
    assert_keyword("let", Keyword::LET);
    assert_keyword("match", Keyword::MATCH);
    assert_keyword("pub", Keyword::PUB);
    assert_keyword("return", Keyword::RETURN);
    assert_keyword("self", Keyword::SELF);
    assert_keyword("trait", Keyword::TRAIT);
    assert_keyword("type", Keyword::TYPE);
    assert_keyword("void", Keyword::VOID);
    assert_keyword("where", Keyword::WHERE);
    assert_keyword("while", Keyword::WHILE);
}
