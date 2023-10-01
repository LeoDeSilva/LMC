use crate::compiler::lexer::Token;
use crate::compiler::node::Node;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    token: Token,
    next_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tok = tokens[0].clone();
        let next_tok = if tokens.len() > 1 { tokens[1].clone() } else { Token::EOF };
        Parser { tokens: tokens, position: 0, token: tok, next_token: next_tok }
    }

    fn next_token(&mut self) {
        self.position += 1;
        self.token = if self.position >= self.tokens.len() { Token::EOF } else { self.tokens[self.position].clone() };
        self.next_token = if self.position + 1 >= self.tokens.len() { Token::EOF } else { self.tokens[self.position + 1].clone() };
    }

    pub fn parse(&mut self) -> Node {
        let program = Node::BLOCK(Box::new(vec![]));
        while self.token != Token::EOF {
            println!("{:?}", self.token);
            self.next_token();
        }
        program
    }
}