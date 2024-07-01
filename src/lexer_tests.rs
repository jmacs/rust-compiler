use crate::lexer::*;
use crate::token::*;

fn read_tokens(input: &str) -> Vec<TokenFrame> {
    let mut lexer = Lexer::new();
    let mut frames: Vec<TokenFrame> = Vec::new();
    lexer.read_line(input);

    while let Some(frame) = lexer.next_token() {
        frames.push(frame);
    }

    frames
}

fn assert_all_tokens(input: &str, expected: Vec<TokenFrame>) {
    let actual = read_tokens(input);
    assert_eq!(expected.len(), actual.len(), "{}", {
        let actual_tokens: String = actual
            .iter()
            .map(|r| format!("{}: {:?}", r.position, r.token))
            .collect::<Vec<_>>()
            .join("\n");
        let expected_tokens: String = expected
            .iter()
            .map(|r| format!("{}: {:?}", r.position, r.token))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "Expected {} frames from '{}' but got {}:\n\nExpected:\n{}\n\nActual:\n{}",
            expected.len(),
            input,
            actual.len(),
            expected_tokens,
            actual_tokens,
        )
    });

    for (index, left) in expected.iter().enumerate() {
        let right = &actual[index];
        assert_eq!(
            left.token, right.token,
            "Expected {:?} to be {:?}",
            left.token, right.token
        );
        assert_eq!(
            left.position, right.position,
            "Expected token '{:?}' to be in position {} but was at {}",
            left.token, left.position, right.position,
        );
    }
}

fn assert_keyword(input: &str, keyword: Keyword) {
    let frames = read_tokens(input);
    assert_eq!(
        frames.len(),
        1,
        "Expected 1 keyword token from {} ({:?})",
        input,
        keyword
    );
    let expect = Token::Keyword(keyword);
    assert_eq!(
        expect, frames[0].token,
        "Expected {:?} to be {:?}",
        frames[0].token, expect
    );
}

fn assert_token(input: &str, expect_token: Token) {
    let frames = read_tokens(input);
    assert_eq!(frames.len(), 1, "{}", {
        let token_strings: String = frames
            .iter()
            .map(|r| format!("{}: {:?}", r.position, r.token))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "Expected 1 {:?} token from '{}' but got {} tokens:\n{}",
            expect_token,
            input,
            frames.len(),
            token_strings,
        )
    });
    assert_eq!(
        frames[0].token, expect_token,
        "Expected {:?} to be {:?}",
        expect_token, frames[0].token
    );
}

#[test]
fn test_illegal_char() {
    assert_token("✓", Token::Illegal('✓'));
    assert_token("☺", Token::Illegal('☺'));
    assert_token("☒", Token::Illegal('☒'));
    assert_token("“", Token::Illegal('“'));
    assert_token("”", Token::Illegal('”'));
}

