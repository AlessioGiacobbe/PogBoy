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
                        self.Decoder.disassemble(address as i32 - 12, 25, address as i32);
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
                    //0x58 LD E,B
                    88 => {
                        self.ld_r_r("B", "E")
                    },
                    //0x59 LD E,C
                    89 => {
                        self.ld_r_r("C", "E")
                    },
                    //0x5A LD E,D
                    90 => {
                        self.ld_r_r("D", "E")
                    },
                    //0x5B LD E,E
                    91 => {
                        self.ld_r_r("E", "E")
                    },
                    //0x5C LD E,H
                    92 => {
                        self.ld_r_r("H", "E")
                    },
                    //0x5D LD E,L
                    93 => {
                        self.ld_r_r("L", "E")
                    },
                    //0x5E LD E,(HL)
                    94 => {
                        //TODO should read from memory
                    },
                    //0x5F LD E,A
                    95 => {
                        self.ld_r_r("A", "E")
                    },
                    //0x60 LD H,B
                    96 => {
                        self.ld_r_r("B", "H")
                    },
                    //0x61 LD H,C
                    97 => {
                        self.ld_r_r("C", "H")
                    },
                    //0x62 LD H,D
                    98 => {
                        self.ld_r_r("D", "H")
                    },
                    //0x63 LD H,E
                    99 => {
                        self.ld_r_r("E", "H")
                    },
                    //0x64 LD H,H
                    100 => {
                        self.ld_r_r("H", "H")
                    },
                    //0x65 LD H,L
                    101 => {
                        self.ld_r_r("L", "H")
                    },
                    //0x66 LD H,(HL)
                    102 => {
                        //TODO should read from memory
                    },
                    //0x67 LD H,A
                    103 => {
                        self.ld_r_r("A", "H")
                    },
                    //0x68 LD L,B
                    104 => {
                        self.ld_r_r("B", "L")
                    },
                    //0x69 LD L,C
                    105 => {
                        self.ld_r_r("C", "L")
                    },
                    //0x6A LD L,D
                    106 => {
                        self.ld_r_r("D", "L")
                    },
                    //0x6B LD L,E
                    107 => {
                        self.ld_r_r("E", "L")
                    },
                    //0x6C LD L,H
                    108 => {
                        self.ld_r_r("H", "L")
                    },
                    //0x6D LD L,L
                    109 => {
                        self.ld_r_r("L", "L")
                    },
                    //0x6E LD L,(HL)
                    110 => {
                        //TODO should read from memory
                    },
                    //0x6F LD L,A
                    111 => {
                        self.ld_r_r("A", "L")
                    },

                    //0x78 LD A,B
                    120 => {
                        self.ld_r_r("B", "A")
                    },
                    //0x79 LD A,C
                    121 => {
                        self.ld_r_r("C", "A")
                    },
                    //0x7A LD A,D
                    122 => {
                        self.ld_r_r("D", "A")
                    },
                    //0x7B LD A,E
                    123 => {
                        self.ld_r_r("E", "A")
                    },
                    //0x7C LD A,H
                    124 => {
                        self.ld_r_r("H", "A")
                    },
                    //0x7D LD A,L
                    125 => {
                        self.ld_r_r("L", "A")
                    },
                    //0x7E LD A,(HL)
                    126 => {
                        //TODO should read from memory
                    },
                    //0x7F LD A,A
                    127 => {
                        self.ld_r_r("A", "A")
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