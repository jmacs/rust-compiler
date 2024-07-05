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

fn assert_token(input: &str, expect_token: Token) {
    assert_tokens(input, vec![expect_token]);
}

fn assert_tokens(input: &str, expected: Vec<Token>) {
    let actual = read_tokens(input);
    assert_eq!(expected.len(), actual.len(), "{}", {
        let actual_tokens: String = actual
            .iter()
            .map(|frame| format!("{:?}", frame.token))
            .collect::<Vec<_>>()
            .join("\n");
        let expected_tokens: String = expected
            .iter()
            .map(|token| format!("{:?}", token))
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

    for (index, expected_token) in expected.iter().enumerate() {
        let actual_token_frame = &actual[index];
        assert_eq!(
            &actual_token_frame.token, expected_token,
            "Expected {:?} to be {:?}",
            &actual_token_frame.token, expected_token
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

fn assert_span(input: &str, start: usize, end: usize) {
    let frames = read_tokens(input);
    let frame = &frames[0];
    assert_eq!(
        frame.start, start,
        "Expected '{:?}' to start at {} but was {}",
        frame.token, start, frame.start
    );
    assert_eq!(
        frame.end, end,
        "Expected '{}' ({:?}) to end at {} but was {}",
        input, frame.token, end, frame.end
    );
}

#[test]
fn test_spans() {
    assert_span("!", 0, 1);
    assert_span("+=", 0, 2);
    assert_span("literal", 0, 7);
    assert_span(r#""string""#, 0, 8);
    assert_span(" !", 1, 2);
    assert_span(" +=", 1, 3);
    assert_span(" literal", 1, 8);
    assert_span(r#" "string""#, 1, 9);
}

#[test]
fn test_illegal_char() {
    assert_token("✓", Token::Error(TokenError::Illegal('✓')));
    assert_token("☺", Token::Error(TokenError::Illegal('☺')));
    assert_token("☒", Token::Error(TokenError::Illegal('☒')));
    assert_token("“", Token::Error(TokenError::Illegal('“')));
    assert_token("”", Token::Error(TokenError::Illegal('”')));
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
    assert_token(".", Token::Dot);
    assert_token("{", Token::LBrace);
    assert_token("[", Token::LBracket);
    assert_token("(", Token::LParen);
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
fn test_char_literal() {
    assert_token(
        // basic char
        "'c'",
        Token::CharLiteral("c".to_string()),
    );
    assert_token(
        // empty char
        "''",
        Token::CharLiteral("".to_string()),
    );
    assert_token(
        // unterminated char
        "'c",
        Token::Error(TokenError::UnterminatedCharLiteral),
    );
    assert_token(
        // escaped quote
        r#"'\''"#,
        Token::CharLiteral(r#"\'"#.to_string()),
    );
}

#[test]
fn test_string_literal() {
    assert_token(
        // basic string
        r#""Make it so.""#,
        Token::StringLiteral("Make it so.".to_string()),
    );
    assert_token(
        // empty string
        r#""""#,
        Token::StringLiteral("".to_string()),
    );
    assert_token(
        // escaped double quotes
        r#""\"Make it so\"""#,
        Token::StringLiteral(r#"\"Make it so\""#.to_string()),
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
    assert_token(
        // integer
        "1234",
        Token::NumberLiteral(Number {
            kind: NumberKind::Integer,
            value: "1234".to_string(),
            postfix: None,
        }),
    );
    assert_token(
        // decimal
        "123.34",
        Token::NumberLiteral(Number {
            kind: NumberKind::Decimal,
            value: "123.34".to_string(),
            postfix: None,
        }),
    );
    assert_token(
        // Hexadecimal
        "0xFF",
        Token::NumberLiteral(Number {
            kind: NumberKind::Hexadecimal,
            value: "0xFF".to_string(),
            postfix: None,
        }),
    );
    assert_token(
        // All possible hex characters
        "0xabcdefABCDEF1234567890",
        Token::NumberLiteral(Number {
            kind: NumberKind::Hexadecimal,
            value: "0xabcdefABCDEF1234567890".to_string(),
            postfix: None,
        }),
    );
    assert_token(
        // Malformed hex
        "0x",
        Token::Error(TokenError::MalformedHexadecimal),
    );
    assert_token(
        // decimal with trailing period
        "123. ",
        Token::Error(TokenError::MalformedDecimal),
    );
    assert_token(
        // multiple decimals
        "123.456.789",
        Token::Error(TokenError::MalformedDecimal),
    );
    assert_tokens(
        // with negative prefix
        "-1234",
        vec![
            Token::Minus,
            Token::NumberLiteral(Number {
                kind: NumberKind::Integer,
                value: "1234".to_string(),
                postfix: None,
            }),
        ],
    );
    assert_tokens(
        // negative decimal
        "-12.34",
        vec![
            Token::Minus,
            Token::NumberLiteral(Number {
                kind: NumberKind::Decimal,
                value: "12.34".to_string(),
                postfix: None,
            }),
        ],
    );
    assert_token(
        // with separators
        "123_456_789",
        Token::NumberLiteral(Number {
            kind: NumberKind::Integer,
            value: "123_456_789".to_string(),
            postfix: None,
        }),
    );
    assert_token(
        // with trailing separators
        "123_",
        Token::NumberLiteral(Number {
            kind: NumberKind::Integer,
            value: "123_".to_string(),
            postfix: None,
        }),
    );
    assert_token(
        // with postfix
        "123f",
        Token::NumberLiteral(Number {
            kind: NumberKind::Integer,
            value: "123".to_string(),
            postfix: Some("f".to_string()),
        }),
    );
    assert_token(
        // with invalid postfix (parser error)
        "123abc",
        Token::NumberLiteral(Number {
            kind: NumberKind::Integer,
            value: "123".to_string(),
            postfix: Some("abc".to_string()),
        }),
    );
    assert_tokens(
        // with legal postfix character
        "123☺",
        vec![
            Token::NumberLiteral(Number {
                kind: NumberKind::Integer,
                value: "123".to_string(),
                postfix: None,
            }),
            Token::Error(TokenError::Illegal('☺')),
        ],
    );
    assert_tokens(
        // with trailing +
        "123+",
        vec![
            Token::NumberLiteral(Number {
                kind: NumberKind::Integer,
                value: "123".to_string(),
                postfix: None,
            }),
            Token::Plus,
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
    assert_token("// comment", Token::Comment("// comment".to_string()));
}
