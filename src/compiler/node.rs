use crate::compiler::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    BLOCK(Box<Vec<Node>>),
    DECLARATION(String, Box<Node>), // identifier, expression
    INFIX(Box<Node>, Token, Box<Node>),
    INVOCATION(String, Box<Vec<Node>>),
    LIBRARY(String),
    FUNCTION(String, Vec<String>, Box<Node>),
    RETURN(Box<Node>),

    IDENTIFIER(String),
    NUMBER(i32),
    STRING(String),
}