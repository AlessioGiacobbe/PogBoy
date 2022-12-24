mod registers;

pub mod CPU{
    use std::fmt::{Display, Formatter};
    use crate::cpu::registers::Registers::Registers;
    use crate::decoder::decoder::Decoder;
    use crate::op_codes_parser::op_codes_parser::{Instruction, Operand};

    pub struct CPU {
        pub(crate) Registers: Registers,
        pub(crate) Decoder: Decoder,
    }

    enum HalfCarryOperationsMode {
        Add,        //8bit
        AddWords,   //16bit
        GreaterThan,
        Increment,
        Decrement,
    }

    impl Display for CPU {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Registers : {}", self.Registers)
        }
    }

    impl CPU {

        pub(crate) fn new(Decoder: Option<Decoder>) -> CPU {
            let Registers: Registers = Registers::new();
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
                        panic!("⚠️{:#04x} NOT IMPLEMENTED⚠️ {:?}", instruction.opcode, instruction)
                    },
                    _ => {
                        println!("STATUS AFTER EXECUTING 0x{:04X} {}", address, self);
                    }
                };
            }
        }

        pub(crate) fn execute(&mut self, instruction: Instruction) -> Result<(), Instruction> {
            if instruction.prefixed {
                match instruction.opcode {
                    0 => {} //TODO 0x0 RLC B
                    _ => return Err(instruction)
                }
            }else{
                match instruction.opcode {
                    0 => {}, //0x00 NOP
                    0x01 => self.ld_nn( instruction.operands, "BC"), //0x01 LD BC, d16
                    0x03 => self.inc_nn( "BC"), //0x03 INC BC
                    0x04 => self.inc( "B"), //0x04 INC B
                    0x05 => self.dec( "B"), //0x05 DEC B
                    0x06 => self.ld_r_d8( "B", instruction), //0x06 LD B,d8
                    0x07 => self.rlca(), //0x07 RLCA
                    0x09 => self.add_hl_n( "BC"), //0x09 ADD HL, BC
                    0x0B => self.dec_nn( "BC"), //0x0B DEC BC
                    0x0C => self.inc( "C"), //0x0C INC C
                    0x0D => self.dec( "C"), //0x0D DEC C
                    0x0E => self.ld_r_d8( "C", instruction), //0x0E LD C,d8
                    0x0F => self.rrca(), //0x0F RRCA
                    0x11 => self.ld_nn( instruction.operands, "DE"), //0x11 LD DE, d16
                    0x13 => self.inc_nn( "DE"), //0x13 INC DE
                    0x14 => self.inc( "D"), //0x14 INC D
                    0x15 => self.dec( "D"), //0x15 DEC D
                    0x16 => self.ld_r_d8( "D", instruction), //0x16 LD D,d8
                    0x17 => self.rla(), //0x17 RLA
                    0x18 => {}, //0x18 JR e8
                    0x19 => self.add_hl_n( "DE"), //0x19 ADD HL, DE
                    0x1B => self.dec_nn( "DE"), //0x1B DEC DE
                    0x1C => self.inc( "E"), //0x1C INC E
                    0x1D => self.dec( "E"), //0x1D DEC E
                    0x1E => self.ld_r_d8( "E", instruction), //0x1E LD E,d8
                    0x1F => self.rra(), //0x1F RRA
                    0x21 => self.ld_nn( instruction.operands, "HL"), //0x21 LD HL, d16
                    0x23 => self.inc_nn( "HL"), //0x23 INC HL
                    0x24 => self.inc( "H"), //0x24 INC H
                    0x25 => self.dec( "H"), //0x25 DEC H
                    0x26 => self.ld_r_d8( "H", instruction), //0x26 LD H,d8
                    0x27 => self.daa(), //0x27 DAA
                    0x29 => self.add_hl_n( "HL"), //0x29 ADD HL, HL
                    0x2B => self.dec_nn( "HL"), //0x2B DEC HL
                    0x2C => self.inc( "L"), //0x2C INC L
                    0x2D => self.dec( "L"), //0x2D DEC L
                    0x2E => self.ld_r_d8( "L", instruction), //0x2E LD L,d8
                    0x2F => self.cpl(), //0x2F CPL
                    0x31 => self.ld_nn(instruction.operands, "SP"), //0x31 LD SP, d16
                    0x33 => self.inc_nn( "SP"), //0x33 INC SP
                    0x37 => self.scf(), //0x37 SCF
                    0x39 => self.add_hl_n( "SP"), //0x39 ADD HL, SP
                    0x3B => self.dec_nn( "SP"), //0x3B DEC SP
                    0x3C => self.inc( "A"), //0x3C INC A
                    0x3D => self.dec("A"), //0x3D DEC A
                    0x3E => self.ld_r_d8( "A", instruction), //0x3E LD a,d8
                    0x3F => self.ccf(), //0x3F CCF
                    0x40 => self.ld_r_r("B", "B"), //0x40 LD B,B
                    0x41 => self.ld_r_r("C", "B"), //0x41 LD B,C
                    0x42 => self.ld_r_r("D", "B"), //0x42 LD B,D
                    0x43 => self.ld_r_r("E", "B"), //0x43 LD B,E
                    0x44 => self.ld_r_r("H", "B"), //0x44 LD B,H
                    0x45 => self.ld_r_r("L", "B"), //0x45 LD B,L
                    0x46 => {}, //TODO 0x46 LD B,(HL)
                    0x47 => self.ld_r_r("A", "B"), //0x47 LD B,A
                    0x48 => self.ld_r_r("B", "C"), //0x48 LD C,B
                    0x49 => self.ld_r_r("C", "C"), //0x49 LD C,C
                    0x4A => self.ld_r_r("D", "C"), //0x4A LD C,D
                    0x4B => self.ld_r_r("E", "C"), //0x4B LD C,E
                    0x4C => self.ld_r_r("H", "C"), //0x4C LD C,H
                    0x4D => self.ld_r_r("L", "C"), //0x4D LD C,L
                    0x4E => {}, //TODO 0x4E LD C,(HL)
                    0x4F => self.ld_r_r("A", "C"), //0x4F LD C,A
                    0x50 => self.ld_r_r("B", "D"), //0x50 LD D,B
                    0x51 => self.ld_r_r("C", "D"), //0x51 LD D,C
                    0x52 => self.ld_r_r("D", "D"), //0x52 LD D,D
                    0x53 => self.ld_r_r("E", "D"), //0x53 LD D,E
                    0x54 => self.ld_r_r("H", "D"), //0x54 LD D,H
                    0x55 => self.ld_r_r("L", "D"), //0x55 LD D,L
                    0x56 => {}, //TODO 0x56 LD D,(HL)
                    0x57 =>self.ld_r_r("A", "D"), //0x57 LD D,A
                    0x58 => self.ld_r_r("B", "E"), //0x58 LD E,B
                    0x59 => self.ld_r_r("C", "E"), //0x59 LD E,C
                    0x5A => self.ld_r_r("D", "E"), //0x5A LD E,D
                    0x5B => self.ld_r_r("E", "E"), //0x5B LD E,E
                    0x5C => self.ld_r_r("H", "E"), //0x5C LD E,H
                    0x5D => self.ld_r_r("L", "E"), //0x5D LD E,L
                    0x5E => {}, //TODO 0x5E LD E,(HL)
                    0x5F => self.ld_r_r("A", "E"), //0x5F LD E,A
                    0x60 => self.ld_r_r("B", "H"), //0x60 LD H,B
                    0x61 => self.ld_r_r("C", "H"), //0x61 LD H,C
                    0x62 => self.ld_r_r("D", "H"), //0x62 LD H,D
                    0x63 => self.ld_r_r("E", "H"), //0x63 LD H,E
                    0x64 => self.ld_r_r("H", "H"), //0x64 LD H,H
                    0x65 => self.ld_r_r("L", "H"), //0x65 LD H,L
                    0x66 => {}, //TODO 0x66 LD H,(HL)
                    0x67 => self.ld_r_r("A", "H"), //0x67 LD H,A
                    0x68 => self.ld_r_r("B", "L"), //0x68 LD L,B
                    0x69 => self.ld_r_r("C", "L"), //0x69 LD L,C
                    0x6A => self.ld_r_r("D", "L"), //0x6A LD L,D
                    0x6B => self.ld_r_r("E", "L"), //0x6B LD L,E
                    0x6C => self.ld_r_r("H", "L"), //0x6C LD L,H
                    0x6D => self.ld_r_r("L", "L"), //0x6D LD L,L
                    0x6E => {}, //TODO 0x6E LD L,(HL)
                    0x6F => self.ld_r_r("A", "L"), //0x6F LD L,A
                    0x78 => self.ld_r_r("B", "A"), //0x78 LD A,B
                    0x79 => self.ld_r_r("C", "A"), //0x79 LD A,C
                    0x7A => self.ld_r_r("D", "A"), //0x7A LD A,D
                    0x7B => self.ld_r_r("E", "A"), //0x7B LD A,E
                    0x7C => self.ld_r_r("H", "A"), //0x7C LD A,H
                    0x7D => self.ld_r_r("L", "A"), //0x7D LD A,L
                    0x7E => {}, //TODO 0x7E LD A,(HL)
                    0x7F => self.ld_r_r("A", "A"), //0x7F LD A,A
                    0x80 => self.add_a_r("B"), //0x80 ADD A,B
                    0x81 => self.add_a_r("C"), //0x81 ADD A,C
                    0x82 => self.add_a_r("D"), //0x82 ADD A,D
                    0x83 => self.add_a_r("E"), //0x83 ADD A,E
                    0x84 => self.add_a_r("H"), //0x84 ADD A,H
                    0x85 => self.add_a_r("L"), //0x85 ADD A,L
                    0x87 => self.add_a_r("A"), //0x87 ADD A,A
                    0x88 => self.adc_a("B"), //0x88 ADC A,B
                    0x89 => self.adc_a("C"), //0x89 ADC A,C
                    0x8A => self.adc_a("D"), //0x8A ADC A,D
                    0x8B => self.adc_a("E"), //0x8B ADC A,E
                    0x8C => self.adc_a("H"), //0x8C ADC A,H
                    0x8D => self.adc_a("L"), //0x8D ADC A,L
                    0x8E => {}, //TODO 0x8E ADC A,(HL)
                    0x8F => self.adc_a("A"), //0x8F ADC A,A
                    0x90 => self.sub_a_r("B"), //0x90 SUB B
                    0x91 => self.sub_a_r("C"), //0x91 SUB C
                    0x92 => self.sub_a_r("D"), //0x92 SUB D
                    0x93 => self.sub_a_r("E"), //0x93 SUB B
                    0x94 => self.sub_a_r("H"), //0x94 SUB H
                    0x95 => self.sub_a_r("L"), //0x90 SUB L
                    0x96 => {}, //TODO 0x96 SUB (HL)
                    0x97 => self.sub_a_r("A"), //0x97 SUB A
                    0x98 => self.sbc_a("B"), //0x98 SBC A,B
                    0x99 => self.sbc_a("C"), //0x99 SBC A,C
                    0x9A => self.sbc_a("D"), //0x9A SBC A,D
                    0x9B => self.sbc_a("E"), //0x9B SBC A,E
                    0x9C => self.sbc_a("H"), //0x9C SBC A,H
                    0x9D => self.sbc_a("L"), //0x9D SBC A,L
                    0x9E => {}, //TODO 0x9E SBC A,(HL)
                    0x9F => self.sbc_a("A"), //0x9F SBC A,A
                    0xA0 => self.and_a_r("B"), //0xA0 AND B
                    0xA1 => self.and_a_r("C"), //0xA1 AND C
                    0xA2 => self.and_a_r("D"), //0xA2 AND D
                    0xA3 => self.and_a_r("E"), //0xA3 AND E
                    0xA4 => self.and_a_r("H"), //0xA4 AND H
                    0xA5 => self.and_a_r("L"), //0xA5 AND L
                    0xA6 => {}, //TODO 0xA6 AND (HL)
                    0xA7 => self.and_a_r("A"), //0xA7 AND A
                    0xA8 => self.xor_a("B"), //0xA8 XOR B
                    0xA9 => self.xor_a("C"), //0xA9 XOR C
                    0xAA => self.xor_a("D"), //0xAA XOR D
                    0xAB => self.xor_a("E"), //0xAB XOR E
                    0xAC => self.xor_a("H"), //0xAC XOR H
                    0xAD => self.xor_a("L"), //0xAD XOR L
                    0xAE => {}, //TODO 0xAE XOR (HL)
                    0xAF => self.xor_a("A"), //0xAF XOR A
                    0xB0 => self.or_a_r("B"), //0xB0 OR B
                    0xB1 => self.or_a_r("C"), //0xB1 OR C
                    0xB2 => self.or_a_r("D"), //0xB2 OR D
                    0xB3 => self.or_a_r("E"), //0xB3 OR E
                    0xB4 => self.or_a_r("H"), //0xB4 OR H
                    0xB5 => self.or_a_r("L"), //0xB5 OR L
                    0xB6 => {}, //TODO 0xB6 OR (HL)
                    0xB7 => self.or_a_r("A"), //0xB7 OR A
                    0xB8 => self.cp_a("B"), //0xB8 CP B
                    0xB9 => self.cp_a("C"), //0xB9 CP C
                    0xBA => self.cp_a("D"), //0xBA CP D
                    0xBB => self.cp_a("E"), //0xBB CP E
                    0xBC => self.cp_a("H"), //0xBC CP H
                    0xBD => self.cp_a("L"), //0xBD CP L
                    0xBE => {}, //TODO 0xBE CP (HL)
                    0xBF => self.cp_a("A"), //0xBF CP A
                    0xC6 => self.add_a_n(instruction.operands),
                    0xD6 => self.sub_a_n(instruction.operands),
                    0xE6 => self.and_a_n(instruction.operands),
                    0xF6 => self.or_a_n(instruction.operands),
                    _ => return Err(instruction)
                }
            }
            Ok(())
        }

        fn ld_nn(&mut self, operands: Vec<Operand>, name: &str){
            let d16 = operands.into_iter().find(|operand| operand.name == "d16").expect("Operand d16 not found");
            self.Registers.set_item(name, d16.value.expect("Operand d16 has no value"))
        }

        fn inc_nn(&mut self, name: &str){
            let mut current_value = self.Registers.get_item(name);
            current_value += 1;
            self.Registers.set_item(name, current_value);
        }

        fn dec_nn(&mut self, name: &str){
            let mut current_value = self.Registers.get_item(name);
            current_value -= 1;
            self.Registers.set_item(name, current_value);
        }

        fn ld_r_r(&mut self, from: &str, to: &str){
            let from_value = self.Registers.get_item(from);
            self.Registers.set_item(to, from_value);
        }

        pub(crate) fn add_a_n(&mut self, operands: Vec<Operand>){
            let d8 = operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            self.add_a_value(d8.value.expect("operand has no value") as u16);
        }

        pub(crate) fn add_a_r(&mut self, to_add: &str){
            let value_to_add = self.Registers.get_item(to_add) as u16;
            self.add_a_value(value_to_add);
        }

        fn add_a_value(&mut self, value_to_add: u16){
            let current_value = self.Registers.get_item("A") as u16;
            let result = current_value + value_to_add;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", (result > 0xFF) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value as i16, value_to_add as i16, 0, HalfCarryOperationsMode::Add) as u16);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn adc_a(&mut self, to_add: &str){
            let to_add = self.Registers.get_item(to_add) as i16;
            let current_value = self.Registers.get_item("A") as i16;
            let carry = self.Registers.get_item("c") as i16;
            let result = to_add + current_value + carry;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", (result > 0xFF) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, to_add, carry, HalfCarryOperationsMode::Add) as u16);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn sub_a_r(&mut self, to_sub: &str) {
            let to_sub = self.Registers.get_item(to_sub) as i16;
            self.sub_a_value(to_sub)
        }

        pub(crate) fn sub_a_n(&mut self, operands: Vec<Operand>){
            let d8 = operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            self.sub_a_value(d8.value.expect("operand has no value") as i16);
        }

        fn sub_a_value(&mut self, to_sub: i16){
            let current_value = self.Registers.get_item("A") as i16;
            let result = current_value - to_sub;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", (to_sub > current_value) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, to_sub, 0, HalfCarryOperationsMode::GreaterThan) as u16);
            self.Registers.set_item("n", 1);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn sbc_a(&mut self, to_sub: &str) {
            let to_sub = self.Registers.get_item(to_sub) as i16;
            let current_value = self.Registers.get_item("A") as i16;
            let carry = self.Registers.get_item("c") as i16;
            let result = current_value - to_sub - carry;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", ((to_sub + carry) > current_value) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, to_sub, 1, HalfCarryOperationsMode::GreaterThan) as u16);
            self.Registers.set_item("n", 1);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn and_a_r(&mut self, to_and: &str){
            let to_and = self.Registers.get_item(to_and) as i16;
            self.and_a_value(to_and);
        }

        pub(crate) fn and_a_n(&mut self, operands: Vec<Operand>){
            let d8 = operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            self.and_a_value(d8.value.expect("operand has no value") as i16);
        }

        fn and_a_value(&mut self, to_and: i16){
            let current_value = self.Registers.get_item("A") as i16;
            let result = current_value & to_and;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", 0);
            self.Registers.set_item("h", 1);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn or_a_r(&mut self, to_or: &str){
            let to_or = self.Registers.get_item(to_or) as i16;
            self.or_a_value(to_or);
        }

        pub(crate) fn or_a_n(&mut self, operands: Vec<Operand>){
            let d8 = operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            self.or_a_value(d8.value.expect("operand has no value") as i16);
        }

        fn or_a_value(&mut self, to_or: i16){
            let current_value = self.Registers.get_item("A") as i16;
            let result = current_value | to_or;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", 0);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn xor_a(&mut self, to_xor: &str) {
            let to_xor = self.Registers.get_item(to_xor) as i16;
            let current_value = self.Registers.get_item("A") as i16;
            let result = current_value ^ to_xor;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", 0);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn cp_a(&mut self, to_cp: &str) {
            let old_a = self.Registers.get_item("A");
            self.sub_a_r(to_cp);
            self.Registers.set_item("A", old_a);
        }

        pub(crate) fn inc(&mut self, to_inc: &str) {
            let current_value = self.Registers.get_item(to_inc) as i16;
            let result = current_value + 1;

            self.Registers.set_item("n", 0);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, 0, 0, HalfCarryOperationsMode::Increment) as u16);
            self.Registers.set_item(to_inc, result as u16);
            self.Registers.set_item("z", (self.Registers.get_item(to_inc) == 0) as u16);
        }

        pub(crate) fn dec(&mut self, to_dec: &str) {
            let current_value = self.Registers.get_item(to_dec) as i16;
            let result = current_value - 1;

            self.Registers.set_item("n", 1);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, 0, 0, HalfCarryOperationsMode::Decrement) as u16);
            self.Registers.set_item(to_dec, result as u16);
            self.Registers.set_item("z", (self.Registers.get_item(to_dec) == 0) as u16);
        }

        pub(crate) fn ld_r_d8(&mut self, destination: &str, instruction: Instruction){
            //TODO move operand finding somewhere else, this function should just receive a number
            let d8 = instruction.operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            self.Registers.set_item(destination, d8.value.expect("Operand d8 has no value"))
        }

        pub(crate) fn rra(&mut self){
            let current_value = self.Registers.get_item("A") as u8;
            let carry = current_value & 1;  //0th bit
            let current_carry = self.Registers.get_item("c") as u8;
            let result = current_value >> 1 | current_carry << 7;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", carry as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", 0);
        }

        //rrca is rotate right circular, no data is loss
        pub(crate) fn rrca(&mut self){
            let current_value = self.Registers.get_item("A") as u8;
            let carry = current_value & 1;
            let result = current_value.rotate_right(1);

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", carry as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", 0);
        }

        pub(crate) fn rla(&mut self){
            let current_value = self.Registers.get_item("A") as u8;
            let carry = (current_value & 0x80) >> 7;
            let current_carry = self.Registers.get_item("c") as u8;
            let result = current_value << 1 | current_carry;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", carry as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", 0);
        }

        pub(crate) fn rlca(&mut self){
            let current_value = self.Registers.get_item("A") as u8;
            let carry = (current_value & 0x80) >> 7;
            let result = current_value.rotate_left(1);

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", carry as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", 0);
        }

        pub(crate) fn daa(&mut self){
            let mut current_value = self.Registers.get_item("A") as u8;
            let negative = self.Registers.get_item("n") as u8;
            let carry = self.Registers.get_item("c") as u8;
            let half_carry = self.Registers.get_item("h") as u8;

            if negative != 0 {
                if half_carry != 0 {
                    current_value = (current_value - 0x6) & 0xFF;
                }
                if carry != 0 {
                    current_value = current_value - 0x60;
                }
            }else{
                if ((current_value & 0xF) > 9) || half_carry != 0 {
                    current_value = current_value + 6;
                }
                if carry != 0 || (current_value > 0x9F) {
                    current_value = current_value + 0x60;
                }
            }

            self.Registers.set_item("A", current_value as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
            self.Registers.set_item("c", (current_value > 0xFF) as u16);
        }

        //supplement carry (0 -> 1, 1 -> 0)
        pub(crate) fn ccf(&mut self){
            let carry = self.Registers.get_item("c") as u8;
            self.Registers.set_item("c", (carry == 0) as u16);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("h", 0);
        }

        //complement accumulator
        pub(crate) fn cpl(&mut self){
            let accumulator = self.Registers.get_item("A") as u8;
            self.Registers.set_item("A", !accumulator as u16);
            self.Registers.set_item("n", 1);
            self.Registers.set_item("h", 1);
        }

        pub(crate) fn scf(&mut self){
            self.Registers.set_item("c", 1);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("h", 0);
        }

        pub(crate) fn add_hl_n(&mut self, to_add: &str){
            let current_value = self.Registers.get_item("HL") as u32;
            let to_add = self.Registers.get_item(to_add) as u32;
            let result: u32 = current_value + to_add;
            self.Registers.set_item("HL", result as u16);
            self.Registers.set_item("c", (result & 0x10000 != 0) as u16); //if true, result is bigger than 16 bit max value
            self.Registers.set_item("n", 0);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value as i16, to_add as i16, 0, HalfCarryOperationsMode::AddWords) as u16);
        }

        //half carry is carry calculated on the first half of a byte (from 3rd bit)
        fn calculate_half_carry(value: i16, second_operator: i16, carry: i16, mode: HalfCarryOperationsMode) -> bool{
            let rounded_value = value & 0xF;
            let rounded_second_operator = second_operator & 0xF;
            let rounded_word = value & 0xFFF;
            let rounded_second_word = value & 0xFFF;
            match mode {
                HalfCarryOperationsMode::Add => {
                    (rounded_second_operator + rounded_value + carry) > 0xF
                }
                HalfCarryOperationsMode::AddWords => {
                    (rounded_word +  rounded_second_word) > 0xFFF
                }
                HalfCarryOperationsMode::GreaterThan => {
                    (rounded_second_operator + carry) > rounded_value
                }
                HalfCarryOperationsMode::Increment => {
                    (rounded_value + 1) > 0xF
                }
                HalfCarryOperationsMode::Decrement => {
                    //we carry only if value is 0
                    rounded_value == 0
                }
            }
        }
    }
}