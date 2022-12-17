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

    enum halfCarryOperationsMode {
        ADD,
        GREATER_THAN
    }

    impl Display for CPU {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Registers : {}", self.Registers)
        }
    }

    impl CPU {

        pub(crate) fn new(Decoder: Option<Decoder>) -> CPU {
            let mut Registers: Registers = Registers::new();
            let decoder = Decoder.unwrap_or(Decoder{
                data: vec![],
                address: 0,
                unprefixed_op_codes: Default::default(),
                prefixed_op_codes: Default::default()
            });
            CPU {
                Registers,
                Decoder: decoder
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
                    _ => {
                        println!("STATUS AFTER EXECUTING 0x{:04X} {}", address, self);
                    }
                };
            }
        }

        pub(crate) fn execute(&mut self, Instruction: Instruction) -> Result<(), Instruction> {
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
                    0x01 => {
                        CPU::ld_nn(self, Instruction.operands, "BC");
                    },
                    //0x03 INC BC
                    0x03 => {
                        CPU::inc_nn(self, "BC");
                    },
                    //0x0B DEC BC
                    0x0B => {
                        CPU::dec_nn(self, "BC");
                    },
                    //0x11 LD DE, d16
                    0x11 => {
                        CPU::ld_nn(self, Instruction.operands, "DE");
                    },
                    //0x13 INC DE
                    0x13 => {
                        CPU::inc_nn(self, "DE");
                    },
                    //0x18 JR e8
                    0x18 => {
                        /*let current_instruction = self.Registers.get_item("PC");
                        let to_add: u16 = Instruction.operands[0].value.unwrap();
                        self.Registers.set_item("PC", current_instruction + to_add)*/
                    },
                    //0x1B DEC DE
                    0x1B => {
                        CPU::dec_nn(self, "DE");
                    },
                    //0x21 LD HL, d16
                    0x21 => {
                        CPU::ld_nn(self, Instruction.operands, "HL");
                    },
                    //0x23 INC HL
                    0x23 => {
                        CPU::inc_nn(self, "HL");
                    },
                    //0x2B DEC HL
                    0x2B => {
                        CPU::dec_nn(self, "HL");
                    },
                    //0x31 LD SP, d16
                    0x31 => {
                        CPU::ld_nn(self, Instruction.operands, "SP");
                    },
                    //0x33 INC SP
                    0x33 => {
                        CPU::inc_nn(self, "SP");
                    },
                    //0x3B DEC SP
                    0x3B => {
                        CPU::dec_nn(self, "SP");
                    },
                    //0x3E LD a,d8
                    0x3E => {
                        let d8 = Instruction.operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
                        self.Registers.set_item("A", d8.value.expect("Operand d8 has no value"))
                    },
                    //0x40 LD B,B
                    0x40 => {
                        self.ld_r_r("B", "B")
                    },
                    //0x41 LD B,C
                    0x41 => {
                        self.ld_r_r("C", "B")
                    },
                    //0x42 LD B,D
                    0x42 => {
                        self.ld_r_r("D", "B")
                    },
                    //0x43 LD B,E
                    0x43 => {
                        self.ld_r_r("E", "B")
                    },
                    //0x44 LD B,H
                    0x44 => {
                        self.ld_r_r("H", "B")
                    },
                    //0x45 LD B,L
                    0x45 => {
                        self.ld_r_r("L", "B")
                    },
                    //0x46 LD B,(HL)
                    0x46 => {
                        //TODO should read from memory
                    },
                    //0x47 LD B,A
                    0x47 => {
                        self.ld_r_r("A", "B")
                    },
                    //0x48 LD C,B
                    0x48 => {
                        self.ld_r_r("B", "C")
                    },
                    //0x49 LD C,C
                    0x49 => {
                        self.ld_r_r("C", "C")
                    },
                    //0x4A LD C,D
                    0x4A => {
                        self.ld_r_r("D", "C")
                    },
                    //0x4B LD C,E
                    0x4B => {
                        self.ld_r_r("E", "C")
                    },
                    //0x4C LD C,H
                    0x4C => {
                        self.ld_r_r("H", "C")
                    },
                    //0x4D LD C,L
                    0x4D => {
                        self.ld_r_r("L", "C")
                    },
                    //0x4E LD C,(HL)
                    0x4E => {
                        //TODO should read from memory
                    },
                    //0x4F LD C,A
                    0x4F => {
                        self.ld_r_r("A", "C")
                    },
                    //0x50 LD D,B
                    0x50 => {
                        self.ld_r_r("B", "D")
                    },
                    //0x51 LD D,C
                    0x51 => {
                        self.ld_r_r("C", "D")
                    },
                    //0x52 LD D,D
                    0x52 => {
                        self.ld_r_r("D", "D")
                    },
                    //0x53 LD D,E
                    0x53 => {
                        self.ld_r_r("E", "D")
                    },
                    //0x54 LD D,H
                    0x54 => {
                        self.ld_r_r("H", "D")
                    },
                    //0x55 LD D,L
                    0x55 => {
                        self.ld_r_r("L", "D")
                    },
                    //0x56 LD D,(HL)
                    0x56 => {
                        //TODO should read from memory
                    },
                    //0x57 LD D,A
                    0x57 => {
                        self.ld_r_r("A", "D")
                    },
                    //0x58 LD E,B
                    0x58 => {
                        self.ld_r_r("B", "E")
                    },
                    //0x59 LD E,C
                    0x59 => {
                        self.ld_r_r("C", "E")
                    },
                    //0x5A LD E,D
                    0x5A => {
                        self.ld_r_r("D", "E")
                    },
                    //0x5B LD E,E
                    0x5B => {
                        self.ld_r_r("E", "E")
                    },
                    //0x5C LD E,H
                    0x5C => {
                        self.ld_r_r("H", "E")
                    },
                    //0x5D LD E,L
                    0x5D => {
                        self.ld_r_r("L", "E")
                    },
                    //0x5E LD E,(HL)
                    0x5E => {
                        //TODO should read from memory
                    },
                    //0x5F LD E,A
                    0x5F => {
                        self.ld_r_r("A", "E")
                    },
                    //0x60 LD H,B
                    0x60 => {
                        self.ld_r_r("B", "H")
                    },
                    //0x61 LD H,C
                    0x61 => {
                        self.ld_r_r("C", "H")
                    },
                    //0x62 LD H,D
                    0x62 => {
                        self.ld_r_r("D", "H")
                    },
                    //0x63 LD H,E
                    0x63 => {
                        self.ld_r_r("E", "H")
                    },
                    //0x64 LD H,H
                    0x64 => {
                        self.ld_r_r("H", "H")
                    },
                    //0x65 LD H,L
                    0x65 => {
                        self.ld_r_r("L", "H")
                    },
                    //0x66 LD H,(HL)
                    0x66 => {
                        //TODO should read from memory
                    },
                    //0x67 LD H,A
                    0x67 => {
                        self.ld_r_r("A", "H")
                    },
                    //0x68 LD L,B
                    0x68 => {
                        self.ld_r_r("B", "L")
                    },
                    //0x69 LD L,C
                    0x69 => {
                        self.ld_r_r("C", "L")
                    },
                    //0x6A LD L,D
                    0x6A => {
                        self.ld_r_r("D", "L")
                    },
                    //0x6B LD L,E
                    0x6B => {
                        self.ld_r_r("E", "L")
                    },
                    //0x6C LD L,H
                    0x6C => {
                        self.ld_r_r("H", "L")
                    },
                    //0x6D LD L,L
                    0x6D => {
                        self.ld_r_r("L", "L")
                    },
                    //0x6E LD L,(HL)
                    0x6E => {
                        //TODO should read from memory
                    },
                    //0x6F LD L,A
                    0x6F => {
                        self.ld_r_r("A", "L")
                    },

                    //0x78 LD A,B
                    0x78 => {
                        self.ld_r_r("B", "A")
                    },
                    //0x79 LD A,C
                    0x79 => {
                        self.ld_r_r("C", "A")
                    },
                    //0x7A LD A,D
                    0x7A => {
                        self.ld_r_r("D", "A")
                    },
                    //0x7B LD A,E
                    0x7B => {
                        self.ld_r_r("E", "A")
                    },
                    //0x7C LD A,H
                    0x7C => {
                        self.ld_r_r("H", "A")
                    },
                    //0x7D LD A,L
                    0x7D => {
                        self.ld_r_r("L", "A")
                    },
                    //0x7E LD A,(HL)
                    0x7E => {
                        //TODO should read from memory
                    },
                    //0x7F LD A,A
                    0x7F => {
                        self.ld_r_r("A", "A")
                    },
                    //0x80 ADD A,B
                    0x80 => {
                        self.add_a("B")
                    },
                    //0x81 ADD A,C
                    0x81 => {
                        self.add_a("C")
                    },
                    //0x82 ADD A,D
                    0x82 => {
                        self.add_a("D")
                    },
                    //0x83 ADD A,E
                    0x83 => {
                        self.add_a("E")
                    },
                    //0x84 ADD A,H
                    0x84 => {
                        self.add_a("H")
                    },
                    //0x85 ADD A,L
                    0x85 => {
                        self.add_a("L")
                    },
                    //0x87 ADD A,A
                    0x87 => {
                        self.add_a("A")
                    },
                    //0x88 ADC A,B
                    0x88 => {
                        self.adc_a("B")
                    },
                    //0x89 ADC A,C
                    0x89 => {
                        self.adc_a("C")
                    },
                    //0x8A ADC A,D
                    0x8A => {
                        self.adc_a("D")
                    },
                    //0x8B ADC A,E
                    0x8B => {
                        self.adc_a("E")
                    },
                    //0x8C ADC A,H
                    0x8C => {
                        self.adc_a("H")
                    },
                    //0x8D ADC A,L
                    0x8D => {
                        self.adc_a("L")
                    },
                    //0x8E ADC A,(HL)
                    0x8E => {
                        //TODO should read from memory
                    },
                    //0x8F ADC A,A
                    0x8F => {
                        self.adc_a("A")
                    },
                    //0x90 SUB B
                    0x90 => {
                        self.sub_a("B")
                    },
                    //0x91 SUB C
                    0x91 => {
                        self.sub_a("C")
                    },
                    //0x92 SUB D
                    0x92 => {
                        self.sub_a("D")
                    },
                    //0x93 SUB B
                    0x93 => {
                        self.sub_a("E")
                    },
                    //0x94 SUB H
                    0x94 => {
                        self.sub_a("H")
                    },
                    //0x90 SUB L
                    0x95 => {
                        self.sub_a("L")
                    },
                    //0x96 SUB (HL)
                    0x96 => {
                        //TODO should read from memory
                    },
                    //0x97 SUB A
                    0x97 => {
                        self.sub_a("A")
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

        pub(crate) fn add_a(&mut self, to_add: &str){
            let value_to_add = self.Registers.get_item(to_add);
            let current_value = self.Registers.get_item("A");
            let result = current_value + value_to_add;
            self.Registers.set_item("c", (result > 0xFF) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, value_to_add, 0, halfCarryOperationsMode::ADD) as u16);

            let rounded_result = result & 0xFF;

            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (rounded_result == 0) as u16);
            self.Registers.set_item("A", rounded_result);
        }

        pub(crate) fn adc_a(&mut self, to_add: &str){
            let to_add = self.Registers.get_item(to_add);
            let current_value = self.Registers.get_item("A");
            let carry = self.Registers.get_item("c");
            let result = to_add + current_value + carry;
            let rounded_result = result & 0xFF;

            self.Registers.set_item("c", (result > 0xFF) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, to_add, carry, halfCarryOperationsMode::ADD) as u16);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (rounded_result == 0) as u16);
            self.Registers.set_item("A", rounded_result);
        }

        pub(crate) fn sub_a(&mut self, to_sub: &str) {
            let to_sub = self.Registers.get_item(to_sub);
            let current_value = self.Registers.get_item("A");
            let rounded_result = (current_value - to_sub) & 0xFF;
            self.Registers.set_item("c", (to_sub > current_value) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, to_sub, 0, halfCarryOperationsMode::GREATER_THAN) as u16);
            self.Registers.set_item("n", 1);
            self.Registers.set_item("z", (rounded_result == 0) as u16);
            self.Registers.set_item("A", rounded_result);
        }

            //half carry is carry calculated on the first half of a byte (from 3rd bit)
        fn calculate_half_carry(value: u16, second_operator: u16, carry: u16, mode: halfCarryOperationsMode) -> bool{
            let rounded_value = value & 0xF;
            let rounded_second_operator = second_operator & 0xF;
            match mode {
                halfCarryOperationsMode::ADD => {
                    (rounded_second_operator + rounded_value + carry) > 0xF
                }
                halfCarryOperationsMode::GREATER_THAN => {
                    rounded_second_operator > rounded_value
                }
            }
        }
    }
}