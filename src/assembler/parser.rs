use std::collections::HashMap;
use crate::assembler::lexer;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Instruction {
    HLT,
    ADD(lexer::Token),
    SUB(lexer::Token),
    LDA(lexer::Token),
    STA(lexer::Token),
    BRA(lexer::Token),
    BRZ(lexer::Token),
    BGT(lexer::Token),
    BLT(lexer::Token),
    INP,
    OUT,
    OTC,
    DAT(lexer::Token),
    CALL(lexer::Token),
    RET,
}

pub struct Parser {
    tokens: Vec<lexer::Token>, 
    position: usize,
    instruction_number: usize,
    tok: lexer::Token,
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Token>) -> Self {
        let tok = tokens[0].clone();
        Parser { tokens: tokens, position: 0, instruction_number: 0, tok: tok }
    }

    pub fn peek(&self) -> lexer::Token {
        if self.position + 1 < self.tokens.len() {
            return self.tokens[self.position + 1].clone();
        }

        return lexer::Token::EOF;
    }

    pub fn eat_token(&mut self) {
        self.position += 1;
        self.tok = self.tokens[self.position].clone();
    }

    pub fn parse(&mut self) -> (Vec<Instruction>, HashMap<String, u16>) {
        let mut symbol_table: HashMap<String, u16> = HashMap::new();
        let mut program: Vec<Instruction> = vec![];

        while self.tok != lexer::Token::EOF {
            match &self.tok {
                lexer::Token::Label(identifier) => { 
                    symbol_table.insert(identifier.clone(), self.instruction_number.try_into().unwrap()); 
                    self.eat_token();
                }
                lexer::Token::NEWLINE => { self.eat_token(); }
                _ => {
                    program.push(self.parse_instruction());
                    self.instruction_number += 1;
                    self.eat_token();
                    if self.tok != lexer::Token::NEWLINE && self.tok != lexer::Token::EOF {
                        panic!("expected newline, line: {}", self.instruction_number);
                    } else if self.tok == lexer::Token::EOF {
                        break;
                    }

                    self.eat_token();
                }
            }
        }

        (program, symbol_table)
    }

    fn parse_instruction(&mut self) -> Instruction {
        match &self.tok {
            lexer::Token::HLT => Instruction::HLT,
            lexer::Token::ADD => Instruction::ADD(self.parse_operand()),
            lexer::Token::SUB => Instruction::SUB(self.parse_operand()),
            lexer::Token::LDA => Instruction::LDA(self.parse_operand()),
            lexer::Token::STA => Instruction::STA(self.parse_operand()),
            lexer::Token::BRA => Instruction::BRA(self.parse_operand()),
            lexer::Token::BRZ => Instruction::BRZ(self.parse_operand()),
            lexer::Token::BGT => Instruction::BGT(self.parse_operand()),
            lexer::Token::BLT => Instruction::BLT(self.parse_operand()),
            lexer::Token::INP => Instruction::INP,
            lexer::Token::OUT => Instruction::OUT,
            lexer::Token::OTC => Instruction::OTC,
            lexer::Token::DAT => Instruction::DAT(self.parse_operand()),
            lexer::Token::CALL => Instruction::CALL(self.parse_operand()),
            lexer::Token::RET => Instruction::RET,
            _ => { panic!("Unexpected token found in parse_instruction(): {:?}", self.tok)}
        }

    }

    fn parse_operand(&mut self) -> lexer::Token {
        match self.peek() {
            lexer::Token::Label(_) | lexer::Token::Number(_) => { 
                self.eat_token();
                self.tok.clone() 
            },

            lexer::Token::NEWLINE | lexer::Token::EOF => { 
                lexer::Token::Number(0) 
            },

            _ => { panic!("Unexpected token found in parse_operand(): {:?}", self.tok)}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_instruction() {
        let mut p = Parser::new(vec![lexer::Token::ADD, lexer::Token::Number(10), lexer::Token::EOF]);
        let (prog, sym_table) = p.parse();
        assert_eq!(prog, vec![Instruction::ADD(lexer::Token::Number(10))]);
        assert_eq!(sym_table, HashMap::new());
    }

    #[test]
    fn test_parse_multiline_program() {
        let mut p = Parser::new(vec![
            lexer::Token::LDA, lexer::Token::Label(String::from("ONE")), lexer::Token::NEWLINE,
            lexer::Token::ADD, lexer::Token::Label(String::from("TWO")), lexer::Token::NEWLINE,
            lexer::Token::STA, lexer::Token::Label(String::from("RESULT")), lexer::Token::NEWLINE,
            lexer::Token::NEWLINE,
            lexer::Token::Label(String::from("ONE")), lexer::Token::DAT, lexer::Token::Number(1), lexer::Token::NEWLINE,
            lexer::Token::Label(String::from("TWO")), lexer::Token::DAT, lexer::Token::Number(2), lexer::Token::NEWLINE,
            lexer::Token::Label(String::from("RESULT")), lexer::Token::DAT, lexer::Token::EOF,
            ]);

        let (prog, sym_table) = p.parse();
        assert_eq!(prog, vec![
            Instruction::LDA(lexer::Token::Label(String::from("ONE"))),
            Instruction::ADD(lexer::Token::Label(String::from("TWO"))),
            Instruction::STA(lexer::Token::Label(String::from("RESULT"))),
            Instruction::DAT(lexer::Token::Number(1)),
            Instruction::DAT(lexer::Token::Number(2)),
            Instruction::DAT(lexer::Token::Number(0)),
            ]);

        assert_eq!(sym_table, HashMap::from([
            (String::from("ONE"), 3),
            (String::from("TWO"), 4),
            (String::from("RESULT"), 5),
        ]));
    }
}