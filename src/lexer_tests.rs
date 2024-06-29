use crate::lexer::*;

#[allow(dead_code)]
fn read_tokens(input: &str) -> Vec<TokenResult> {
    let mut lexer = Lexer::new();
    let mut results: Vec<TokenResult> = Vec::new();
    lexer.read_line(input);

    while let Some(result) = lexer.next_token() {
        results.push(result);
    }

    results
}

#[allow(dead_code)]
fn assert_all_tokens(input: &str, expected: Vec<TokenResult>) {
    let actual = read_tokens(input);
    assert_eq!(
        expected.len(),
        actual.len(),
        "Expected {} results but got {}",
        expected.len(),
        actual.len()
    );

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

#[allow(dead_code)]
fn assert_keyword(input: &str, keyword: Keyword) {
    let results = read_tokens(input);
    assert_eq!(
        results.len(),
        1,
        "Expected 1 keyword token from {} ({:?})",
        input,
        keyword
    );
    let expect = Token::Keyword(keyword);
    assert_eq!(
        expect, results[0].token,
        "Expected {:?} to be {:?}",
        results[0].token, expect
    );
}

#[allow(dead_code)]
fn assert_token(input: &str, expect_token: Token) {
    let results = read_tokens(input);
    assert_eq!(results.len(), 1, "{}", {
        let token_strings: String = results
            .iter()
            .map(|r| format!("{}: {:?}", r.position, r.token))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "Expected 1 {:?} token from '{}' but got {} tokens:\n{}",
            expect_token,
            input,
            results.len(),
            token_strings,
        )
    });
    assert_eq!(
        results[0].token, expect_token,
        "Expected {:?} to be {:?}",
        expect_token, results[0].token
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
    assert_all_tokens(
        "'c'",
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
    assert_all_tokens(
        r#""""#,
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
    assert_all_tokens(
        r#"''"#,
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
fn test_unterminated_char_literal() {
    assert_all_tokens(
        "'c",
        vec![
            TokenResult {
                token: Token::Quote,
                position: 0,
            },
            TokenResult {
                token: Token::CharLiteral(String::from("c")),
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
fn test_unterminated_string_literal() {
    assert_all_tokens(
        r#""Make it so"#,
        vec![
            TokenResult {
                token: Token::DblQuote,
                position: 0,
            },
            TokenResult {
                token: Token::StringLiteral(String::from(r#"Make it so"#)),
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
fn test_boolean_literals() {
    assert_token("false", Token::BoolLiteral(false));
    assert_token("true", Token::BoolLiteral(true));
}

#[test]
fn test_identifiers() {
    assert_token(
        // just letters
        "foobar",
        Token::Identifier(String::from("foobar")),
    );
    assert_token(
        // with trailing numbers
        "foobar123",
        Token::Identifier(String::from("foobar123")),
    );
    assert_token(
        // with trailing numbers
        "foobar123",
        Token::Identifier(String::from("foobar123")),
    );
    assert_token(
        // with underscores
        "foo_bar_123",
        Token::Identifier(String::from("foo_bar_123")),
    );
    assert_token(
        // with leading underscore
        "_foobar",
        Token::Identifier(String::from("_foobar")),
    );
    assert_token(
        // with trailing underscore
        "foobar_",
        Token::Identifier(String::from("foobar_")),
    );
    assert_token(
        // just underscore
        "_",
        Token::Identifier(String::from("_")),
    );
    assert_token(
        // with dollar sign
        "foo$bar$123",
        Token::Identifier(String::from("foo$bar$123")),
    );
    assert_token(
        // with leading dollar sign
        "$foobar",
        Token::Identifier(String::from("$foobar")),
    );
    assert_token(
        // with trailing dollar sign
        "foobar$",
        Token::Identifier(String::from("foobar$")),
    );
    assert_token(
        // just dollar sign
        "$",
        Token::Identifier(String::from("$")),
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
fn test_number_literals() {
    assert_token(
        //
        "1234",
        Token::NumberLiteral(String::from("1234")),
    );
    assert_token(
        //
        "12.34",
        Token::NumberLiteral(String::from("12.34")),
    );
    assert_token(
        //
        "12.",
        Token::NumberLiteral(String::from("12.")),
    );
    assert_token(
        //
        "123_456_789",
        Token::NumberLiteral(String::from("123_456_789")),
    );
    assert_token(
        //
        "123_",
        Token::NumberLiteral(String::from("123_")),
    );
    assert_token(
        //
        "123abc",
        Token::NumberLiteral(String::from("123abc")),
    );
}

#[test]
fn test_comment() {
    assert_all_tokens(
        "// comment",
        vec![TokenResult {
            token: Token::Comment(String::from("// comment")),
            position: 0,
        }],
    );
}
