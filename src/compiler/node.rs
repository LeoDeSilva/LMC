#[derive(Debug, PartialEq)]
pub enum Node {
    BLOCK(Box<Vec<Node>>),
    DECLARATION(Box<Node>, Box<Node>), // identifier, expression

    IDENTIFIER(String),
    NUMBER(i32),
    STRING(String),
}