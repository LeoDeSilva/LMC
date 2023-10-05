use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Identifier(String),
    String(String),
    Number(i32),

    SEMICOLON,
    COMMA,
    EOF,

    ADD,
    SUB,

    NOT,
    NE,

    EQ,
    EE,
    GT,
    GTE,
    LT,
    LTE,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    LET,
    FN,
    USE,
    RETURN,
    HALT,
    IF,
    ELIF,
    ELSE,
}


pub struct Lexer {
    program: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(program: Vec<char>) -> Self {
        let position: usize = 0;
        let read_position: usize = 0;
        let ch = '\0';

        Lexer { program: program, position: position, read_position: read_position, ch: ch }
    }

    fn eat_char(&mut self) {
        if self.read_position >= self.program.len() {
            self.ch = '\0';
        } else {
            self.ch = self.program[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        self.eat_char();

        loop {
            let token: Token = self.lex_token();
            if token == Token::EOF {
                tokens.push(token);
                break;
            }

            tokens.push(token);
        }

        tokens
    }

    fn lex_token(&mut self) -> Token {
        self.eat_whitespace();
        let tok: Token;

        match self.ch {
            ',' => { tok = Token::COMMA }
            '+' => { tok = Token::ADD }
            '-' => { tok = Token::SUB }

            '=' => { tok = self.lex_multichar(Token::EQ, ('=', Token::EE)) }
            '!' => { tok = self.lex_multichar(Token::NOT, ('=', Token::NE)) }
            '>' => { tok = self.lex_multichar(Token::GT, ('=', Token::GTE)) }
            '<' => { tok = self.lex_multichar(Token::LT, ('=', Token::LTE)) }

            '(' => { tok = Token::LPAREN }
            ')' => { tok = Token::RPAREN }
            '{' => { tok = Token::LBRACE }
            '}' => { tok = Token::RBRACE }

            ';' => { tok = Token::SEMICOLON }
            '\0' => { tok = Token::EOF }
            '"' => { tok = self.lex_string() }

            '0'..='9' => { return self.lex_number() }
            '_' |'A'..='z' => { return self.lex_identifier() }

            _ => { panic!("Unexpected character found in lexer: {}", self.ch) }
        }

        self.eat_char();
        tok
    }

    fn lex_multichar(&mut self, single: Token, double: (char, Token)) -> Token {
        if self.read_position >= self.program.len() || self.program[self.read_position] != double.0{
            return single;
        } 

        self.eat_char();
        return double.1
    }

    fn lex_number(&mut self) -> Token {
        let position: usize = self.position;
        while self.position < self.program.len() && self.ch.is_numeric() {
            self.eat_char();
        }

        let numeral_str: String = self.program[position..self.position].to_vec().iter().collect();
        Token::Number(numeral_str.parse::<i32>().unwrap())
    }

    fn lex_identifier(&mut self) -> Token {
        let keywords: HashMap<String, Token> = [
            ("let".to_string(), Token::LET),
            ("use".to_string(), Token::USE),
            ("fn".to_string(), Token::FN),
            ("return".to_string(), Token::RETURN),
            ("halt".to_string(), Token::HALT),
            ("if".to_string(), Token::IF),
            ("elif".to_string(), Token::ELIF),
            ("else".to_string(), Token::ELSE),
        ].iter().cloned().collect();

        let position: usize = self.position;
        while self.position < self.program.len() && self.ch.is_alphanumeric() || self.ch == '_' {
            self.eat_char();
        }

        let identifier: String = self.program[position..self.position].to_vec().iter().collect();

        if keywords.contains_key(&identifier) {
            keywords.get(&identifier).unwrap().clone()
        } else {
            Token::Identifier(identifier)
        }
    }

    fn lex_string(&mut self) -> Token {
        self.eat_char();
        let position: usize = self.position;

        while self.ch != '"' && self.position < self.program.len() {
            self.eat_char();
        }
        
        Token::String(self.program[position..self.position].to_vec().iter().collect())
    }

    fn eat_whitespace(&mut self) {
        while self.position < self.program.len() && self.ch.is_ascii_whitespace() {
            self.eat_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_number() {
        let mut l = Lexer::new(String::from("10 0 99 2147483647").chars().collect());
        assert_eq!(l.lex(), vec![
            Token::Number(10),
            Token::Number(0),
            Token::Number(99),
            Token::Number(2147483647),
            Token::EOF,
        ])
    }

    #[test]
    fn test_lex_multichar() {
        let mut l = Lexer::new(String::from("= == >= < <= > != ! =").chars().collect());
        assert_eq!(l.lex(), vec![
            Token::EQ,
            Token::EE,
            Token::GTE,
            Token::LT,
            Token::LTE,
            Token::GT,
            Token::NE,
            Token::NOT,
            Token::EQ,
            Token::EOF,
        ])
    }

    #[test]
    fn test_lex_identifier() {
        let mut l = Lexer::new(String::from("x y is_str let use if elif else").chars().collect());
        assert_eq!(l.lex(), vec![
            Token::Identifier(String::from("x")),
            Token::Identifier(String::from("y")),
            Token::Identifier(String::from("is_str")),
            Token::LET,
            Token::USE,
            Token::IF,
            Token::ELIF,
            Token::ELSE,
            Token::EOF,
        ])
    }

    #[test]
    fn test_lex_string() {
        let mut l = Lexer::new(String::from("\"Hello World\"").chars().collect());
        assert_eq!(l.lex(), vec![
            Token::String(String::from("Hello World")),
            Token::EOF,
        ])
    }

    #[test]
    fn test_lex_empty() {
        let mut l = Lexer::new(String::from("").chars().collect());
        assert_eq!(l.lex(), vec![
            Token::EOF,
        ])
    }
}
