mod registers;

pub mod CPU{
    use crate::cpu::registers::Registers::Registers;
    use crate::decoder::decoder::Decoder;
    use crate::op_codes_parser::op_codes_parser::Instruction;

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

        pub(crate) fn execute(&self, Instruction: Instruction) {
            match Instruction.mnemonic.as_str() {
                "NOP" => println!("NOPE!"),
                _ => panic!("Instruction {} ({}) not implemented", Instruction.mnemonic, Instruction )
            }
        }
    }
}