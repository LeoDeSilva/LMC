use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Label(String),
    Number(u16),
    NEWLINE,
    EOF,

    HLT,
    ADD,
    SUB,
    LDA,
    STA,
    BRA,
    BRZ,
    BLT,
    BGT,
    INP,
    OUT,
    OTC,
    DAT,
}


pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}


impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        let position: usize= 0;
        let read_position: usize= 0;
        let ch: char = char::from_u32(0).unwrap();
        Lexer { input, position, read_position, ch }
    }


    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        self.read_char();
        loop {
            let token = self.next_token();
            if token == Token::EOF {
                tokens.push(token);
                break;
            } 

            tokens.push(token);

        }

        return tokens;
    }


    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = char::default();
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }


    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        let tok: Token;
        match self.ch {
            '0'..='9' => { return self.read_number() }
            '_' |'A'..='z' => { return self.read_identifier() }
            '\n' => { tok = Token::NEWLINE }
            '\0' => { tok = Token::EOF }
            _ => { panic!("Unexpected character found in lexer: {}", self.ch) }
        }

        self.read_char();
        return tok;
    }


    pub fn eat_whitespace(&mut self) {
        while self.position < self.input.len() && self.ch.is_ascii_whitespace() && self.ch != '\n' {
            self.read_char();
        }
    }


    pub fn read_number(&mut self) -> Token {
        let position = self.position;
        while self.position < self.input.len() && self.ch.is_numeric() {
            self.read_char();
        }

        let numeric_identifier: String = self.input[position..self.position].to_vec().iter().collect();
        return Token::Number(numeric_identifier.parse::<u16>().unwrap());
    }



    pub fn read_identifier(&mut self) -> Token {
        let keywords: HashMap<&str, Token> = [
            ("hlt", Token::HLT),
            ("add", Token::ADD),
            ("sub", Token::SUB),
            ("lda", Token::LDA),
            ("sta", Token::STA),
            ("bra", Token::BRA),
            ("brz", Token::BRZ),
            ("bgt", Token::BGT),
            ("blt", Token::BLT),
            ("inp", Token::INP),
            ("out", Token::OUT),
            ("otc", Token::OTC),
            ("dat", Token::DAT),
            ].iter().cloned().collect();

        let position = self.position;
        while self.position < self.input.len() && (self.ch.is_alphabetic() || self.ch == '_' ) {
            self.read_char();
        }

        let identifier: String = self.input[position..self.position].to_vec().iter().collect();
        if keywords.contains_key(identifier.to_lowercase().as_str()) {
            let token: Token = keywords.get(identifier.to_lowercase().as_str()).unwrap().clone();
            return token;
        }

        return Token::Label(self.input[position..self.position].to_vec().iter().collect());
    }
}

// Unit Tetsts

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let mut l = Lexer::new(String::from("main add 0\n\tsta RESULT").chars().collect());
        assert_eq!(l.lex(), vec![
            Token::Label(String::from("main")),
            Token::ADD,
            Token::Number(0),
            Token::NEWLINE,
            Token::STA,
            Token::Label(String::from("RESULT")),
            Token::EOF,
            ])
    }

    #[test]
    fn test_keyword_capitalisation() {
        let mut l = Lexer::new(String::from("ADD sta DaT\n lda lad").chars().collect());
        assert_eq!(l.lex(), vec![
            Token::ADD,
            Token::STA,
            Token::DAT,
            Token::NEWLINE,
            Token::LDA,
            Token::Label(String::from("lad")),
            Token::EOF,
            ])
    }

    #[test]
    fn test_identifier_underscores() {
        let mut l = Lexer::new(String::from("_start st_art start_ _").chars().collect());
        assert_eq!(l.lex(), vec![
            Token::Label(String::from("_start")),
            Token::Label(String::from("st_art")),
            Token::Label(String::from("start_")),
            Token::Label(String::from("_")),
            Token::EOF,
            ])

    }

    #[test]
    fn test_empty() {
        let mut l = Lexer::new(String::from("").chars().collect());
        assert_eq!(l.lex(), vec![ Token::EOF ])
    }

    #[test]
    fn test_whitespace() {
        let mut l = Lexer::new(String::from("\t\n\t    \t\t \r\n").chars().collect());
        assert_eq!(l.lex(), vec![ 
            Token::NEWLINE,
            Token::NEWLINE,
            Token::EOF 
            ])
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let mut l = Lexer::new(String::from("add #10").chars().collect());
        l.lex();
    }
}