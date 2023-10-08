use std::vec;

pub struct Machine {
    pub memory: Vec<u8>,
    stack: Vec<u16>,
    pc: u16,
    acc: u16,
    hlt: bool,

    n: bool, // Negative flag
    c: bool, // Carry flag
}

impl Machine {
    pub fn new() -> Self {
        let memory: Vec<u8> = vec![0; 0xffff];
        Machine { memory: memory, stack: vec![], pc: 0, acc: 0, hlt: false, c: false, n: false }
    }


    pub fn load(&mut self, program: Vec<u8>) {
        for (position, byte) in program.iter().enumerate() {
            self.memory[position] = *byte;
        }
    }


    pub fn emulate(&mut self) {
        while !self.hlt {
            self.clock_cycle();
        }
    }


    fn clock_cycle(&mut self) {
        let opcode: u8= self.memory[self.pc as usize];
        let operand_u8: Vec<u8> = self.memory[(self.pc as usize + 1)..(self.pc as usize + 3)].to_vec();
        let operand: u16 = ((operand_u8[0] as u16) << 8) | operand_u8[1] as u16;
        self.pc += 3;

        match opcode {
            0b0000 => { self.hlt = true; },  // HLT
            0b0001 => { // ADD
                self.c = self.acc + self.address_operand(operand) > u16::MAX;
                self.n = false;

                self.acc = u16::wrapping_add(self.acc, self.address_operand(operand));
            },  
            0b0010 => { // SUB
                self.n = self.address_operand(operand) > self.acc;
                self.c = false;

                self.acc = u16::wrapping_sub(self.acc, self.address_operand(operand));
                // self.acc -= self.address_operand(operand); 
            },
            0b0011 => { self.acc = self.address_operand(operand); },   // LDA
            0b0100 => { // STA
                self.memory[operand as usize + 1] = (self.acc >> 8) as u8; 
                self.memory[operand as usize + 2] = self.acc as u8;
            },  
            0b0101 => { self.pc = operand; },  // BRA
            0b0110 => { self.pc = if self.acc == 0 { operand } else { self.pc }  },  // BRZ
            0b0111 => { self.pc = if self.acc > 0 && !self.n { operand } else { self.pc }  },  // BGT
            0b1011 => { self.pc = if self.n { operand } else { self.pc }  },  // BLT
            0b1000 => {
                let mut rl = rustyline::DefaultEditor::new().unwrap();
                let line = rl.readline("").unwrap();

                if !(line.trim().parse::<u16>().is_ok()) && line.trim().len() == 1 {
                    self.acc = line.chars().nth(0).unwrap() as u16;
                } else {
                    self.acc = line.trim().parse::<u16>().unwrap();
                }
            },  // INP
            0b1001 => { println!("{:?}", self.acc) },  // OUT
            0b1010 => { println!("{}", char::from_u32(self.acc as u32).expect("invalid ASCII char")); },  // OTC
            0b1100 => {},

            0b1101 => { // CALL
                self.stack.push(self.pc);
                self.pc = operand;
            },

            0b1110 => { // RET
                self.pc = self.stack.pop().unwrap();
            },
            _ => { panic!("Unexpected opcode found in emulator") }
        }

    }


    fn address_operand(&mut self, operand: u16) -> u16 {
        let operand_u8: Vec<u8> = self.memory[(operand as usize + 1)..(operand as usize + 3)].to_vec();
        ((operand_u8[0] as u16) << 8) | operand_u8[1] as u16
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_clock_cycle() {
        let mut m = Machine::new();
        m.load(vec![1, 0, 3, 0, 0, 3]);
        m.clock_cycle();
        assert_eq!(m.pc, 3);
        assert_eq!(m.acc, 3);
    }
}