use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::*;
use std::vec::IntoIter;

pub struct Parser {
    current: TokenFrame,
    peek: Option<TokenFrame>,
    iter: IntoIter<TokenFrame>,
}

impl Parser {
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

    fn location(&self) -> Location {
        Location {
            position: self.current.position,
            line: self.current.line,
        }
    }

    fn expect<F>(&mut self, mut callback: F) -> Node
    where
        F: FnMut(&mut Self, &Token) -> Node,
    {
        if let Some(peek) = self.peek.clone() {
            return callback(self, &peek.token);
        }
        Node::UnexpectedEndOfInput
    }
}

// ----------------------------------------------------------------------
// Begin parse functions
// ----------------------------------------------------------------------

fn create_program(p: &mut Parser) -> Program {
    let mut program = Program::new();

    while p.advance_token() {
        let node = p.expect(|p, token| match token {
            Token::Keyword(Keyword::LET) => parse_variable_statement(p),
            Token::Keyword(Keyword::RETURN) => parse_return_statement(p),
            Token::Keyword(Keyword::IF) => parse_if_statement(p),
            _ => Node::UnexpectedToken(token.clone()),
        });
        program.statements.push(node);
    }

    program
}

fn parse_variable_statement(p: &mut Parser) -> Node {
    let location = p.location();

    let keyword = p.expect(|p, token| match token {
        Token::Keyword(Keyword::LET) => parse_keyword(p),
        Token::Keyword(Keyword::CONST) => parse_keyword(p),
        _ => Node::UnexpectedToken(token.clone()),
    });

    // todo: create expression node

    let ident = p.expect(|p, token| match token {
        Token::Identifier(_) => parse_identifier(p),
        _ => Node::UnexpectedToken(token.clone()),
    });

    let operator = p.expect(|p, token| match token {
        Token::Equal => parse_operator(p),
        _ => Node::UnexpectedToken(token.clone()),
    });

    let literal = p.expect(|p, token| match token {
        Token::NumberLiteral(_) => parse_identifier(p),
        _ => Node::UnexpectedToken(token.clone()),
    });

    let semi = p.expect(|p, token| match token {
        Token::Semi => parse_identifier(p),
        _ => Node::UnexpectedToken(token.clone()),
    });

    Node::VariableStatement(VariableStatementNode {
        location,
        keyword: Box::new(keyword),
        ident: Box::new(ident),
        operator: Box::new(operator),
        literal: Box::new(literal),
        semi: Box::new(semi),
    })
}

fn parse_keyword(p: &mut Parser) -> Node {
    p.advance_token();
    Node::Identifier(p.current.token.clone())
}

fn parse_identifier(p: &mut Parser) -> Node {
    p.advance_token();
    Node::Identifier(p.current.token.clone())
}

fn parse_operator(p: &mut Parser) -> Node {
    p.advance_token();
    Node::Operator(p.current.token.clone())
}

fn parse_return_statement(_p: &mut Parser) -> Node {
    Node::ReturnStatement(ReturnStatementNode {})
}

fn parse_if_statement(_p: &mut Parser) -> Node {
    Node::IfStatement(IfStatementNode {})
}
