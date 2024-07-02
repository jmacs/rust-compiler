use crate::token::*;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Node>,
    pub errors: Vec<ParseError>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            errors: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Node {
    VariableDeclaration(VariableDeclarationNode),
    ReturnStatement(ReturnStatementNode),
    IfStatement(IfStatementNode),
    Keyword(KeywordNode),
    Identifier(IdentifierNode),
    NumberLiteral(NumberLiteralNode),
    Semi(Token),
}

#[derive(Debug, PartialEq)]
pub struct Location {
    pub position: usize,
    pub line: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
    UnexpectedKeyword(Keyword),
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclarationNode {
    pub location: Location,
    pub keyword: Box<Node>,
    pub identifier: Box<Node>,
    pub literal: Box<Node>,
    pub semi: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatementNode {}

#[derive(Debug, PartialEq)]
pub struct IfStatementNode {}

#[derive(Debug, PartialEq)]
pub struct KeywordNode {
    pub location: Location,
    pub keyword: Keyword,
}

#[derive(Debug, PartialEq)]
pub struct IdentifierNode {
    pub location: Location,
    pub identifier: String,
}

#[derive(Debug, PartialEq)]
pub struct NumberLiteralNode {
    pub location: Location,
    pub kind: NumberKind,
    pub value: String,
    pub postfix: Option<String>,
}
