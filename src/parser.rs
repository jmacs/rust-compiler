use crate::ast::*;
use crate::lexer::*;
use crate::token::*;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::vec::IntoIter;

pub struct Parser {
    current: TokenFrame,
    peek: Option<TokenFrame>,
    iter: IntoIter<TokenFrame>,
}

impl Parser {
    pub fn parse_file(file_path: &str) -> Result<Program, io::Error> {
        let mut lexer = Lexer::new();
        let mut tokens: Vec<TokenFrame> = Vec::new();

        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            lexer.read_line(&line?);
            while let Some(frame) = lexer.next_token() {
                tokens.push(frame);
            }
        }

        let mut parser = Parser::new(tokens);
        Ok(create_program(&mut parser))
    }

    pub fn parse_source(lines: Vec<&str>) -> Program {
        let mut lexer = Lexer::new();
        let mut lines_iter = lines.into_iter();
        let mut tokens: Vec<TokenFrame> = Vec::new();

        while let Some(line) = lines_iter.next() {
            lexer.read_line(line);
            while let Some(frame) = lexer.next_token() {
                tokens.push(frame);
            }
        }

        let mut parser = Parser::new(tokens);
        create_program(&mut parser)
    }

    pub fn new(tokens: Vec<TokenFrame>) -> Self {
        Self {
            iter: tokens.into_iter(),
            current: TokenFrame::empty(),
            peek: None,
        }
    }

    fn span(&self) -> Span {
        Span {
            start: self.current.start,
            end: self.current.end,
        }
    }

    fn advance_token(&mut self) -> bool {
        let current_option = self.iter.next();
        if let Some(current) = current_option {
            self.current = self.peek.take().unwrap_or_else(TokenFrame::empty);
            self.peek = Some(current);
            return true;
        } else if self.peek.is_some() {
            self.current = self.peek.take().unwrap();
            return true;
        }
        false
    }

    #[allow(dead_code)]
    fn peek_token(&self) -> &Token {
        match &self.peek {
            Some(peek) => &peek.token,
            None => &Token::EOF,
        }
    }

    fn current_token(&self) -> &Token {
        &self.current.token
    }

    fn current_frame(&self) -> &TokenFrame {
        &self.current
    }

    fn expect_token(&self, token: Token) -> Result<(), ParseError> {
        return if self.current.token == token {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(self.current.clone()))
        };
    }

    fn expect_keyword(&self, keywords: Vec<Keyword>) -> Result<(), ParseError> {
        match &self.current.token {
            Token::Keyword(kw) => {
                if keywords.contains(kw) {
                    Ok(())
                } else {
                    Err(ParseError::UnexpectedKeyword(kw.clone()))
                }
            }
            _ => Err(ParseError::UnexpectedToken(self.current.clone())),
        }
    }

    #[allow(dead_code)]
    fn expect_peek(&self, token: Token) -> Result<(), ParseError> {
        self.peek
            .as_ref()
            .ok_or(ParseError::UnexpectedEndOfInput)
            .and_then(|peek| {
                if peek.token == token {
                    Ok(())
                } else {
                    Err(ParseError::UnexpectedToken(peek.clone()))
                }
            })
    }
}

// ----------------------------------------------------------------------
// Begin parse functions
// ----------------------------------------------------------------------

fn unexpected_token(p: &mut Parser) -> ParseError {
    ParseError::UnexpectedToken(p.current_frame().clone())
}

fn synchronize(_p: &mut Parser, _program: &Program) {
    // todo: skip tokens until a recovery point is found.
}

fn create_program(p: &mut Parser) -> Program {
    let mut program = Program::new();

    p.advance_token(); // prime the peek token

    while p.advance_token() {
        match parse_root_statement(p) {
            Ok(statement) => program.statements.push(statement),
            Err(err) => {
                program.errors.push(err);
                synchronize(p, &program);
                return program;
            }
        }
    }

    program
}

fn parse_root_statement(p: &mut Parser) -> Result<Node, ParseError> {
    let statement = match p.current_token() {
        Token::Keyword(Keyword::LET) => parse_variable_statement(p),
        Token::Keyword(Keyword::CONST) => parse_variable_statement(p),
        _ => Err(unexpected_token(p)),
    }?;
    Ok(statement)
}

fn parse_variable_statement(p: &mut Parser) -> Result<Node, ParseError> {
    let location = p.span();

    p.expect_keyword(vec![Keyword::LET, Keyword::CONST])?;
    let keyword = parse_keyword(p)?;

    let identifier = parse_identifier(p)?;

    p.expect_token(Token::Equal)?;
    p.advance_token();

    let literal = parse_literal(p)?;

    let semi = match p.current_token() {
        Token::Semi => parse_semi(p),
        _ => Err(unexpected_token(p)),
    }?;

    Ok(Node::Variable(VariableNode {
        span: location,
        keyword: Box::new(keyword),
        identifier: Box::new(identifier),
        literal: Box::new(literal),
        semi: Box::new(semi),
    }))
}

fn parse_keyword(p: &mut Parser) -> Result<Node, ParseError> {
    let location = p.span();
    let keyword = match p.current_token() {
        Token::Keyword(keyword) => Ok(keyword.clone()),
        _ => Err(unexpected_token(p)),
    }?;
    p.advance_token();
    Ok(Node::Keyword(KeywordNode {
        span: location,
        keyword,
    }))
}

fn parse_identifier(p: &mut Parser) -> Result<Node, ParseError> {
    let location = p.span();
    let identifier = match p.current_token() {
        Token::Identifier(identifier) => Ok(identifier.clone()),
        _ => Err(unexpected_token(p)),
    }?;
    p.advance_token();
    Ok(Node::Ident(IdentNode {
        span: location,
        identifier,
    }))
}

fn parse_literal(p: &mut Parser) -> Result<Node, ParseError> {
    return match p.current_token() {
        Token::NumberLiteral(_) => parse_number_literal(p),
        _ => Err(unexpected_token(p)),
    };
}

fn parse_number_literal(p: &mut Parser) -> Result<Node, ParseError> {
    let location = p.span();
    let number = match p.current_token() {
        Token::NumberLiteral(literal) => Ok(literal.clone()),
        _ => Err(unexpected_token(p)),
    }?;
    p.advance_token();
    Ok(Node::Number(NumberNode {
        span: location,
        kind: number.kind,
        postfix: number.postfix,
        value: number.value,
    }))
}

fn parse_semi(p: &mut Parser) -> Result<Node, ParseError> {
    p.advance_token();
    let location = p.span();
    Ok(Node::Semi(location))
}
