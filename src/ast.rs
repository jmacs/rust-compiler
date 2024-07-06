use crate::token::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Node {
    Variable(VariableNode),
    Return(ReturnNode),
    If(IfNode),
    Keyword(KeywordNode),
    Ident(IdentNode),
    Number(NumberNode),
    Semi(Span),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ParseError {
    UnexpectedToken(TokenFrame),
    UnexpectedEndOfInput,
    UnexpectedKeyword(Keyword),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct VariableNode {
    pub span: Span,
    pub keyword: Box<Node>,
    pub identifier: Box<Node>,
    pub literal: Box<Node>,
    pub semi: Box<Node>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ReturnNode {
    pub span: Span,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct IfNode {
    pub span: Span,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct KeywordNode {
    pub span: Span,
    pub keyword: Keyword,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct IdentNode {
    pub span: Span,
    pub identifier: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct NumberNode {
    pub span: Span,
    pub kind: NumberKind,
    pub value: String,
    pub postfix: Option<String>,
}
