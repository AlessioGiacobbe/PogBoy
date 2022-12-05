mod registers;

pub mod CPU{
    use crate::cpu::registers::Registers::Registers;
    use crate::decoder::decoder::Decoder;
    use crate::op_codes_parser::op_codes_parser::{Instruction, OperandValue};

    pub struct CPU {
        pub(crate) Registers: Registers,
        pub(crate) Decoder: Decoder,
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
                self.execute(instruction)
            }
        }

        pub(crate) fn execute(&mut self, Instruction: Instruction) {
            match Instruction.opcode {
                0 => println!("NOPE!"),
                //JR e
                24 => {
                    let current_instruction = self.Registers.get_item("PC");
                    //TODO OperandValue could be replaced with u16? (u8 values can be casted safely)
                    let to_add: u16 = match Instruction.operands[0].value.clone().unwrap() {
                        OperandValue::U8(value) => value as u16,
                        OperandValue::U16(value) => value
                    };

                    self.Registers.set_item("PC", current_instruction + to_add)
                },
                //DI
                243 => {
                    //TODO
                },
                _ => panic!("⚠️NOT IMPLEMENTED⚠️ opcode : {}, mnemonic : {}, operands : {:?}", Instruction.opcode, Instruction.mnemonic, Instruction.operands )
            }
        }
    }
}