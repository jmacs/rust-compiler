use crate::token::*;

const NULL_CHAR: char = '\0';
const ESCAPE_CHAR: char = '\\';
const QUOTE: char = '\'';
const DBL_QUOTE: char = '"';

#[derive(Debug, PartialEq)]
enum ReadMode {
    Default,
    // MultilineComment,
    // MultilineString,
}

pub struct Lexer {
    read_mode: ReadMode,
    line: Vec<char>,
    character: char,
    position: usize,
    line_num: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            line: Vec::new(),
            read_mode: ReadMode::Default,
            position: 0,
            read_position: 0,
            line_num: 0,
            character: NULL_CHAR,
        }
    }

    pub fn read_line(&mut self, input: &str) {
        self.line_num += 1;
        self.line.clear();
        self.line.extend(input.chars());
        self.position = 0;
        self.read_position = 0;
        self.next_char();
    }

    pub fn next_token(&mut self) -> Option<TokenFrame> {
        let position = self.position;
        let line = self.line_num;

        if self.character == NULL_CHAR {
            return None;
        }

        let mut token_opt = match self.read_mode {
            ReadMode::Default => {
                self.skip_whitespace();
                let token = self.read_token();
                if token == Token::EOF {
                    return None;
                }
                Some(token)
            }
        };

        if token_opt.is_none() {
            return None;
        }

        let token = token_opt.take().unwrap();

        Some(TokenFrame {
            token,
            position,
            line,
        })
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
            // Else
            //
            ch => {
                return match ch {
                    _ if is_digit(ch) => self.read_number_literal(),
                    _ if is_alpha(ch) => self.read_alpha_literal(),
                    _ => {
                        self.next_char();
                        Token::Error(TokenError::Illegal(ch))
                    }
                }
            }
        };

        self.next_char();
        token
    }

    fn read_string_literal(&mut self) -> Token {
        let start_pos = self.position;
        if !self.read_until_char(DBL_QUOTE) {
            return Token::Error(TokenError::UnterminatedStringLiteral);
        }
        let literal = self.slice_line(start_pos + 1, self.position);
        self.next_char();
        Token::StringLiteral(literal)
    }

    fn read_char_literal(&mut self) -> Token {
        let start_pos = self.position;
        if !self.read_until_char(QUOTE) {
            return Token::Error(TokenError::UnterminatedCharLiteral);
        }
        let literal = self.slice_line(start_pos + 1, self.position);
        self.next_char();
        Token::CharLiteral(literal)
    }

    fn read_number_literal(&mut self) -> Token {
        let start_pos = self.position;
        let lookahead = self.peek_char();

        // Hexadecimal
        if self.character == '0' && (lookahead == 'X' || lookahead == 'x') {
            self.next_char();
            self.read_while(|ch| !is_hex(ch));
            let literal = self.slice_line(start_pos, self.position);
            if literal.len() == 2 {
                return Token::Error(TokenError::MalformedHexadecimal);
            }
            return Token::NumberLiteral(Number {
                kind: NumberKind::Hexadecimal,
                value: literal,
                postfix: None,
            });
        }

        // Integer
        let mut number_kind = NumberKind::Integer;
        self.read_while(|ch| !is_valid_number_literal_char(ch));

        // Decimal
        if self.character == '.' && is_digit(self.peek_char()) {
            number_kind = NumberKind::Decimal;
            self.next_char();
            self.read_while(|ch| !is_valid_number_literal_char(ch));
        }

        let number_value = self.slice_line(start_pos, self.position);

        // Postfix
        let mut postfix_opt: Option<String> = None;
        if is_alpha(self.character) {
            let postfix_start = self.position;
            self.read_while(|ch| !is_alpha(ch));
            let postfix_str = self.slice_line(postfix_start, self.position);
            postfix_opt = Some(postfix_str);
        }

        Token::NumberLiteral(Number {
            kind: number_kind,
            value: number_value,
            postfix: postfix_opt,
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
        let literal = self.slice_line(start_pos, self.position);
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

    fn read_while<F>(&mut self, mut condition: F)
    where
        F: FnMut(char) -> bool,
    {
        while self.character != NULL_CHAR {
            self.next_char();
            if condition(self.character) {
                break;
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
