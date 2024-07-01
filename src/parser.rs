use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::*;
use std::vec::IntoIter;

pub struct Parser {
    current: TokenFrame,
    peek: Option<TokenFrame>,
    program: Option<Program>,
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
        parser.build_program()
    }

    pub fn new(tokens: Vec<TokenFrame>) -> Self {
        Self {
            iter: tokens.into_iter(),
            program: None,
            peek: None,
            current: TokenFrame::empty(),
        }
    }

    pub fn build_program(&mut self) -> Program {
        let program = Program::new();

        let mut current_option = self.iter.next();

        if current_option.is_none() {
            return program;
        }

        self.program = Some(program);
        self.peek = current_option.take();

        while self.advance_token() {
            self.consume();
        }

        self.program.take().unwrap()
    }

    fn advance_token(&mut self) -> bool {
        let current_option = self.iter.next();
        if let Some(current) = current_option {
            self.current = self.peek.take().unwrap();
            self.peek = Some(current);
            self.consume();
            return true;
        } else if self.peek.is_some() {
            self.current = self.peek.take().unwrap();
            self.consume();
            return true;
        }
        false
    }

    fn consume(&mut self) {
        let node_option = match self.current.token {
            Token::Keyword(Keyword::LET) => Some(self.parse_let_statement()),
            Token::Keyword(Keyword::RETURN) => Some(self.parse_return_statement()),
            Token::Keyword(Keyword::IF) => Some(self.parse_if_statement()),
            _ => None,
        };
        if let Some(node) = node_option {
            let program = self.program.as_mut().unwrap();
            program.statements.push(node);
        }
    }

    // ----------------------------------------------------------------------
    // Begin parse functions
    // ----------------------------------------------------------------------

    fn parse_let_statement(&mut self) -> Node {
        Node::LetStatement(LetStatementNode {})
    }

    fn parse_return_statement(&mut self) -> Node {
        Node::ReturnStatement(ReturnStatementNode {})
    }

    fn parse_if_statement(&mut self) -> Node {
        Node::IfStatement(IfStatementNode {})
    }
}
