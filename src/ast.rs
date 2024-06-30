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
    LetStatement(LetStatementNode),
    ReturnStatement(ReturnStatementNode),
    IfStatement(IfStatementNode),
}

#[derive(Debug)]
pub struct LetStatementNode {}

#[derive(Debug)]
pub struct ReturnStatementNode {}

#[derive(Debug, PartialEq)]
pub struct IfStatementNode {}
