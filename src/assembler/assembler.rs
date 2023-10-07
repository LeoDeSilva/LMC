use std::collections::HashMap;
use crate::assembler::lexer;
use crate::assembler::parser;

pub struct Compiler {
    program: Vec<parser::Instruction>,
    symbol_table: HashMap<String, u16>,
} 

impl Compiler {
    pub fn new(program: Vec<parser::Instruction>, symbol_table: HashMap<String, u16>) -> Self {
        Compiler { program: program, symbol_table: symbol_table } 
    }

    pub fn compile(&mut self) -> Vec<u8>{
        let mut out: Vec<u8> = vec![];
        for instruction in &self.program {
            let bin_instruction = self.compile_instruction(instruction.clone());
            out.extend(bin_instruction);
        }

        out
    }

    fn compile_instruction(&self, instruction: parser::Instruction) -> Vec<u8> {
        let opcode_map: fn(&parser::Instruction) -> u8 = |instruction: &parser::Instruction| match instruction {
            parser::Instruction::HLT    => 0b0000,
            parser::Instruction::ADD(_) => 0b0001,
            parser::Instruction::SUB(_) => 0b0010,
            parser::Instruction::LDA(_) => 0b0011,
            parser::Instruction::STA(_) => 0b0100,
            parser::Instruction::BRA(_) => 0b0101,
            parser::Instruction::BRZ(_) => 0b0110,
            parser::Instruction::BGT(_) => 0b0111,
            parser::Instruction::BLT(_) => 0b1011,
            parser::Instruction::INP    => 0b1000,
            parser::Instruction::OUT    => 0b1001,
            parser::Instruction::OTC    => 0b1010,
            parser::Instruction::DAT(_) => 0b1100,
            parser::Instruction::CALL(_) => 0b1101,
            parser::Instruction::RET => 0b1110,
        };

        let operand_map = |instruction: &parser::Instruction| match instruction {
            parser::Instruction::HLT | parser::Instruction::INP | 
            parser::Instruction::OUT | parser::Instruction::OTC |
            parser::Instruction::RET => 0,

            parser::Instruction::ADD(operand) | parser::Instruction::SUB(operand) | 
            parser::Instruction::LDA(operand) | parser::Instruction::STA(operand) | 
            parser::Instruction::BRA(operand) | parser::Instruction::BRZ(operand) | 
            parser::Instruction::BGT(operand) | parser::Instruction::BLT(operand) |
            parser::Instruction::DAT(operand) | parser::Instruction::CALL(operand) 
            => self.compile_operand(operand.clone()),
        };


        let bin_opcode = opcode_map(&instruction);
        let bin_operand = operand_map(&instruction);

        vec![bin_opcode, (bin_operand >> 8) as u8, bin_operand as u8]
    }

    fn compile_operand(&self, operand: lexer::Token) -> u16 {
        // Label: replace with addr*3 (3 byte instructions) e.g. 0=0, 1=3, 6=18,
        match operand {
            lexer::Token::Number(value) => { return value }
            lexer::Token::Label(identifier) => {
                // Lookup, *3, u16
                let address = self.symbol_table.get(&identifier).expect(&format!("undefined label found in compiler, got: {}", identifier));
                return address*3;
            }

            _ => { panic!("unexpected operand found in compile_operand(): {:?}", operand)}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembler() {
        let mut c = Compiler::new(vec![
            parser::Instruction::LDA(lexer::Token::Label(String::from("ONE"))),
            parser::Instruction::DAT(lexer::Token::Number(1))
        ], 
        HashMap::from([
            (String::from("ONE"), 1),
        ]));

        let bin = c.compile();
        assert_eq!(bin, vec![
            3, 0, 3, 12, 0, 1
        ])
    }

    #[test]
    #[should_panic]
    fn test_undefined_label() {
        let mut c = Compiler::new(vec![
            parser::Instruction::LDA(lexer::Token::Label(String::from("ONE"))),
            parser::Instruction::DAT(lexer::Token::Number(1))
        ], 
        HashMap::new());

        c.compile();
    }

    #[test]
    fn test_empty() {
        let mut c = Compiler::new(vec![], HashMap::new());
        let bin = c.compile();
        assert_eq!(bin, vec![])
    }
}