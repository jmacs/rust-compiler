const NULL_CHAR: char = '\0';
const ESCAPE_CHAR: char = '\\';
const QUOTE: char = '\'';
const DBL_QUOTE: char = '"';

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Illegal(char),
    Identifier(String),
    Keyword(Keyword),
    MultilineComment(String),
    Comment(String),

    //
    // Literals
    //
    BoolLiteral(bool),
    CharLiteral(String),
    NumberLiteral(Number),
    StringLiteral(String),
    TemplateLiteral(String),

    //
    // Delimiters + Operators
    //
    Amp,              // &
    Asterisk,         // *
    At,               // @
    BSlash,           // \
    Backtick,         // `
    Bang,             // !
    Caret,            // ^
    Colon,            // :
    Comma,            // ,
    DblQuote,         // "
    DivideEqual,      // /=
    Dot,              // .
    Equal,            // =
    EqualTo,          // ==
    FSlash,           // /
    GreaterThan,      // >
    GreaterThanEqual, // >=
    LBrace,           // {
    LBracket,         // [
    LParen,           // (
    LessThan,         // <
    LessThanEqual,    // <=
    LogicalAnd,       // &&
    LogicalOr,        // ||
    Minus,            // -
    MinusEqual,       // -=
    MultiplyEqual,    // *=
    NotEqualTo,       // !=
    Percent,          // %
    Pipe,             // |
    Plus,             // +
    PlusEqual,        // +=
    Question,         // ?
    Quote,            // '
    RBrace,           // }
    RBracket,         // ]
    RParen,           // )
    Semi,             // ;
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    AS,
    ASYNC,
    AWAIT,
    BREAK,
    CONST,
    CONTINUE,
    ELSE,
    FOR,
    FUNC,
    IF,
    IMPL,
    LET,
    MATCH,
    PUB,
    RETURN,
    SELF,
    TRAIT,
    TYPE,
    USE,
    VOID,
    WHERE,
    WHILE,
}

#[derive(Debug, PartialEq)]
pub enum NumberKind {
    Integer,
    Decimal,
    Hexadecimal,
}

#[derive(Debug, PartialEq)]
pub struct Number {
    pub kind: NumberKind,
    pub value: String,
    pub postfix: Option<String>,
}

pub struct TokenResult {
    pub token: Token,
    pub position: usize,
}

#[derive(Debug, PartialEq)]
pub enum ReadMode {
    Default,
    StringLiteral,
    CharLiteral,
    MultilineComment,
    MultilineString,
}

