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
    Variable(VariableNode),
    Return(ReturnNode),
    If(IfNode),
    Keyword(KeywordNode),
    Ident(IdentNode),
    Number(NumberLiteralNode),
    Semi(Span),
}

#[derive(Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
    UnexpectedKeyword(Keyword),
}

#[derive(Debug, PartialEq)]
pub struct VariableNode {
    pub span: Span,
    pub keyword: Box<Node>,
    pub identifier: Box<Node>,
    pub literal: Box<Node>,
    pub semi: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnNode {}

#[derive(Debug, PartialEq)]
pub struct IfNode {}

#[derive(Debug, PartialEq)]
pub struct KeywordNode {
    pub span: Span,
    pub keyword: Keyword,
}

#[derive(Debug, PartialEq)]
pub struct IdentNode {
    pub span: Span,
    pub identifier: String,
}

#[derive(Debug, PartialEq)]
pub struct NumberLiteralNode {
    pub span: Span,
    pub kind: NumberKind,
    pub value: String,
    pub postfix: Option<String>,
}
