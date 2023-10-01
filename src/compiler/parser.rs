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

    fn next(&mut self) {
        self.position += 1;
        self.token = if self.position >= self.tokens.len() { Token::EOF } else { self.tokens[self.position].clone() };
        self.next_token = if self.position + 1 >= self.tokens.len() { Token::EOF } else { self.tokens[self.position + 1].clone() };
    }

    fn peek_error(&self, t: Token) {
        if std::mem::discriminant(&self.next_token) != std::mem::discriminant(&t) {
            panic!("expected next token to be {:?}, got {:?}", t, self.next_token);
        }
    }

    fn is_error(&self, t: Token) {
        if std::mem::discriminant(&self.token) != std::mem::discriminant(&t) {
            panic!("expected next token to be {:?}, got {:?}", t, self.token);
        }
    }

    pub fn parse(&mut self) -> Node {
        let mut statements = vec![];

        while self.token != Token::EOF {
            let statement = self.parse_statement();
            statements.push(statement);
            self.next();
        }

        Node::BLOCK(Box::new(statements))
    }

    fn parse_statement(&mut self) -> Node {
        match self.token {
            Token::LET => { self.parse_declaration() }
            _ => { panic!("Invalid Token: {:?} to begin a statement", self.token) }
        }
    }

    fn parse_declaration(&mut self) -> Node {
        self.peek_error(Token::Identifier(String::from("a")));
        self.next(); // pointing to identifier

        let identifier_str = if let Token::Identifier(identifier) = &self.token { identifier } else { panic!("DECLARATION, expected type IDENTIFIER following LET, got: {:?}", self.token) };
        let identifier: Node = Node::IDENTIFIER(identifier_str.clone());

        self.next();
        self.is_error(Token::EQ);
        self.next(); // positing to expression

        let expression = self.parse_expression();
        self.peek_error(Token::SEMICOLON);
        self.next();


        Node::DECLARATION(
            Box::new(identifier),
            Box::new(expression),
        )
    }

    fn parse_expression(&mut self) -> Node {
        let node: Node;
        match &self.token {
            Token::Number(value) => { node = Node::NUMBER(value.clone()); }
            Token::String(value) => { node = Node::STRING(value.clone()); }
            Token::Identifier(id) => { node = Node::IDENTIFIER(id.clone()); }

            _ => { panic!("Unexpected token in expression: {:?}", self.token); }
        }

        node
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
            Token::Number(10),
            Token::SEMICOLON,
            Token::EOF,
        ]);

        assert_eq!(p.parse(), Node::BLOCK(Box::new(vec![
            Node::DECLARATION(
                Box::new(Node::IDENTIFIER(String::from("x"))), 
                Box::new(Node::NUMBER(10)),
            )
        ])))
    }
}