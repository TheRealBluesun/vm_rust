use instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    flag_isequal: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            flag_isequal: false,
        }
    }

    pub fn reset(&mut self) {
        self.registers = [0; 32];
        self.pc = 0;
        self.remainder = 0;
        self.flag_isequal = false;
    }

    pub fn run(&mut self) {
        let mut is_done: bool = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let val = self.next_16_bits() as u16;
                self.registers[register] = val as i32;
            }
            Opcode::ADD => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 + val2;
            }
            Opcode::MUL => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 * val2;
            }
            Opcode::DIV => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 / val2;
                self.remainder = (val1 % val2) as u32;
            }
            Opcode::JMP => {
                let addr = self.registers[self.next_8_bits() as usize];
                self.pc = addr as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;
            }
            Opcode::EQ => {
                self.flag_isequal = self.registers[self.next_8_bits() as usize]
                    == self.registers[self.next_8_bits() as usize];
            }
            Opcode::JEQ => {
                if self.flag_isequal{
                    self.pc = self.registers[self.next_8_bits() as usize] as usize;
                }
            }
            Opcode::JNE => {
                if !self.flag_isequal{
                    self.pc = self.registers[self.next_8_bits() as usize] as usize;
                }
            }
            Opcode::NOP => {
                
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
            _ => {
                println!("Unrecognized opcode found! Terminating!");
                return true;
            }
        }
        return false;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = (self.program[self.pc] as u16) << 8 | (self.program[self.pc + 1] as u16);
        self.pc += 2;
        return result;
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let val = 0xF as u8;
        let mut test_vm = VM::new();
        let test_bytes = vec![0x01, 0x1, 0x0, val, 0x0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[0x1] as u8, val);
    }

    #[test]
    fn test_opcode_add() {
        for val2 in 0..0xFF {
            let val1 = 0x02 as u8;
            //let val2 = 0x0F as u8;
            let mut test_vm = VM::new();
            let test_bytes = vec![
                0x01, 0x0, 0x0, val1, 0x01, 0x1, 0x0, val2, 0x02, 0x0, 0x01, 0x2, 0x0,
            ];
            test_vm.program = test_bytes;
            test_vm.run();
            println!("{:?}", test_vm.registers);
            println!("{:?}", test_vm.program);
            assert_eq!(test_vm.registers[0x02] as u32, val1 as u32 + val2 as u32);
        }
    }

    #[test]
    fn test_opcode_mul() {
        for val2 in 0..0xFF {
            let val1 = 0x02 as u8;
            let mut test_vm = VM::new();
            let test_bytes = vec![
                0x01, 0x0, 0x0, val1, 0x01, 0x1, 0x0, val2, 0x03, 0x0, 0x01, 0x2, 0x0,
            ];
            test_vm.program = test_bytes;
            test_vm.run();
            println!("{:?}", test_vm.registers);
            println!("{:?}", test_vm.program);
            assert_eq!(test_vm.registers[0x02] as u32, val1 as u32 * val2 as u32);
        }
    }

    #[test]
    fn test_opcode_div() {
        for val2 in 1..0xFF {
            let val1 = 0x02 as u8;
            let mut test_vm = VM::new();
            let test_bytes = vec![
                0x01, 0x0, 0x0, val1, 0x01, 0x1, 0x0, val2, 0x04, 0x0, 0x01, 0x2, 0x0,
            ];
            test_vm.program = test_bytes;
            test_vm.run();
            println!("{:?}", test_vm.registers);
            println!("{:?}", test_vm.program);
            assert_eq!(test_vm.registers[0x02] as u32, val1 as u32 / val2 as u32);
            assert_eq!(test_vm.remainder, val1 as u32 % val2 as u32);
        }
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 3;
        test_vm.program = vec![5, 0, 0, 0];
        test_vm.run();
        // PC should be four (4).
        // JMP 0 --> PC = 2
        // (now we're at PC = 3, so offset 0x3, which is 0 = HLT)
        // We decode_opcode(), so pc +=1 == 4, now we halt
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![6, 0, 0, 0, 6, 0, 0, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.pc, 9);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.registers[1] = 1;
        test_vm.registers[2] = 0xFF;
        test_vm.program = vec![8, 0, 1];
        test_vm.run();
        assert_eq!(test_vm.flag_isequal, true);
        test_vm.reset();
        test_vm.program = vec![8, 0, 2];
        assert_eq!(test_vm.flag_isequal, false);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 4;
        test_vm.program = vec![9, 0, 0, 0, 0];
        test_vm.flag_isequal = true;
        test_vm.run();
        assert_eq!(test_vm.pc, 5);
    }

    #[test]
    fn test_jne_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 4;
        test_vm.program = vec![10, 0, 0, 0, 0];
        test_vm.flag_isequal = false;
        test_vm.run();
        assert_eq!(test_vm.pc, 5);
    }

    #[test]
    fn test_nop_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![11, 11, 11, 11, 11, 0];
        test_vm.run();
        assert_eq!(test_vm.pc, 6);
    }   
}
