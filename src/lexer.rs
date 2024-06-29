const NULL_CHAR: char = '\0';
const ESCAPE_CHAR: char = '\\';
const QUOTE: char = '\'';
const DBL_QUOTE: char = '"';

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    IllegalChar(String),
    Identifier(String),
    Keyword(Keyword),
    StringLiteral(String),
    NumberLiteral(String),
    CharLiteral(String),
    TemplateLiteral,
    BoolLiteral(bool),
    MultilineComment,
    Comment,

    //
    // Delimiter
    //
    Backtick, // `
    Colon,    // :
    Comma,    // ,
    DblQuote, // "
    Dot,      // .
    LBrace,   // {
    LBracket, // [
    LParen,   // (
    Quote,    // '
    RBrace,   // }
    RBracket, // ]
    RParen,   // )
    Semi,     // ;

    //
    // Operator
    //
    At,               // @
    Amp,              // &
    Asterisk,         // *
    Bang,             // !
    BSlash,           // \
    Caret,            // ^
    Equal,            // =
    EqualTo,          // ==
    FSlash,           // /
    DivideEqual,      // /=
    GreaterThan,      // >
    GreaterThanEqual, // >=
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
    IMPORT,
    LET,
    MATCH,
    PUB,
    RETURN,
    SELF,
    TRAIT,
    TYPE,
    VOID,
    WHERE,
    WHILE,
}

pub struct TokenResult {
    pub token: Token,
    pub position: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadMode {
    Default,
    StringLiteral,
    CharLiteral,
    MultilineComment,
    MultilineString,
}

pub struct Lexer {
    mode: ReadMode,
    line: Vec<char>,
    character: char,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            mode: ReadMode::Default,
            line: Vec::new(),
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
    }

    pub fn next_token(&mut self) -> Option<TokenResult> {
        match self.mode {
            ReadMode::Default => {
                self.skip_whitespace();
                let position = self.position;
                let token = self.read_token();
                if token != Token::EOF {
                    Some(TokenResult { token, position })
                } else {
                    None
                }
            }
            ReadMode::StringLiteral => {
                let position = self.position;
                let token = self.read_string_literal();
                Some(TokenResult { token, position })
            }
            ReadMode::CharLiteral => {
                let position = self.position;
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

    fn next_char(&mut self) {
        if self.read_position < self.line.len() {
            let ch = self.line[self.read_position];
            self.position = self.read_position;
            self.read_position += 1;
            self.character = ch;
        } else {
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
            // Other
            //
            ch => {
                if is_letter(ch) {
                    return self.read_letter_literal();
                }
                if is_digit(ch) {
                    return self.read_number_literal();
                }
                Token::IllegalChar(String::from(ch))
            }
        };

        self.next_char();
        token
    }

    fn read_string_literal(&mut self) -> Token {
        if self.character == DBL_QUOTE {
            self.next_char();
            return if self.mode == ReadMode::Default {
                self.mode = ReadMode::StringLiteral;
                Token::DblQuote // Open
            } else {
                self.mode = ReadMode::Default;
                Token::DblQuote // Close
            };
        }
        let start_pos = self.position;
        let mut last_char: char;
        while self.character != NULL_CHAR {
            last_char = self.character;
            self.next_char();
            if self.character == DBL_QUOTE && last_char != ESCAPE_CHAR {
                break;
            }
        }
        if self.character == NULL_CHAR {
            // the string literal did not terminate on this line.
            // let the parser handle this error.
            self.mode = ReadMode::Default;
        }
        let literal = self.slice_str(start_pos, self.position);
        Token::StringLiteral(literal)
    }

    fn read_char_literal(&mut self) -> Token {
        if self.character == QUOTE {
            self.next_char();
            return if self.mode == ReadMode::Default {
                self.mode = ReadMode::CharLiteral;
                Token::Quote // Open
            } else {
                self.mode = ReadMode::Default;
                Token::Quote // Close
            };
        }
        let start_pos = self.position;
        let mut last_char: char;
        while self.character != NULL_CHAR {
            last_char = self.character;
            self.next_char();
            if self.character == QUOTE && last_char != ESCAPE_CHAR {
                break;
            }
        }
        if self.character == NULL_CHAR {
            // the character literal did not terminate on this line.
            // let the parser handle this error.
            self.mode = ReadMode::Default;
        }
        let literal = self.slice_str(start_pos, self.position);
        Token::CharLiteral(literal)
    }

    fn read_letter_literal(&mut self) -> Token {
        let start_pos = self.position;
        while self.character != NULL_CHAR {
            if is_letter(self.character) || is_digit(self.character) {
                self.next_char();
            }
        }
        let literal = self.slice_str(start_pos, self.read_position);
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
            "void" => Token::Keyword(Keyword::VOID),
            "where" => Token::Keyword(Keyword::WHERE),
            "while" => Token::Keyword(Keyword::WHILE),
            _ => Token::Identifier(literal),
        }
    }

    fn read_number_literal(&mut self) -> Token {
        Token::EOF
    }

    fn read_single_line_comment(&mut self) -> Token {
        Token::EOF
    }

    fn slice_str(&self, start_pos: usize, end_pos: usize) -> String {
        let end_pos_min = end_pos.min(self.line.len());
        let sliced_chars = &self.line[start_pos..end_pos_min];
        sliced_chars.iter().collect()
    }
}

fn is_whitespace(ch: char) -> bool {
    if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
        return true;
    }
    return false;
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || ch == '$'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}
