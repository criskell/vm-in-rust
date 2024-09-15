use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    program: Vec<u8>,
    pc: usize,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;

        while !is_done {
            is_done = self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            }

            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }

            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }

            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }

            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }

            Opcode::HLT => {
                println!("HLT encountered.");

                return true;
            }

            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }

            Opcode::JMPF => {
                let register = self.next_8_bits() as usize;
                let target = self.pc as i32 + self.registers[register];
                self.pc = target as usize;
            }

            Opcode::JMPB => {
                let register = self.next_8_bits() as usize;
                println!("{} {}", register, self.pc);
                let target = self.pc as i32 - self.registers[register];
                self.pc = target as usize;
            }

            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 == register2;

                self.next_8_bits();
            }

            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 != register2;

                self.next_8_bits();
            }

            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 > register2;

                self.next_8_bits();
            }

            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 < register2;

                self.next_8_bits();
            }

            Opcode::GTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 >= register2;

                self.next_8_bits();
            }

            Opcode::LTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = register1 <= register2;

                self.next_8_bits();
            }

            Opcode::JEQ => {
                let register = self.registers[self.next_8_bits() as usize];

                if self.equal_flag {
                    self.pc = register as usize;

                    return false;
                }

                self.next_16_bits();
            }

            Opcode::JNEQ => {
                let register = self.registers[self.next_8_bits() as usize];

                if !self.equal_flag {
                    self.pc = register as usize;

                    return false;
                }

                self.next_16_bits();
            }

            _ => {
                println!("Unrecognized opcode found! Terminating!");

                return true;
            }
        }

        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
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
        let program = vec![5, 0, 0, 0];

        test_vm.program = program;
        test_vm.run();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let program = vec![200, 0, 0, 0];

        test_vm.program = program;
        test_vm.run();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        let program = vec![0, 0, (500 >> 8) as u8, (500 & 0xFF) as u8];

        test_vm.program = program;
        test_vm.run();

        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        let program = vec![0, 0, 0, 10, 0, 1, 0, 15, 1, 0, 1, 2];

        test_vm.program = program;
        test_vm.run();

        assert_eq!(test_vm.registers[2], 25);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        let program = vec![0, 0, 0, 15, 0, 1, 0, 10, 2, 0, 1, 2];

        test_vm.program = program;
        test_vm.run();

        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = VM::new();
        let program = vec![0, 0, 0, 15, 0, 1, 0, 10, 3, 0, 1, 2];

        test_vm.program = program;
        test_vm.run();

        assert_eq!(test_vm.registers[2], 150);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = VM::new();
        let program = vec![0, 0, 0, 8, 0, 1, 0, 5, 4, 0, 1, 2];

        test_vm.program = program;
        test_vm.run();

        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 3);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_instruction();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 10;
        test_vm.program = vec![7, 0, 0, 0];
        test_vm.run_instruction();

        assert_eq!(test_vm.pc, 12);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::new();

        test_vm.program = vec![0, 0, 0, 0, 8, 0, 0, 0];

        test_vm.run_instruction();

        test_vm.registers[0] = 5;

        test_vm.run_instruction();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();

        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0, 0];

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;

        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[1] = 16;
        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::new();

        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0, 0];

        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;

        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[1] = 11;
        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = VM::new();

        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0];

        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;

        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, false);

        test_vm.registers[0] = 10;
        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, false);

        test_vm.registers[0] = 11;
        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = VM::new();

        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];

        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;

        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[0] = 10;
        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, false);

        test_vm.registers[0] = 11;
        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gtq_opcode() {
        let mut test_vm = VM::new();

        test_vm.program = vec![15, 0, 1, 0, 15, 0, 1, 0, 15, 0, 1, 0];

        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;

        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, false);

        test_vm.registers[0] = 10;
        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[0] = 11;
        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_ltq_opcode() {
        let mut test_vm = VM::new();

        test_vm.program = vec![16, 0, 1, 0, 16, 0, 1, 0, 16, 0, 1, 0];

        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;

        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[0] = 10;
        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[0] = 11;
        test_vm.run_instruction();

        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();

        test_vm.program = vec![9, 0, 1, 0, 10, 2, 0, 0];

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.registers[2] = 0;

        test_vm.run_instruction();
        test_vm.run_instruction();

        assert_eq!(test_vm.pc, 0);

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 11;

        test_vm.run_instruction();
        test_vm.run_instruction();

        assert_eq!(test_vm.pc, 8);
    }

    #[test]
    fn test_jneq_opcode() {
        let mut test_vm = VM::new();

        test_vm.program = vec![9, 0, 1, 0, 11, 2, 0, 0];

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 11;
        test_vm.registers[2] = 0;

        test_vm.run_instruction();
        test_vm.run_instruction();

        assert_eq!(test_vm.pc, 0);

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;

        test_vm.run_instruction();
        test_vm.run_instruction();

        assert_eq!(test_vm.pc, 8);
    }
}
