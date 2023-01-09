mod registers;

pub mod CPU{
    use std::fmt::{Display, Formatter};
    use crate::cpu::registers::Registers::Registers;
    use crate::decoder::decoder::Decoder;
    use crate::interrupt::interrupt::Interrupt;
    use crate::mmu::mmu::MMU;
    use crate::op_codes_parser::op_codes_parser::{Instruction, Operand};

    pub struct CPU {
        pub(crate) Registers: Registers,
        pub(crate) Decoder: Decoder,
        pub(crate) MMU: MMU,
        pub(crate) Interrupt: Interrupt,
        pub(crate) is_stopped: bool
    }

    pub enum JumpCondition {
        Zero,
        NotZero,
        Carry,
        NotCarry,
        None
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

        pub(crate) fn new(Decoder: Option<Decoder>, MMU: MMU) -> CPU {
            let Registers: Registers = Registers::new();
            let decoder = Decoder.unwrap_or(Decoder{
                data: vec![],
                address: 0,
                unprefixed_op_codes: Default::default(),
                prefixed_op_codes: Default::default()
            });
            CPU {
                Registers,
                Decoder: decoder,
                MMU,
                Interrupt: Default::default(),
                is_stopped: false
            }
        }

        pub(crate) fn run(&mut self) {
            loop {
                if self.is_stopped {
                    continue
                }

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
                    0x02 => self.ld_address_value("BC", "A"), //0x02 LD (BC), A
                    0x03 => self.inc_nn( "BC"), //0x03 INC BC
                    0x04 => self.inc( "B"), //0x04 INC B
                    0x05 => self.dec( "B"), //0x05 DEC B
                    0x06 => self.ld_r_d8( "B", instruction), //0x06 LD B,d8
                    0x07 => self.rlca(), //0x07 RLCA
                    0x09 => self.add_hl_n( "BC"), //0x09 ADD HL, BC
                    0x0A => self.ld_a_address("BC"), //0x0A LD A,(BC)
                    0x0B => self.dec_nn( "BC"), //0x0B DEC BC
                    0x0C => self.inc( "C"), //0x0C INC C
                    0x0D => self.dec( "C"), //0x0D DEC C
                    0x0E => self.ld_r_d8( "C", instruction), //0x0E LD C,d8
                    0x0F => self.rrca(), //0x0F RRCA
                    0x10 => self.is_stopped = true, //0x10 STOP
                    0x11 => self.ld_nn( instruction.operands, "DE"), //0x11 LD DE, d16
                    0x12 => self.ld_address_value("DE", "A"), //0x12 LD (DE),A
                    0x13 => self.inc_nn( "DE"), //0x13 INC DE
                    0x14 => self.inc( "D"), //0x14 INC D
                    0x15 => self.dec( "D"), //0x15 DEC D
                    0x16 => self.ld_r_d8( "D", instruction), //0x16 LD D,d8
                    0x17 => self.rla(), //0x17 RLA
                    0x18 => self.jr_r8(instruction, JumpCondition::None), //0x18 JR r8
                    0x19 => self.add_hl_n( "DE"), //0x19 ADD HL, DE
                    0x1A => self.ld_a_address("DE"), //0x1A LD A,(DE)
                    0x1B => self.dec_nn( "DE"), //0x1B DEC DE
                    0x1C => self.inc( "E"), //0x1C INC E
                    0x1D => self.dec( "E"), //0x1D DEC E
                    0x1E => self.ld_r_d8( "E", instruction), //0x1E LD E,d8
                    0x1F => self.rra(), //0x1F RRA
                    0x20 => self.jr_r8(instruction, JumpCondition::NotZero), //0x20 JR NZ,r8
                    0x21 => self.ld_nn( instruction.operands, "HL"), //0x21 LD HL, d16
                    0x22 => self.ld_hl_pointer_dec_inc_a(true), //0x22 LD (HL+), A
                    0x23 => self.inc_nn( "HL"), //0x23 INC HL
                    0x24 => self.inc( "H"), //0x24 INC H
                    0x25 => self.dec( "H"), //0x25 DEC H
                    0x26 => self.ld_r_d8( "H", instruction), //0x26 LD H,d8
                    0x27 => self.daa(), //0x27 DAA
                    0x28 => self.jr_r8(instruction, JumpCondition::Zero), //0x28 JR Z,r8
                    0x29 => self.add_hl_n( "HL"), //0x29 ADD HL, HL
                    0x2A => { self.ld_a_address("HL"); self.inc_nn("HL") }, //0x2A LD A,(HL+)
                    0x2B => self.dec_nn( "HL"), //0x2B DEC HL
                    0x2C => self.inc( "L"), //0x2C INC L
                    0x2D => self.dec( "L"), //0x2D DEC L
                    0x2E => self.ld_r_d8( "L", instruction), //0x2E LD L,d8
                    0x2F => self.cpl(), //0x2F CPL
                    0x30 => self.jr_r8(instruction, JumpCondition::NotCarry), //0x30 JR NC,r8
                    0x31 => self.ld_nn(instruction.operands, "SP"), //0x31 LD SP, d16
                    0x32 => self.ld_hl_pointer_dec_inc_a(false), //0x32 LD (HL-), A
                    0x33 => self.inc_nn( "SP"), //0x33 INC SP
                    0x34 => self.inc_hl_pointer(), //0x34 INC (HL)
                    0x35 => self.dec_hl_pointer(), //0x35 DEC (HL)
                    0x36 => self.ld_hl_pointer_d8(instruction), //0x36 LD (HL),d8
                    0x37 => self.scf(), //0x37 SCF
                    0x38 => self.jr_r8(instruction, JumpCondition::Carry), //0x38 JR C,r8
                    0x39 => self.add_hl_n( "SP"), //0x39 ADD HL, SP
                    0x3A => { self.ld_a_address("HL"); self.dec_nn("HL") }, //0x3A LD A,(HL-)
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
                    0x46 => self.ld_r_hl("B"), //0x46 LD B,(HL)
                    0x47 => self.ld_r_r("A", "B"), //0x47 LD B,A
                    0x48 => self.ld_r_r("B", "C"), //0x48 LD C,B
                    0x49 => self.ld_r_r("C", "C"), //0x49 LD C,C
                    0x4A => self.ld_r_r("D", "C"), //0x4A LD C,D
                    0x4B => self.ld_r_r("E", "C"), //0x4B LD C,E
                    0x4C => self.ld_r_r("H", "C"), //0x4C LD C,H
                    0x4D => self.ld_r_r("L", "C"), //0x4D LD C,L
                    0x4E => self.ld_r_hl("C"), //0x4E LD C,(HL)
                    0x4F => self.ld_r_r("A", "C"), //0x4F LD C,A
                    0x50 => self.ld_r_r("B", "D"), //0x50 LD D,B
                    0x51 => self.ld_r_r("C", "D"), //0x51 LD D,C
                    0x52 => self.ld_r_r("D", "D"), //0x52 LD D,D
                    0x53 => self.ld_r_r("E", "D"), //0x53 LD D,E
                    0x54 => self.ld_r_r("H", "D"), //0x54 LD D,H
                    0x55 => self.ld_r_r("L", "D"), //0x55 LD D,L
                    0x56 => self.ld_r_hl("D"), //0x56 LD D,(HL)
                    0x57 =>self.ld_r_r("A", "D"), //0x57 LD D,A
                    0x58 => self.ld_r_r("B", "E"), //0x58 LD E,B
                    0x59 => self.ld_r_r("C", "E"), //0x59 LD E,C
                    0x5A => self.ld_r_r("D", "E"), //0x5A LD E,D
                    0x5B => self.ld_r_r("E", "E"), //0x5B LD E,E
                    0x5C => self.ld_r_r("H", "E"), //0x5C LD E,H
                    0x5D => self.ld_r_r("L", "E"), //0x5D LD E,L
                    0x5E => self.ld_r_hl("E"), //0x5E LD E,(HL)
                    0x5F => self.ld_r_r("A", "E"), //0x5F LD E,A
                    0x60 => self.ld_r_r("B", "H"), //0x60 LD H,B
                    0x61 => self.ld_r_r("C", "H"), //0x61 LD H,C
                    0x62 => self.ld_r_r("D", "H"), //0x62 LD H,D
                    0x63 => self.ld_r_r("E", "H"), //0x63 LD H,E
                    0x64 => self.ld_r_r("H", "H"), //0x64 LD H,H
                    0x65 => self.ld_r_r("L", "H"), //0x65 LD H,L
                    0x66 => self.ld_r_hl("H"), //0x66 LD H,(HL)
                    0x67 => self.ld_r_r("A", "H"), //0x67 LD H,A
                    0x68 => self.ld_r_r("B", "L"), //0x68 LD L,B
                    0x69 => self.ld_r_r("C", "L"), //0x69 LD L,C
                    0x6A => self.ld_r_r("D", "L"), //0x6A LD L,D
                    0x6B => self.ld_r_r("E", "L"), //0x6B LD L,E
                    0x6C => self.ld_r_r("H", "L"), //0x6C LD L,H
                    0x6D => self.ld_r_r("L", "L"), //0x6D LD L,L
                    0x6E => self.ld_r_hl("L"), //0x6E LD L,(HL)
                    0x6F => self.ld_r_r("A", "L"), //0x6F LD L,A
                    0x70 => self.ld_address_value("HL", "B"), //0x70 LD (HL),B
                    0x71 => self.ld_address_value("HL", "C"), //0x71 LD (HL),C
                    0x72 => self.ld_address_value("HL", "D"), //0x72 LD (HL),D
                    0x73 => self.ld_address_value("HL", "E"), //0x73 LD (HL),E
                    0x74 => self.ld_address_value("HL", "H"), //0x74 LD (HL),H
                    0x75 => self.ld_address_value("HL", "L"), //0x75 LD (HL),L
                    0x76 => self.is_stopped = true, //0x76 HALT
                    0x77 => self.ld_address_value("HL", "A"), //0x77 LD (HL),A
                    0x78 => self.ld_r_r("B", "A"), //0x78 LD A,B
                    0x79 => self.ld_r_r("C", "A"), //0x79 LD A,C
                    0x7A => self.ld_r_r("D", "A"), //0x7A LD A,D
                    0x7B => self.ld_r_r("E", "A"), //0x7B LD A,E
                    0x7C => self.ld_r_r("H", "A"), //0x7C LD A,H
                    0x7D => self.ld_r_r("L", "A"), //0x7D LD A,L
                    0x7E => self.ld_r_hl("A"), //0x7E LD A,(HL)
                    0x7F => self.ld_r_r("A", "A"), //0x7F LD A,A
                    0x80 => self.add_a_r("B"), //0x80 ADD A,B
                    0x81 => self.add_a_r("C"), //0x81 ADD A,C
                    0x82 => self.add_a_r("D"), //0x82 ADD A,D
                    0x83 => self.add_a_r("E"), //0x83 ADD A,E
                    0x84 => self.add_a_r("H"), //0x84 ADD A,H
                    0x85 => self.add_a_r("L"), //0x85 ADD A,L
                    0x86 => self.add_a_hl(), //0x86 ADD A,(HL)
                    0x87 => self.add_a_r("A"), //0x87 ADD A,A
                    0x88 => self.adc_a_r("B"), //0x88 ADC A,B
                    0x89 => self.adc_a_r("C"), //0x89 ADC A,C
                    0x8A => self.adc_a_r("D"), //0x8A ADC A,D
                    0x8B => self.adc_a_r("E"), //0x8B ADC A,E
                    0x8C => self.adc_a_r("H"), //0x8C ADC A,H
                    0x8D => self.adc_a_r("L"), //0x8D ADC A,L
                    0x8F => self.adc_a_r("A"), //0x8F ADC A,A
                    0x90 => self.sub_a_r("B"), //0x90 SUB B
                    0x91 => self.sub_a_r("C"), //0x91 SUB C
                    0x92 => self.sub_a_r("D"), //0x92 SUB D
                    0x93 => self.sub_a_r("E"), //0x93 SUB B
                    0x94 => self.sub_a_r("H"), //0x94 SUB H
                    0x95 => self.sub_a_r("L"), //0x95 SUB L
                    0x96 => self.sub_a_hl(), //0x96 SUB (HL)
                    0x97 => self.sub_a_r("A"), //0x97 SUB A
                    0x98 => self.sbc_a_r("B"), //0x98 SBC A,B
                    0x99 => self.sbc_a_r("C"), //0x99 SBC A,C
                    0x9A => self.sbc_a_r("D"), //0x9A SBC A,D
                    0x9B => self.sbc_a_r("E"), //0x9B SBC A,E
                    0x9C => self.sbc_a_r("H"), //0x9C SBC A,H
                    0x9D => self.sbc_a_r("L"), //0x9D SBC A,L
                    0x9E => self.sbc_a_hl(), //0x9E SBC A,(HL)
                    0x9F => self.sbc_a_r("A"), //0x9F SBC A,A
                    0xA0 => self.and_a_r("B"), //0xA0 AND B
                    0xA1 => self.and_a_r("C"), //0xA1 AND C
                    0xA2 => self.and_a_r("D"), //0xA2 AND D
                    0xA3 => self.and_a_r("E"), //0xA3 AND E
                    0xA4 => self.and_a_r("H"), //0xA4 AND H
                    0xA5 => self.and_a_r("L"), //0xA5 AND L
                    0xA6 => self.and_a_hl(), //0xA6 AND (HL)
                    0xA7 => self.and_a_r("A"), //0xA7 AND A
                    0xA8 => self.xor_a_r("B"), //0xA8 XOR B
                    0xA9 => self.xor_a_r("C"), //0xA9 XOR C
                    0xAA => self.xor_a_r("D"), //0xAA XOR D
                    0xAB => self.xor_a_r("E"), //0xAB XOR E
                    0xAC => self.xor_a_r("H"), //0xAC XOR H
                    0xAD => self.xor_a_r("L"), //0xAD XOR L
                    0xAE => self.xor_a_hl(), //0xAE XOR (HL)
                    0xAF => self.xor_a_r("A"), //0xAF XOR A
                    0xB0 => self.or_a_r("B"), //0xB0 OR B
                    0xB1 => self.or_a_r("C"), //0xB1 OR C
                    0xB2 => self.or_a_r("D"), //0xB2 OR D
                    0xB3 => self.or_a_r("E"), //0xB3 OR E
                    0xB4 => self.or_a_r("H"), //0xB4 OR H
                    0xB5 => self.or_a_r("L"), //0xB5 OR L
                    0xB6 => self.or_a_hl(), //0xB6 OR (HL)
                    0xB7 => self.or_a_r("A"), //0xB7 OR A
                    0xB8 => self.cp_a("B"), //0xB8 CP B
                    0xB9 => self.cp_a("C"), //0xB9 CP C
                    0xBA => self.cp_a("D"), //0xBA CP D
                    0xBB => self.cp_a("E"), //0xBB CP E
                    0xBC => self.cp_a("H"), //0xBC CP H
                    0xBD => self.cp_a("L"), //0xBD CP L
                    0xBE => self.cp_a_hl(), //0xBE CP (HL)
                    0xBF => self.cp_a("A"), //0xBF CP A
                    0xC1 => self.pop_rr("BC"), //0xC1 POP BC
                    0xC5 => self.push_rr("BC"), //0xC5 PUSH BC
                    0xC6 => self.add_a_n(instruction.operands), //0xC6 ADD A,d8
                    0xC7 => self.rst(0x0), //0xC7 RST 00H
                    0xCB => {}, //0xCB CB PREFIX
                    0xCF => self.rst(0x8), //0xCF RST 08H
                    0xD1 => self.pop_rr("DE"), //0xD1 POP DE
                    0xD3 => (), //0xD3 UNDEFINED
                    0xD5 => self.push_rr("DE"), //0xD5 PUSH DE
                    0xD6 => self.sub_a_n(instruction.operands), //0xD6 SUB d8
                    0xD7 => self.rst(0x10), //0xD7 RST 10H
                    0xDB => (), //0xDB UNDEFINED
                    0xDD => (), //0xDD UNDEFINED
                    0xDF => self.rst(0x18), //0xDF RST 18H
                    0xE1 => self.pop_rr("HL"), //0xE1 POP HL
                    0xE2 => self.ld_c_pointer_a(), //0xE2 LD (C),A
                    0xE3 => (), //0xE3 UNDEFINED
                    0xE4 => (), //0xE4 UNDEFINED
                    0xE5 => self.push_rr("HL"), //0xE5 PUSH HL
                    0xE6 => self.and_a_n(instruction.operands), //0xE6 AND d8
                    0xE7 => self.rst(0x20), //0xE7 RST 20H
                    0xEB => (), //0xEB UNDEFINED
                    0xEC => (), //0xEC UNDEFINED
                    0xED => (), //0xED UNDEFINED
                    0xEF => self.rst(0x28), //0xEF RST 28H
                    0xF1 => self.pop_rr("AF"), //0xF1 POP AF
                    0xF2 => self.ld_a_c_pointer(), //0xF2 LD A,(C)
                    0xF3 => self.Interrupt.enabled = false, //0xF3 DI
                    0xF4 => (), //0xF4 UNDEFINED
                    0xF5 => self.push_rr("AF"), //0xF5 PUSH AF
                    0xF6 => self.or_a_n(instruction.operands),  //0xF6 OR d8
                    0xF7 => self.rst(0x30), //0xF7 RST 30H
                    0xFB => self.Interrupt.enabled = true, //0xFB EI
                    0xFC => (), //0xFC UNDEFINED
                    0xFD => (), //0xFD UNDEFINED
                    0xFF => self.rst(0x38), //0xFF RST 38H
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

        fn ld_r_hl(&mut self, to: &str){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.Registers.set_item(to, value_at_hl as u16);
        }

        pub(crate) fn add_a_n(&mut self, operands: Vec<Operand>){
            let d8 = operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            self.add_a(d8.value.expect("operand has no value") as u16);
        }

        pub(crate) fn add_a_r(&mut self, to_add: &str){
            let value_to_add = self.Registers.get_item(to_add) as u16;
            self.add_a(value_to_add);
        }

        pub(crate) fn ld_a16_pointer_sp(&mut self, Instruction: Instruction){
            let a16 = Instruction.operands.into_iter().find(|operand| operand.name == "a16").expect("Operand a16 not found").value.unwrap();
            let sp = self.Registers.get_item("SP");
            self.MMU.write_word(a16 as i32, sp);
        }

        pub(crate) fn ld_a_c_pointer(&mut self){
            let c = self.Registers.get_item("C");
            let a = self.Registers.get_item("A");
            let value_at_c = self.MMU.read_byte((0xFF00 + c) as i32);
            self.Registers.set_item("A", value_at_c as u16);
        }

        pub(crate) fn ld_c_pointer_a(&mut self){
            let c = self.Registers.get_item("C");
            let a = self.Registers.get_item("A");
            self.MMU.write_byte((0xFF00 + c) as i32, a as u8);
        }

        pub(crate) fn add_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.add_a(value_at_hl as u16);
        }

        fn add_a(&mut self, value: u16){
            let current_value = self.Registers.get_item("A") as u16;
            let result = current_value + value;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", (result > 0xFF) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value as i16, value as i16, 0, HalfCarryOperationsMode::Add) as u16);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn adc_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.adc_a(value_at_hl as i16);
        }

        pub(crate) fn adc_a_r(&mut self, to_add: &str){
            let to_add = self.Registers.get_item(to_add) as i16;
            self.adc_a(to_add);
        }

        pub(crate) fn adc_d8(&mut self, Instruction: Instruction){

        }

        pub(crate) fn adc_a(&mut self, value: i16) {
            let current_value = self.Registers.get_item("A") as i16;
            let carry = self.Registers.get_item("c") as i16;
            let result = value + current_value + carry;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", (result > 0xFF) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, value, carry, HalfCarryOperationsMode::Add) as u16);
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

        pub(crate) fn sub_a_hl(&mut self) {
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.sub_a_value(value_at_hl as i16);
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

        pub(crate) fn sbc_a_r(&mut self, to_sub: &str){
            let to_sub = self.Registers.get_item(to_sub) as i16;
            self.sbc_a(to_sub)
        }

        pub(crate) fn sbc_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.sbc_a(value_at_hl as i16);
        }

        pub(crate) fn sbc_a(&mut self, value: i16) {
            let current_value = self.Registers.get_item("A") as i16;
            let carry = self.Registers.get_item("c") as i16;
            let result = current_value - value - carry;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", ((value + carry) > current_value) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, value, 1, HalfCarryOperationsMode::GreaterThan) as u16);
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

        pub(crate) fn and_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.and_a_value(value_at_hl as i16);
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

        pub(crate) fn or_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.or_a_value(value_at_hl as i16);
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

        pub(crate) fn xor_a_r(&mut self, to_xor: &str) {
            let to_xor = self.Registers.get_item(to_xor) as i16;
            self.xor_a_value(to_xor);
        }

        pub(crate) fn xor_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.xor_a_value(value_at_hl as i16);
        }

        pub(crate) fn xor_a_value(&mut self, value: i16){
            let current_value = self.Registers.get_item("A") as i16;
            let result = current_value ^ value;

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

        pub(crate) fn cp_a_hl(&mut self){
            let old_a = self.Registers.get_item("A");
            self.sub_a_hl();
            self.Registers.set_item("A", old_a);
        }

        pub(crate) fn ld_hl_pointer_d8(&mut self, instruction: Instruction){
            let d8 = instruction.operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            let hl = self.Registers.get_item("HL") as i32;
            self.MMU.write_byte(hl, d8.value.expect("d8 has no value") as u8);
        }

        pub(crate) fn ld_hl_pointer_dec_inc_a(&mut self, increase: bool){
            let mut hl = self.Registers.get_item("HL");
            let a = self.Registers.get_item("A");
            self.MMU.write_byte(hl as i32, a as u8);
            if increase {
                hl += 1;
            }else {
                hl -= 1;
            }
            self.Registers.set_item("HL", hl);
        }

        pub(crate) fn inc_hl_pointer(&mut self) {
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);

            let result = value_at_hl + 1;

            self.Registers.set_item("n", 0);
            self.Registers.set_item("h", CPU::calculate_half_carry(value_at_hl as i16, 0, 0, HalfCarryOperationsMode::Increment) as u16);
            self.MMU.write_byte(hl as i32, result);
            self.Registers.set_item("z", (self.MMU.read_byte(hl as i32) == 0) as u16);
        }

        pub(crate) fn inc(&mut self, to_inc: &str) {
            let current_value = self.Registers.get_item(to_inc) as i16;
            let result = current_value + 1;

            self.Registers.set_item("n", 0);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, 0, 0, HalfCarryOperationsMode::Increment) as u16);
            self.Registers.set_item(to_inc, result as u16);
            self.Registers.set_item("z", (self.Registers.get_item(to_inc) == 0) as u16);
        }

