use crate::compiler::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    BLOCK(Box<Vec<Node>>),
    DECLARATION(Box<Node>, Box<Node>), // identifier, expression
    INFIX(Box<Node>, Token, Box<Node>),
    INVOCATION(Box<Node>, Box<Vec<Node>>),
    LIBRARY(Box<Node>),

    IDENTIFIER(String),
    NUMBER(i32),
    STRING(String),
}