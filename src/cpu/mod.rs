mod registers;

pub mod CPU{
    use std::fmt::{Display, Formatter};
    use std::io::Error;
    use crate::cpu::registers::Registers::Registers;
    use crate::decoder::decoder::Decoder;
    use crate::op_codes_parser::op_codes_parser::{Instruction, Operand};

    pub struct CPU {
        pub(crate) Registers: Registers,
        pub(crate) Decoder: Decoder,
    }

    impl Display for CPU {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Registers : {}", self.Registers)
        }
    }

    impl CPU {
        pub(crate) fn new(Decoder: Decoder) -> CPU {
            let mut Registers: Registers = Registers::new();

            CPU {
                Registers,
                Decoder
            }
        }

        pub(crate) fn run(&mut self) {
            loop {
                let address = self.Registers.get_item("PC");
                let (next_address, instruction) = self.Decoder.decode(address as i32);
                self.Registers.set_item("PC", next_address as u16);
                match self.execute(instruction) {
                    Err(instruction) => {
                        self.Decoder.disassemble(address as i32, 12);
                        println!("{}", self);
                        panic!("⚠️NOT IMPLEMENTED⚠️ {:?}", instruction)
                    },
                    _ => {}
                };
            }
        }

        pub(crate) fn execute(&mut self, Instruction: Instruction) -> std::result::Result<(), Instruction> {
            if Instruction.prefixed {
                match Instruction.opcode {
                    0 => {
                        //TODO RLC B
                    }
                    _ => return Err(Instruction)
                }
            }else{
                match Instruction.opcode {
                    //0x00 NOP
                    0 => {},
                    //0x01 LD BC, d16
                    1 => {
                        CPU::ld_nn(self, Instruction.operands, "BC");
                    },
                    //0x03 INC BC
                    3 => {
                        CPU::inc_nn(self, "BC");
                    },
                    //0x0B DEC BC
                    11 => {
                        CPU::dec_nn(self, "BC");
                    },
                    //0x11 LD DE, d16
                    17 => {
                        CPU::ld_nn(self, Instruction.operands, "DE");
                    },
                    //0x13 INC DE
                    19 => {
                        CPU::inc_nn(self, "DE");
                    },
                    //0x18 JR e8
                    24 => {
                        /*let current_instruction = self.Registers.get_item("PC");
                        let to_add: u16 = Instruction.operands[0].value.unwrap();
                        self.Registers.set_item("PC", current_instruction + to_add)*/
                    },
                    //0x1B DEC DE
                    27 => {
                        CPU::dec_nn(self, "DE");
                    },
                    //0x21 LD HL, d16
                    33 => {
                        CPU::ld_nn(self, Instruction.operands, "HL");
                    },
                    //0x23 INC HL
                    35 => {
                        CPU::inc_nn(self, "HL");
                    },
                    //0x2B DEC HL
                    43 => {
                        CPU::dec_nn(self, "HL");
                    },
                    //0x31 LD SP, d16
                    49 => {
                        CPU::ld_nn(self, Instruction.operands, "SP");
                    },
                    //0x33 INC SP
                    51 => {
                        CPU::inc_nn(self, "SP");
                    },
                    //0x3B DEC SP
                    59 => {
                        CPU::dec_nn(self, "SP");
                    },
                    //0x3E LD a,d8
                    62 => {
                        /*let d8 = Instruction.operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
                        self.Registers.set_item("A", d8.value.expect("Operand d8 has no value"))*/
                    },
                    //0x40 LD B,B
                    64 => {
                        self.ld_r_r("B", "B")
                    },
                    //0x41 LD B,C
                    65 => {
                        self.ld_r_r("C", "B")
                    },
                    //0x42 LD B,D
                    66 => {
                        self.ld_r_r("D", "B")
                    },
                    //0x43 LD B,E
                    67 => {
                        self.ld_r_r("E", "B")
                    },
                    //0x44 LD B,H
                    68 => {
                        self.ld_r_r("H", "B")
                    },
                    //0x45 LD B,L
                    69 => {
                        self.ld_r_r("L", "B")
                    },
                    //0x46 LD B,(HL)
                    70 => {
                        //TODO should read from memory
                    },
                    //0x47 LD B,A
                    71 => {
                        self.ld_r_r("A", "B")
                    },
                    //0x48 LD C,B
                    72 => {
                        self.ld_r_r("B", "C")
                    },
                    //0x49 LD C,C
                    73 => {
                        self.ld_r_r("C", "C")
                    },
                    //0x4A LD C,D
                    74 => {
                        self.ld_r_r("D", "C")
                    },
                    //0x4B LD C,E
                    75 => {
                        self.ld_r_r("E", "C")
                    },
                    //0x4C LD C,H
                    76 => {
                        self.ld_r_r("H", "C")
                    },
                    //0x4D LD C,L
                    77 => {
                        self.ld_r_r("L", "C")
                    },
                    //0x4E LD C,(HL)
                    78 => {
                        //TODO should read from memory
                    },
                    //0x4F LD C,A
                    79 => {
                        self.ld_r_r("A", "C")
                    },
                    //0x50 LD D,B
                    80 => {
                        self.ld_r_r("B", "D")
                    },
                    //0x51 LD D,C
                    81 => {
                        self.ld_r_r("C", "D")
                    },
                    //0x52 LD D,D
                    82 => {
                        self.ld_r_r("D", "D")
                    },
                    //0x53 LD D,E
                    83 => {
                        self.ld_r_r("E", "D")
                    },
                    //0x54 LD D,H
                    84 => {
                        self.ld_r_r("H", "D")
                    },
                    //0x55 LD D,L
                    85 => {
                        self.ld_r_r("L", "D")
                    },
                    //0x56 LD D,(HL)
                    86 => {
                        //TODO should read from memory
                    },
                    //0x57 LD D,A
                    87 => {
                        self.ld_r_r("A", "D")
                    },
                    88 => {
                        //TODO
                    },
                    //DI
                    243 => {
                        //TODO
                    },
                    _ => return Err(Instruction)
                }
            }

            Ok(())
        }

        fn ld_nn(&mut self, Operands: Vec<Operand>, name: &str){
            let d16 = Operands.into_iter().find(|operand| operand.name == "d16").expect("Operand d16 not found");
            self.Registers.set_item(name, d16.value.expect("Operand d16 has no value"))
        }

        fn inc_nn(&mut self, name: &str){
            let mut currentValue = self.Registers.get_item(name);
            currentValue += 1;
            self.Registers.set_item(name, currentValue);
        }

        fn dec_nn(&mut self, name: &str){
            let mut currentValue = self.Registers.get_item(name);
            currentValue -= 1;
            self.Registers.set_item(name, currentValue);
        }

        fn ld_r_r(&mut self, from: &str, to: &str){
            let fromValue = self.Registers.get_item(from);
            self.Registers.set_item(to, fromValue);
        }
    }
}