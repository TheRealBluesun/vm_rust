#[derive(Debug, PartialEq)]
pub enum Opcode {
  HLT,
  LOAD,
  ADD,
  MUL,
  DIV,
  JMP,  // absolute jmp
  JMPF, // rel jmp forward
  JMPB, // rel jmp backward
  EQ,   // isequal
  JEQ,  // Jump if EQual
  JNE,  // Jump if Not Equal
  NOP,  // NOOP
  IGL,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
  opcode: Opcode,
}

impl Instruction {
  pub fn new(opcode: Opcode) -> Instruction {
    Instruction { opcode: opcode }
  }
}

impl From<u8> for Opcode {
  fn from(v: u8) -> Self {
    match v {
      0 => return Opcode::HLT,
      1 => return Opcode::LOAD,
      2 => return Opcode::ADD,
      3 => return Opcode::MUL,
      4 => return Opcode::DIV,
      5 => return Opcode::JMP,
      6 => return Opcode::JMPF,
      7 => return Opcode::JMPB,
      8 => return Opcode::EQ,
      9 => return Opcode::JEQ,
      10 => return Opcode::JNE,
      11 => return Opcode::NOP,
      _ => return Opcode::IGL,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_hlt() {
    let opcode = Opcode::HLT;
    assert_eq!(opcode, Opcode::HLT);
  }

  #[test]
  fn test_create_instruction() {
    let instruction = Instruction::new(Opcode::HLT);
    assert_eq!(instruction.opcode, Opcode::HLT);
  }
}
