use crate::token::*;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Node>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    EOF,
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
    VariableStatement(VariableStatementNode),
    ReturnStatement(ReturnStatementNode),
    IfStatement(IfStatementNode),
    Keyword(Token),
    Identifier(Token),
    Operator(Token),
    Literal(Token),
}

#[derive(Debug)]
pub struct Location {
    pub position: usize,
    pub line: usize,
}

#[derive(Debug)]
pub struct VariableStatementNode {
    pub location: Location,
    pub keyword: Box<Node>,
    pub ident: Box<Node>,
    pub operator: Box<Node>,
    pub literal: Box<Node>,
    pub semi: Box<Node>,
}

#[derive(Debug)]
pub struct ReturnStatementNode {}

#[derive(Debug, PartialEq)]
pub struct IfStatementNode {}