pub struct Lexer {
    read_mode: ReadMode,
    line: Vec<char>,
    character: char,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            line: Vec::new(),
            read_mode: ReadMode::Default,
            position: 0,
            read_position: 0,
            character: NULL_CHAR,
        }
    }

    pub fn read_line(&mut self, input: &str) {
        self.line.clear();
        self.line.extend(input.chars());
        self.position = 0;
        self.read_position = 0;
        self.next_char();
        self.reset_read_mode_on_newline();
    }

    pub fn next_token(&mut self) -> Option<TokenResult> {
        let position = self.position;
        if self.character == NULL_CHAR {
            return None;
        }
        match self.read_mode {
            ReadMode::Default => {
                self.skip_whitespace();
                let token = self.read_token();
                if token != Token::EOF {
                    Some(TokenResult { token, position })
                } else {
                    None
                }
            }
            ReadMode::StringLiteral => {
                let token = self.read_string_literal();
                Some(TokenResult { token, position })
            }
            ReadMode::CharLiteral => {
                let token = self.read_char_literal();
                Some(TokenResult { token, position })
            }
            ReadMode::MultilineComment => {
                None // TODO
            }
            ReadMode::MultilineString => {
                None // TODO
            }
        }
    }

    fn read_token(&mut self) -> Token {
        let current = self.character;
        let lookahead = self.peek_char();
        let token = match current {
            NULL_CHAR => Token::EOF,
            '=' => match lookahead {
                '=' => {
                    self.next_char();
                    Token::EqualTo
                }
                _ => Token::Equal,
            },
            '+' => match lookahead {
                '=' => {
                    self.next_char();
                    Token::PlusEqual
                }
                _ => Token::Plus,
            },
            '-' => match lookahead {
                '=' => {
                    self.next_char();
                    Token::MinusEqual
                }
                _ if is_digit(lookahead) => return self.read_number_literal(),
                _ => Token::Minus,
            },
            '!' => match lookahead {
                '=' => {
                    self.next_char();
                    Token::NotEqualTo
                }
                _ => Token::Bang,
            },
            '/' => match lookahead {
                '=' => {
                    self.next_char();
                    Token::DivideEqual
                }
                '/' => {
                    return self.read_single_line_comment();
                }
                _ => Token::FSlash,
            },
            '\\' => Token::BSlash,
            '*' => match lookahead {
                '=' => {
                    self.next_char();
                    Token::MultiplyEqual
                }
                _ => Token::Asterisk,
            },
            '&' => match lookahead {
                '&' => {
                    self.next_char();
                    Token::LogicalAnd
                }
                _ => Token::Amp,
            },
            '%' => Token::Percent,
            '@' => Token::At,
            '|' => match lookahead {
                '|' => {
                    self.next_char();
                    Token::LogicalOr
                }
                _ => Token::Pipe,
            },
            '?' => Token::Question,
            '^' => Token::Caret,
            '<' => match lookahead {
                '=' => {
                    self.next_char();
                    Token::LessThanEqual
                }
                _ => Token::LessThan,
            },
            '>' => match lookahead {
                '=' => {
                    self.next_char();
                    Token::GreaterThanEqual
                }
                _ => Token::GreaterThan,
            },
            '.' => Token::Dot,
            ';' => Token::Semi,
            ':' => Token::Colon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '`' => Token::Backtick,
            QUOTE => return self.read_char_literal(),
            DBL_QUOTE => return self.read_string_literal(),
            //
            // Else
            //
            ch => {
                return match ch {
                    _ if is_digit(ch) => self.read_number_literal(),
                    _ if is_alpha(ch) => self.read_alpha_literal(),
                    _ => {
                        self.next_char();
                        Token::Illegal(ch)
                    }
                }
            }
        };

        self.next_char();
        token
    }

    fn read_string_literal(&mut self) -> Token {
        if self.character == DBL_QUOTE {
            self.next_char();
            self.toggle_read_mode(ReadMode::StringLiteral);
            return Token::DblQuote;
        }
        let start_pos = self.position;
        if !self.read_until_char(DBL_QUOTE) {
            self.read_mode = ReadMode::Default;
        }
        let literal = self.slice_line(start_pos, self.position);
        Token::StringLiteral(literal)
    }

    fn read_char_literal(&mut self) -> Token {
        if self.character == QUOTE {
            self.next_char();
            self.toggle_read_mode(ReadMode::CharLiteral);
            return Token::Quote;
        }
        let start_pos = self.position;
        if !self.read_until_char(QUOTE) {
            self.read_mode = ReadMode::Default;
        }
        let literal = self.slice_line(start_pos, self.position);
        Token::CharLiteral(literal)
    }

    fn read_number_literal(&mut self) -> Token {
        let mut number_kind = NumberKind::Integer;
        let start_pos = self.position;
        let lookahead = self.peek_char();
        if self.character == '0' && (lookahead == 'X' || lookahead == 'x') {
            self.next_char();
            while self.character != NULL_CHAR {
                self.next_char();
                if !is_hex(self.character) {
                    break;
                }
            }
            let literal = self.slice_line(start_pos, self.position);
            return Token::NumberLiteral(Number {
                kind: NumberKind::Hexadecimal,
                value: literal,
                postfix: None,
            });
        }
        while self.character != NULL_CHAR {
            self.next_char();
            if !is_valid_number_literal_char(self.character) {
                break;
            }
        }
        if self.character == '.' && is_digit(self.peek_char()) {
            number_kind = NumberKind::Decimal;
            self.next_char();
            while self.character != NULL_CHAR {
                self.next_char();
                if !is_valid_number_literal_char(self.character) {
                    break;
                }
            }
        }
        let mut postfix: Option<String> = None;
        let literal = self.slice_line(start_pos, self.position);
        if is_alpha(self.character) {
            let postfix_start = self.position;
            while self.character != NULL_CHAR {
                self.next_char();
                if !is_alpha(self.character) {
                    break;
                }
            }
            let postfix_value = self.slice_line(postfix_start, self.position);
            postfix = Some(postfix_value);
        }
        Token::NumberLiteral(Number {
            kind: number_kind,
            value: literal,
            postfix,
        })
    }

    fn read_alpha_literal(&mut self) -> Token {
        let start_pos = self.position;
        while self.character != NULL_CHAR {
            self.next_char();
            if !is_valid_ident_literal_char(self.character) {
                break;
            }
        }
        let literal = self.slice_line(start_pos, self.read_position);
        match literal.as_str() {
            "true" => Token::BoolLiteral(true),
            "false" => Token::BoolLiteral(false),
            "as" => Token::Keyword(Keyword::AS),
            "async" => Token::Keyword(Keyword::ASYNC),
            "await" => Token::Keyword(Keyword::AWAIT),
            "break" => Token::Keyword(Keyword::BREAK),
            "const" => Token::Keyword(Keyword::CONST),
            "continue" => Token::Keyword(Keyword::CONTINUE),
            "else" => Token::Keyword(Keyword::ELSE),
            "for" => Token::Keyword(Keyword::FOR),
            "func" => Token::Keyword(Keyword::FUNC),
            "if" => Token::Keyword(Keyword::IF),
            "impl" => Token::Keyword(Keyword::IMPL),
            "let" => Token::Keyword(Keyword::LET),
            "match" => Token::Keyword(Keyword::MATCH),
            "pub" => Token::Keyword(Keyword::PUB),
            "return" => Token::Keyword(Keyword::RETURN),
            "self" => Token::Keyword(Keyword::SELF),
            "trait" => Token::Keyword(Keyword::TRAIT),
            "type" => Token::Keyword(Keyword::TYPE),
            "use" => Token::Keyword(Keyword::USE),
            "void" => Token::Keyword(Keyword::VOID),
            "where" => Token::Keyword(Keyword::WHERE),
            "while" => Token::Keyword(Keyword::WHILE),
            _ => Token::Identifier(literal),
        }
    }

    fn read_single_line_comment(&mut self) -> Token {
        let start_pos = self.position;
        self.read_until_end();
        let literal = self.slice_line(start_pos, self.read_position);
        Token::Comment(literal)
    }

    fn slice_line(&self, start_pos: usize, end_pos: usize) -> String {
        let end_pos_min = end_pos.min(self.line.len());
        let sliced_chars = &self.line[start_pos..end_pos_min];
        sliced_chars.iter().collect()
    }

    fn toggle_read_mode(&mut self, mode: ReadMode) {
        return if self.read_mode == ReadMode::Default {
            self.read_mode = mode;
        } else {
            self.read_mode = ReadMode::Default;
        };
    }

    fn read_until_char(&mut self, stop_char: char) -> bool {
        let mut last_char: char;
        while self.character != NULL_CHAR {
            last_char = self.character;
            self.next_char();
            if self.character == stop_char && last_char != ESCAPE_CHAR {
                return true;
            }
        }
        return false;
    }

    fn read_until_end(&mut self) {
        while self.character != NULL_CHAR {
            self.next_char();
        }
    }

    fn next_char(&mut self) {
        if self.read_position < self.line.len() {
            let ch = self.line[self.read_position];
            self.position = self.read_position;
            self.read_position += 1;
            self.character = ch;
        } else {
            self.position = self.read_position;
            self.character = NULL_CHAR;
        }
    }

    fn peek_char(&mut self) -> char {
        if self.read_position < self.line.len() {
            self.line[self.read_position]
        } else {
            NULL_CHAR
        }
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.character) {
            self.next_char();
            if self.character == NULL_CHAR {
                break;
            }
        }
    }

    fn reset_read_mode_on_newline(&mut self) {
        // If the last line did not properly terminate a string or character,
        // calling this function on a new line ensures that subsequent lines are lexed correctly.
        if self.read_mode == ReadMode::StringLiteral || self.read_mode == ReadMode::CharLiteral {
            self.read_mode = ReadMode::Default;
        }
    }
}

fn is_whitespace(ch: char) -> bool {
    if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
        return true;
    }
    return false;
}

fn is_alpha(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_hex(ch: char) -> bool {
    is_digit(ch) || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F')
}

fn is_valid_ident_literal_char(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}

fn is_valid_number_literal_char(ch: char) -> bool {
    is_digit(ch) || ch == '_'
}