        pub(crate) fn dec_hl_pointer(&mut self) {
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);

            let result = value_at_hl - 1;

            self.Registers.set_item("n", 1);
            self.Registers.set_item("h", CPU::calculate_half_carry(value_at_hl as i16, 0, 0, HalfCarryOperationsMode::Decrement) as u16);
            self.MMU.write_byte(hl as i32, result);
            self.Registers.set_item("z", (self.MMU.read_byte(hl as i32) == 0) as u16);
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

        pub(crate) fn ld_address_value(&mut self, address_pointer: &str, value: &str){
            let address = self.Registers.get_item(address_pointer);
            let value = self.Registers.get_item(value);
            self.MMU.write_byte(address as i32, value as u8)
        }

        pub(crate) fn ld_a_address(&mut self, address_pointer: &str){
            let address = self.Registers.get_item(address_pointer);
            let value = self.MMU.read_byte(address as i32);
            self.Registers.set_item("A", value as u16);
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

        pub(crate) fn rst(&mut self, new_pc: u16){
            self.write_to_stack(self.Registers.get_item("PC"));
            self.Registers.set_item("PC", new_pc);
        }

        pub(crate) fn jr_r8(&mut self, Instruction: Instruction, JumpCondition: JumpCondition){
            let r8 = Instruction.operands.into_iter().find(|operand| operand.name == "r8").expect("Operand r8 not found").value.unwrap();
            let current_pc = self.Registers.get_item("PC");
            let should_jump = match JumpCondition {
                JumpCondition::Zero => self.Registers.get_item("z") == 1,
                JumpCondition::NotZero => self.Registers.get_item("z") == 0,
                JumpCondition::Carry => self.Registers.get_item("c") == 1,
                JumpCondition::NotCarry => self.Registers.get_item("c") == 0,
                JumpCondition::None => true
            };
            if should_jump {
                self.Registers.set_item("PC", current_pc + r8);
            }
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
        
        pub(crate) fn push_rr(&mut self, to_push: &str){
            let to_push = self.Registers.get_item(to_push);
            self.write_to_stack(to_push);
        }
        
        pub(crate) fn pop_rr(&mut self, pop_into: &str){
            let value = self.read_from_stack();
            self.Registers.set_item(pop_into, value)
        }

        pub(crate) fn write_to_stack(&mut self, value: u16){
            let mut sp = self.Registers.get_item("SP");
            sp -= 2;
            self.MMU.write_byte(sp as i32, (value & 0x00FF) as u8);
            //since value is 16 bits we have to mask and shift to get the first 8 bits
            self.MMU.write_byte((sp + 1) as i32, ((value & 0xFF00) >> 8)  as u8);
            self.Registers.set_item("SP", sp);
        }

        pub(crate) fn read_from_stack(&mut self) -> u16 {
            let mut sp = self.Registers.get_item("SP");
            let first_8_bits = self.MMU.read_byte(sp as i32) as u16;
            let last_8_bits = self.MMU.read_byte((sp + 1) as i32) as u16;
            self.Registers.set_item("SP", sp + 2);
            (first_8_bits | last_8_bits << 8) as u16
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