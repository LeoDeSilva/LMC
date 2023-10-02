use std::collections::HashMap;

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

    fn eat(&mut self) {
        self.position += 1;
        self.token = if self.position >= self.tokens.len() { Token::EOF } else { self.tokens[self.position].clone() };
        self.next_token = if self.position + 1 >= self.tokens.len() { Token::EOF } else { self.tokens[self.position + 1].clone() };
    }

    fn peek_error(&mut self, t: Token) {
        if std::mem::discriminant(&self.next_token) != std::mem::discriminant(&t) {
            panic!("expected eat token to be {:?}, got {:?}", t, self.next_token);
        }

        self.eat();
    }

    fn eat_error(&mut self, t: Token) {
        if std::mem::discriminant(&self.token) != std::mem::discriminant(&t) {
            panic!("expected eat token to be {:?}, got {:?}", t, self.token);
        }
        self.eat();
    }

    fn is_error(&mut self, t: Token) {
        if std::mem::discriminant(&self.token) != std::mem::discriminant(&t) {
            panic!("expected eat token to be {:?}, got {:?}", t, self.token);
        }
    }

    pub fn parse(&mut self) -> Node {
        let mut statements = vec![];

        while self.token != Token::EOF {
            let statement = self.parse_statement();
            statements.push(statement);
            println!("{:?}", self.token);
            self.eat_error(Token::SEMICOLON);
        }

        Node::BLOCK(Box::new(statements))
    }

    fn parse_statement(&mut self) -> Node {
        match self.token {
            Token::LET => { self.parse_declaration() }
            _ => { self.parse_expression(0) }
            // _ => { panic!("Invalid Token: {:?} to begin a statement", self.token) }
        }
    }

    fn parse_declaration(&mut self) -> Node {
        self.peek_error(Token::Identifier(String::from("")));

        let identifier_str = if let Token::Identifier(identifier) = &self.token { identifier } else { panic!("DECLARATION, expected type IDENTIFIER following LET, got: {:?}", self.token) };
        let identifier: Node = Node::IDENTIFIER(identifier_str.clone());

        self.peek_error(Token::EQ);
        self.eat(); // positing to expression

        let expression = self.parse_expression(0);

        Node::DECLARATION(
            Box::new(identifier),
            Box::new(expression),
        )
    }

    fn parse_expression(&mut self, rbp: i32) -> Node {
        let mut lhs = self.parse_atom();
        self.eat();

        let mut peek_rbp = self.get_preference(self.token.clone());

        while self.next_token != Token::EOF && peek_rbp >= rbp {
            lhs = self.parse_infix(lhs, self.token.clone());
            peek_rbp = self.get_preference(self.token.clone());
        }

        lhs
    }

    fn parse_infix(&mut self, lhs: Node, op: Token) -> Node {
        if !vec![Token::ADD, Token::SUB, Token::EE, Token::NE, Token::LT, Token::GT, Token::GTE, Token::LTE].contains(&op) {
            panic!("SyntaxError: unsupported infix operator, got: {:?}", op);
        }

        self.eat();
        let rhs = self.parse_expression(self.get_preference(op.clone()) + 1);

        return Node::INFIX(
            Box::new(lhs), 
            op,
            Box::new(rhs)
        )
    }

    fn parse_atom(&mut self) -> Node {
        let node: Node;
        match &self.token {
            Token::Number(value) => { node = Node::NUMBER(value.clone()); }
            Token::String(value) => { node = Node::STRING(value.clone()); }
            Token::Identifier(id) => { node = Node::IDENTIFIER(id.clone()); }
            Token::LPAREN => {
                self.eat();
                node = self.parse_expression(0);
                self.is_error(Token::RPAREN);
            }

            _ => { panic!("Unexpected token in expression: {:?}", self.token); }
        }

        node
    }

    fn get_preference(&self, t: Token) -> i32 {
        let preferences: HashMap<Token, i32> = [
            (Token::EE, 10),
            (Token::NE, 10),
            (Token::GT, 10),
            (Token::LT, 10),
            (Token::GTE, 10),
            (Token::LTE, 10),

            (Token::ADD, 20),
            (Token::SUB, 20),
            
            // (Token::MUL, 30), (Token::DIV, 30),
            (Token::LPAREN, 0),
        ].iter().cloned().collect();

        if preferences.contains_key(&t) {
            preferences.get(&t).unwrap().clone()
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_declaration() {
        let mut p = Parser::new(vec![
            Token::LET,
            Token::Identifier(String::from("x")),
            Token::EQ,
            Token::Number(1),
            Token::SEMICOLON,

            Token::LET,
            Token::Identifier(String::from("y")),
            Token::EQ,
            Token::Identifier(String::from("x")),
            Token::ADD,
            Token::Number(2),
            Token::SEMICOLON,

            Token::EOF,
        ]);

        assert_eq!(p.parse(), Node::BLOCK(Box::new(vec![
            Node::DECLARATION(
                Box::new(Node::IDENTIFIER(String::from("x"))), 
                Box::new(Node::NUMBER(1)),
            ),
            Node::DECLARATION(
                Box::new(Node::IDENTIFIER(String::from("y"))), 
                Box::new(Node::INFIX(
                    Box::new(Node::IDENTIFIER(String::from("x"))),
                    Token::ADD,
                    Box::new(Node::NUMBER(2)),
                )),
            )
        ])))
    }

    #[test]
    fn test_parse_parenthesis() {
        let mut p = Parser::new(vec![
            Token::Number(3),
            Token::ADD,
            Token::LPAREN,
            Token::Number(2),
            Token::SUB,
            Token::Number(1),
            Token::RPAREN,
            Token::SEMICOLON,

            Token::LET,
            Token::Identifier(String::from("a")),
            Token::EQ,
            Token::LPAREN,

            Token::Number(3),
            Token::ADD,
            Token::LPAREN,
            Token::Number(2),
            Token::SUB,
            Token::Number(1),
            Token::RPAREN,

            Token::RPAREN,
            Token::SEMICOLON,

            Token::EOF,
        ]);

        assert_eq!(p.parse(), Node::BLOCK(Box::new(vec![
            Node::INFIX(
                Box::new(Node::NUMBER(3)), 
                Token::ADD,
                Box::new(Node::INFIX(
                    Box::new(Node::NUMBER(2)), 
                    Token::SUB,
                    Box::new(Node::NUMBER(1)), 
                ))
            ),

            Node::DECLARATION(
                Box::new(Node::IDENTIFIER(String::from("a"))), 
                Box::new(Node::INFIX(
                    Box::new(Node::NUMBER(3)), 
                    Token::ADD,
                    Box::new(Node::INFIX(
                        Box::new(Node::NUMBER(2)), 
                        Token::SUB,
                        Box::new(Node::NUMBER(1)), 
                    ))
                )),
            )
        ])))
    }
}