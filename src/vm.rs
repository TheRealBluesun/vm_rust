use instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
        }
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
                return false;
            },
            Opcode::ADD => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 + val2;
                return false;
            },
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            },
            _ => {
                println!("Unrecognized opcode found! Terminating!");
                return true;
            }
        }
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = (self.program[self.pc] as u16) << 8 | (self.program[self.pc + 1] as u16);
        self.pc += 1;
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
        let val1 = 0x02 as u8;
        let val2 = 0x0F as u8;
        let mut test_vm = VM::new();
        let test_bytes = vec![
            0x01, 0x0, 0x0, val1, 0x01, 0x1, 0x0, val2, 0x02, 0x0, 0x01, 0x2,
        ];
        test_vm.program = test_bytes;
        test_vm.run();
        println!("{:?}", test_vm.registers);
        println!("{:?}", test_vm.program);
        assert_eq!(test_vm.registers[0x02] as u8, val1 + val2);
    }
}