#[test]
fn test_operators_and_delimiters() {
    assert_token("@", Token::At);
    assert_token("&", Token::Amp);
    assert_token("*", Token::Asterisk);
    assert_token("!", Token::Bang);
    assert_token(r#"\"#, Token::BSlash);
    assert_token("^", Token::Caret);
    assert_token("=", Token::Equal);
    assert_token("/", Token::FSlash);
    assert_token(">", Token::GreaterThan);
    assert_token("<", Token::LessThan);
    assert_token("-", Token::Minus);
    assert_token("%", Token::Percent);
    assert_token("|", Token::Pipe);
    assert_token("+", Token::Plus);
    assert_token("?", Token::Question);
    assert_token("`", Token::Backtick);
    assert_token(":", Token::Colon);
    assert_token(",", Token::Comma);
    assert_token(r#"""#, Token::DblQuote);
    assert_token(".", Token::Dot);
    assert_token("{", Token::LBrace);
    assert_token("[", Token::LBracket);
    assert_token("(", Token::LParen);
    assert_token("'", Token::Quote);
    assert_token("}", Token::RBrace);
    assert_token("]", Token::RBracket);
    assert_token(")", Token::RParen);
    assert_token(";", Token::Semi);
    assert_token("==", Token::EqualTo);
    assert_token("+=", Token::PlusEqual);
    assert_token("-=", Token::MinusEqual);
    assert_token("/=", Token::DivideEqual);
    assert_token(">=", Token::GreaterThanEqual);
    assert_token("<=", Token::LessThanEqual);
    assert_token("&&", Token::LogicalAnd);
    assert_token("||", Token::LogicalOr);
    assert_token("-=", Token::MinusEqual);
    assert_token("*=", Token::MultiplyEqual);
    assert_token("!=", Token::NotEqualTo);
}

#[test]
fn test_string_literal() {
    assert_all_tokens(
        r#""Make it so.""#,
        vec![
            TokenFrame {
                token: Token::DblQuote,
                line: 0,
                position: 0,
            },
            TokenFrame {
                token: Token::StringLiteral("Make it so.".to_string()),
                line: 0,
                position: 1,
            },
            TokenFrame {
                token: Token::DblQuote,
                line: 0,
                position: 12,
            },
        ],
    );
}

#[test]
fn test_char_literal() {
    assert_all_tokens(
        "'c'",
        vec![
            TokenFrame {
                token: Token::Quote,
                line: 0,
                position: 0,
            },
            TokenFrame {
                token: Token::CharLiteral("c".to_string()),
                line: 0,
                position: 1,
            },
            TokenFrame {
                token: Token::Quote,
                line: 0,
                position: 2,
            },
        ],
    );
}

#[test]
fn test_empty_string_literal() {
    assert_all_tokens(
        r#""""#,
        vec![
            TokenFrame {
                token: Token::DblQuote,
                line: 0,
                position: 0,
            },
            TokenFrame {
                token: Token::DblQuote,
                line: 0,
                position: 1,
            },
        ],
    );
}

#[test]
fn test_empty_char_literal() {
    assert_all_tokens(
        r#"''"#,
        vec![
            TokenFrame {
                token: Token::Quote,
                line: 0,
                position: 0,
            },
            TokenFrame {
                token: Token::Quote,
                line: 0,
                position: 1,
            },
        ],
    );
}

#[test]
fn test_unterminated_char_literal() {
    assert_all_tokens(
        "'c",
        vec![
            TokenFrame {
                token: Token::Quote,
                line: 0,
                position: 0,
            },
            TokenFrame {
                token: Token::CharLiteral("c".to_string()),
                line: 0,
                position: 1,
            },
        ],
    );
}

#[test]
fn test_string_literal_with_escaped_dbl_quote() {
    assert_all_tokens(
        r#""\"Make it so\"""#,
        vec![
            TokenFrame {
                token: Token::DblQuote,
                line: 0,
                position: 0,
            },
            TokenFrame {
                token: Token::StringLiteral(r#"\"Make it so\""#.to_string()),
                line: 0,
                position: 1,
            },
            TokenFrame {
                token: Token::DblQuote,
                line: 0,
                position: 15,
            },
        ],
    );
}

#[test]
fn test_unterminated_string_literal() {
    assert_all_tokens(
        r#""Make it so"#,
        vec![
            TokenFrame {
                token: Token::DblQuote,
                line: 0,
                position: 0,
            },
            TokenFrame {
                token: Token::StringLiteral(r#"Make it so"#.to_string()),
                line: 0,
                position: 1,
            },
        ],
    );
}

#[test]
fn test_char_literal_with_escaped_quote() {
    assert_all_tokens(
        r#"'\''"#,
        vec![
            TokenFrame {
                token: Token::Quote,
                line: 0,
                position: 0,
            },
            TokenFrame {
                token: Token::CharLiteral(r#"\'"#.to_string()),
                line: 0,
                position: 1,
            },
            TokenFrame {
                token: Token::Quote,
                line: 0,
                position: 3,
            },
        ],
    );
}

#[test]
fn test_boolean_literals() {
    assert_token("false", Token::BoolLiteral(false));
    assert_token("true", Token::BoolLiteral(true));
}

#[test]
fn test_identifiers() {
    assert_token(
        // just letters
        "foobar",
        Token::Identifier("foobar".to_string()),
    );
    assert_token(
        // with trailing numbers
        "foobar123",
        Token::Identifier("foobar123".to_string()),
    );
    assert_token(
        // with trailing numbers
        "foobar123",
        Token::Identifier("foobar123".to_string()),
    );
    assert_token(
        // with underscores
        "foo_bar_123",
        Token::Identifier("foo_bar_123".to_string()),
    );
    assert_token(
        // with leading underscore
        "_foobar",
        Token::Identifier("_foobar".to_string()),
    );
    assert_token(
        // with trailing underscore
        "foobar_",
        Token::Identifier("foobar_".to_string()),
    );
    assert_token(
        // just underscore
        "_",
        Token::Identifier("_".to_string()),
    );
}

#[test]
fn test_number_literals() {
    assert_all_tokens(
        // integer
        "1234",
        vec![TokenFrame {
            position: 0,
            line: 0,
            token: Token::NumberLiteral(Number {
                kind: NumberKind::Integer,
                value: "1234".to_string(),
                postfix: None,
            }),
        }],
    );
    assert_all_tokens(
        // decimal
        "123.34",
        vec![TokenFrame {
            position: 0,
            line: 0,
            token: Token::NumberLiteral(Number {
                kind: NumberKind::Decimal,
                value: "123.34".to_string(),
                postfix: None,
            }),
        }],
    );
    assert_all_tokens(
        // Hexadecimal
        "0xFF",
        vec![TokenFrame {
            position: 0,
            line: 0,
            token: Token::NumberLiteral(Number {
                kind: NumberKind::Hexadecimal,
                value: "0xFF".to_string(),
                postfix: None,
            }),
        }],
    );
    assert_all_tokens(
        // Hexadecimal (mixed numbers and letters)
        "0xabcdefABCDEF1234567890",
        vec![TokenFrame {
            position: 0,
            line: 0,
            token: Token::NumberLiteral(Number {
                kind: NumberKind::Hexadecimal,
                value: "0xabcdefABCDEF1234567890".to_string(),
                postfix: None,
            }),
        }],
    );
    assert_all_tokens(
        // Malformed hexadecimal
        "0x",
        vec![TokenFrame {
            position: 0,
            line: 0,
            token: Token::NumberLiteral(Number {
                kind: NumberKind::Hexadecimal,
                value: "0x".to_string(),
                postfix: None,
            }),
        }],
    );
    assert_all_tokens(
        // decimal with trailing period
        "123. ",
        vec![
            TokenFrame {
                position: 0,
                line: 0,
                token: Token::NumberLiteral(Number {
                    kind: NumberKind::Integer,
                    value: "123".to_string(),
                    postfix: None,
                }),
            },
            TokenFrame {
                position: 3,
                line: 0,
                token: Token::Dot,
            },
        ],
    );
    assert_all_tokens(
        // negative integer
        "-1234",
        vec![TokenFrame {
            position: 0,
            line: 0,
            token: Token::NumberLiteral(Number {
                kind: NumberKind::Integer,
                value: "-1234".to_string(),
                postfix: None,
            }),
        }],
    );
    assert_all_tokens(
        // negative decimal
        "-12.34",
        vec![TokenFrame {
            position: 0,
            line: 0,
            token: Token::NumberLiteral(Number {
                kind: NumberKind::Decimal,
                value: "-12.34".to_string(),
                postfix: None,
            }),
        }],
    );
    assert_all_tokens(
        // with separators
        "123_456_789",
        vec![TokenFrame {
            position: 0,
            line: 0,
            token: Token::NumberLiteral(Number {
                kind: NumberKind::Integer,
                value: "123_456_789".to_string(),
                postfix: None,
            }),
        }],
    );
    assert_all_tokens(
        // with trailing separators
        "123_",
        vec![TokenFrame {
            position: 0,
            line: 0,
            token: Token::NumberLiteral(Number {
                kind: NumberKind::Integer,
                value: "123_".to_string(),
                postfix: None,
            }),
        }],
    );
    assert_all_tokens(
        // with postfix
        "123abc",
        vec![TokenFrame {
            position: 0,
            line: 0,
            token: Token::NumberLiteral(Number {
                kind: NumberKind::Integer,
                value: "123".to_string(),
                postfix: Some("abc".to_string()),
            }),
        }],
    );
    assert_all_tokens(
        // with legal postfix character
        "123☺",
        vec![
            TokenFrame {
                position: 0,
                line: 0,
                token: Token::NumberLiteral(Number {
                    kind: NumberKind::Integer,
                    value: "123".to_string(),
                    postfix: None,
                }),
            },
            TokenFrame {
                token: Token::Illegal('☺'),
                position: 3,
                line: 0,
            },
        ],
    );
    assert_all_tokens(
        // with trailing +
        "123+",
        vec![
            TokenFrame {
                position: 0,
                line: 0,
                token: Token::NumberLiteral(Number {
                    kind: NumberKind::Integer,
                    value: "123".to_string(),
                    postfix: None,
                }),
            },
            TokenFrame {
                token: Token::Plus,
                line: 0,
                position: 3,
            },
        ],
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
    assert_keyword("use", Keyword::USE);
    assert_keyword("void", Keyword::VOID);
    assert_keyword("where", Keyword::WHERE);
    assert_keyword("while", Keyword::WHILE);
}

#[test]
fn test_comment() {
    assert_all_tokens(
        "// comment",
        vec![TokenFrame {
            token: Token::Comment("// comment".to_string()),
            line: 0,
            position: 0,
        }],
    );
}
