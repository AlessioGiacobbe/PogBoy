mod registers;

pub mod CPU{
    use std::fmt::{Display, Formatter};
    use std::io::Error;
    use crate::cpu::registers::Registers::Registers;
    use crate::decoder::decoder::Decoder;
    use crate::op_codes_parser::op_codes_parser::{Instruction};

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
                        println!("RLC B");
                    }

                    _ => return Err(Instruction)
                }
            }else{
                match Instruction.opcode {
                    0 => println!("NOPE!"),
                    //LD BC, d16
                    1 => {
                        let d16 = Instruction.operands.into_iter().find(|operand| operand.name == "d16").expect("Operand d16 not found");
                        self.Registers.set_item("BC", d16.value.expect("Operand d16 has no value"))
                    },
                    //JR e
                    24 => {
                        let current_instruction = self.Registers.get_item("PC");
                        //TODO OperandValue could be replaced with u16? (u8 values can be casted safely)
                        let to_add: u16 = Instruction.operands[0].value.unwrap();

                        self.Registers.set_item("PC", current_instruction + to_add)
                    },
                    //LD a,d8
                    62 => {
                        let d8 = Instruction.operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
                        self.Registers.set_item("A", d8.value.expect("Operand d8 has no value"))
                    },
                    //DI
                    243 => {
                        //TODO
                    },
                    87 => {
                        //TODO
                    },
                    88 => {
                        //TODO
                    },
                    _ => return Err(Instruction)
                }
            }

            Ok(())
        }
    }
}