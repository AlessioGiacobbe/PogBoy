mod interrupts;
mod op;
mod registers;
mod timer;

pub mod CPU {
    use crate::cpu::registers::Registers::Registers;
    use crate::memory::mmu::mmu::MMU;
    use crate::memory::op_codes_parser::op_codes_parser::Instruction;
    use std::fmt::{Display, Formatter};
    use strum_macros::EnumIter;

    pub struct CPU<'a> {
        pub(crate) Registers: Registers,
        pub(crate) MMU: MMU<'a>,
        pub(crate) is_halted: bool,
        pub(crate) clock: u32,
        pub(crate) logging: bool,
    }

    #[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
    pub enum InterruptType {
        VBlank = 1,
        LCD_STAT = 2,
        Timer = 4,
        Serial = 8,
        Joypad = 16,
    }

    #[derive(PartialEq)]
    pub enum JumpCondition {
        Zero,
        NotZero,
        Carry,
        NotCarry,
        None,
    }

    pub enum HalfCarryOperationsMode {
        Add,      //8bit
        AddWords, //16bit
        GreaterThan,
        Increment,
        Decrement,
    }

    impl Display for CPU<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "clock : {} - Registers : {}", self.clock, self.Registers)
        }
    }

    impl CPU<'_> {
        pub(crate) fn new(MMU: MMU) -> CPU {
            let Registers: Registers = Registers::new();
            CPU {
                Registers,
                MMU,
                is_halted: false,
                clock: 0,
                logging: false,
            }
        }

        pub(crate) fn step(&mut self) -> (u32, u32) {
            if self.is_halted {
                if self.logging {
                    println!("STUCK at 0x{:02X}", self.Registers.get_item("PC"));
                }
                return (self.clock, 4);
            }

            let address = self.Registers.get_item("PC");

            let (next_address, instruction) = self.MMU.decode(address as i32);
            self.Registers.set_item("PC", next_address as u16);

            let clock_increment = if instruction.cycles.len() > 2 {
                instruction.cycles[1]
            } else {
                instruction.cycles[0]
            };
            self.clock += clock_increment;

            if self.logging {
                println!(
                    "0x{:02X} Executing {} (op code 0x{:02X})",
                    address, instruction, instruction.opcode
                );
            }

            self.MMU.interrupt_queued = false;
            //println!("Executing {} (op code 0x{:02X})", instruction, instruction.opcode);
            match self.execute(instruction) {
                Err(instruction) => {
                    let starting_address = if address >= 12 { address - 12 } else { 0 };
                    let quantity_to_disassemble = if starting_address <= (0xFFFF - 25) {
                        25
                    } else {
                        0xFFFF - starting_address
                    };
                    self.MMU.disassemble(
                        starting_address as i32,
                        quantity_to_disassemble as i32,
                        address as i32,
                    );
                    println!("{}", self);
                    panic!(
                        "⚠️{:#04x} NOT IMPLEMENTED⚠️ {:?}",
                        instruction.opcode, instruction
                    )
                }
                _ => {
                    if self.logging {
                        println!("STATUS AFTER EXECUTING 0x{:04X} {}", address, self);
                    }
                    self.MMU.interrupt_queued = false;
                    return (self.clock, clock_increment);
                }
            };
        }

        pub(crate) fn execute(&mut self, instruction: Instruction) -> Result<(), Instruction> {
            if instruction.prefixed {
                match instruction.opcode {
                    0x00 => self.rlc_r("B", true),  //0x0 RLC B
                    0x01 => self.rlc_r("C", true),  //0x01 RLC C
                    0x02 => self.rlc_r("D", true),  //0x02 RLC D
                    0x03 => self.rlc_r("E", true),  //0x03 RLC E
                    0x04 => self.rlc_r("H", true),  //0x04 RLC H
                    0x05 => self.rlc_r("L", true),  //0x05 RLC L
                    0x06 => self.rlc_hl_pointer(),  //0x06 RLC (HL)
                    0x07 => self.rlc_r("A", true),  //0x07 RLC A
                    0x08 => self.rrc_r("B", true),  //0x08 RRC B
                    0x09 => self.rrc_r("C", true),  //0x09 RRC C
                    0x0A => self.rrc_r("D", true),  //0x0A RRC D
                    0x0B => self.rrc_r("E", true),  //0x0B RRC E
                    0x0C => self.rrc_r("H", true),  //0x0C RRC H
                    0x0D => self.rrc_r("L", true),  //0x0D RRC L
                    0x0E => self.rrc_hl_pointer(),  //0x0E RRC (HL)
                    0x0F => self.rrc_r("A", true),  //0x0F RRC A
                    0x10 => self.rl_r("B", true),   //0x10 RL B
                    0x11 => self.rl_r("C", true),   //0x11 RL C
                    0x12 => self.rl_r("D", true),   //0x12 RL D
                    0x13 => self.rl_r("E", true),   //0x13 RL E
                    0x14 => self.rl_r("H", true),   //0x14 RL H
                    0x15 => self.rl_r("L", true),   //0x15 RL L
                    0x16 => self.rl_hl_pointer(),   //0x16 RL (HL)
                    0x17 => self.rl_r("A", true),   //0x17 RL A
                    0x18 => self.rr_r("B", true),   //0x18 RR B
                    0x19 => self.rr_r("C", true),   //0x19 RR C
                    0x1A => self.rr_r("D", true),   //0x1A RR D
                    0x1B => self.rr_r("E", true),   //0x1B RR E
                    0x1C => self.rr_r("H", true),   //0x1C RR H
                    0x1D => self.rr_r("L", true),   //0x1D RR L
                    0x1E => self.rr_hl_pointer(),   //0x1E RR (HL)
                    0x1F => self.rr_r("A", true),   //0x1F RR A
                    0x20 => self.sla_r("B"),        //0x20 SLA B
                    0x21 => self.sla_r("C"),        //0x21 SLA C
                    0x22 => self.sla_r("D"),        //0x22 SLA D
                    0x23 => self.sla_r("E"),        //0x23 SLA E
                    0x24 => self.sla_r("H"),        //0x24 SLA H
                    0x25 => self.sla_r("L"),        //0x25 SLA L
                    0x26 => self.sla_hl_pointer(),  //0x26 SLA (HL)
                    0x27 => self.sla_r("A"),        //0x27 SLA A
                    0x28 => self.sra_r("B"),        //0x28 SRA B
                    0x29 => self.sra_r("C"),        //0x29 SRA C
                    0x2A => self.sra_r("D"),        //0x2A SRA D
                    0x2B => self.sra_r("E"),        //0x2B SRA E
                    0x2C => self.sra_r("H"),        //0x2C SRA H
                    0x2D => self.sra_r("L"),        //0x2D SRA L
                    0x2E => self.sra_hl_pointer(),  //0x2E SRA (HL)
                    0x2F => self.sra_r("A"),        //0x2F SRA A
                    0x30 => self.swap_r("B"),       //0x30 SWAP B
                    0x31 => self.swap_r("C"),       //0x31 SWAP C
                    0x32 => self.swap_r("D"),       //0x32 SWAP D
                    0x33 => self.swap_r("E"),       //0x33 SWAP E
                    0x34 => self.swap_r("H"),       //0x34 SWAP H
                    0x35 => self.swap_r("L"),       //0x35 SWAP L
                    0x36 => self.swap_hl_pointer(), //0x36 SWAP (HL)
                    0x37 => self.swap_r("A"),       //0x37 SWAP A
                    0x38 => self.srl_r("B"),        //0x38 SRL B
                    0x39 => self.srl_r("C"),        //0x39 SRL C
                    0x3A => self.srl_r("D"),        //0x3A SRL D
                    0x3B => self.srl_r("E"),        //0x3B SRL E
                    0x3C => self.srl_r("H"),        //0x3C SRL H
                    0x3D => self.srl_r("L"),        //0x3D SRL L
                    0x3E => self.srl_hl_pointer(),  //0x3E SRL (HL)
                    0x3F => self.srl_r("A"),        //0x3F SRL A
                    0x40 => self.bit_n_r(0, "B"),   //0x40 BIT 0,B
                    0x41 => self.bit_n_r(0, "C"),   //0x41  BIT 0,C
                    0x42 => self.bit_n_r(0, "D"),   //0x42  BIT 0,D
                    0x43 => self.bit_n_r(0, "E"),   //0x43  BIT 0,E
                    0x44 => self.bit_n_r(0, "H"),   //0x44  BIT 0,H
                    0x45 => self.bit_n_r(0, "L"),   //0x45  BIT 0,L
                    0x46 => self.bit_hl_pointer(0), //0x46  BIT 0,(HL)
                    0x47 => self.bit_n_r(0, "A"),   //0x47  BIT 0,A
                    0x48 => self.bit_n_r(1, "B"),   //0x48  BIT 1,B
                    0x49 => self.bit_n_r(1, "C"),   //0x49  BIT 1,C
                    0x4A => self.bit_n_r(1, "D"),   //0x4A  BIT 1,D
                    0x4B => self.bit_n_r(1, "E"),   //0x4B  BIT 1,E
                    0x4C => self.bit_n_r(1, "H"),   //0x4C  BIT 1,H
                    0x4D => self.bit_n_r(1, "L"),   //0x4D  BIT 1,L
                    0x4E => self.bit_hl_pointer(1), //0x4E  BIT 1,(HL)
                    0x4F => self.bit_n_r(1, "A"),   //0x4F  BIT 1,A
                    0x50 => self.bit_n_r(2, "B"),   //0x50 BIT 2,B
                    0x51 => self.bit_n_r(2, "C"),   //0x51  BIT 2,C
                    0x52 => self.bit_n_r(2, "D"),   //0x52  BIT 2,D
                    0x53 => self.bit_n_r(2, "E"),   //0x53  BIT 2,E
                    0x54 => self.bit_n_r(2, "H"),   //0x54  BIT 2,H
                    0x55 => self.bit_n_r(2, "L"),   //0x55  BIT 2,L
                    0x56 => self.bit_hl_pointer(2), //0x56  BIT 2,(HL)
                    0x57 => self.bit_n_r(2, "A"),   //0x57  BIT 2,A
                    0x58 => self.bit_n_r(3, "B"),   //0x58  BIT 3,B
                    0x59 => self.bit_n_r(3, "C"),   //0x59  BIT 3,C
                    0x5A => self.bit_n_r(3, "D"),   //0x5A  BIT 3,D
                    0x5B => self.bit_n_r(3, "E"),   //0x5B  BIT 3,E
                    0x5C => self.bit_n_r(3, "H"),   //0x5C  BIT 3,H
                    0x5D => self.bit_n_r(3, "L"),   //0x5D  BIT 3,L
                    0x5E => self.bit_hl_pointer(3), //0x5E  BIT 3,(HL)
                    0x5F => self.bit_n_r(3, "A"),   //0x5F  BIT 3,A
                    0x60 => self.bit_n_r(4, "B"),   //0x60 BIT 4,B
                    0x61 => self.bit_n_r(4, "C"),   //0x61  BIT 4,C
                    0x62 => self.bit_n_r(4, "D"),   //0x62  BIT 4,D
                    0x63 => self.bit_n_r(4, "E"),   //0x63  BIT 4,E
                    0x64 => self.bit_n_r(4, "H"),   //0x64  BIT 4,H
                    0x65 => self.bit_n_r(4, "L"),   //0x65  BIT 4,L
                    0x66 => self.bit_hl_pointer(4), //0x66  BIT 4,(HL)
                    0x67 => self.bit_n_r(4, "A"),   //0x67  BIT 4,A
                    0x68 => self.bit_n_r(5, "B"),   //0x68  BIT 5,B
                    0x69 => self.bit_n_r(5, "C"),   //0x69  BIT 5,C
                    0x6A => self.bit_n_r(5, "D"),   //0x6A  BIT 5,D
                    0x6B => self.bit_n_r(5, "E"),   //0x6B  BIT 5,E
                    0x6C => self.bit_n_r(5, "H"),   //0x6C  BIT 5,H
                    0x6D => self.bit_n_r(5, "L"),   //0x6D  BIT 5,L
                    0x6E => self.bit_hl_pointer(5), //0x6E  BIT 5,(HL)
                    0x6F => self.bit_n_r(5, "A"),   //0x6F  BIT 5,A
                    0x70 => self.bit_n_r(6, "B"),   //0x70 BIT 6,B
                    0x71 => self.bit_n_r(6, "C"),   //0x71  BIT 6,C
                    0x72 => self.bit_n_r(6, "D"),   //0x72  BIT 6,D
                    0x73 => self.bit_n_r(6, "E"),   //0x73  BIT 6,E
                    0x74 => self.bit_n_r(6, "H"),   //0x74  BIT 6,H
                    0x75 => self.bit_n_r(6, "L"),   //0x75  BIT 6,L
                    0x76 => self.bit_hl_pointer(6), //0x76  BIT 6,(HL)
                    0x77 => self.bit_n_r(6, "A"),   //0x77  BIT 6,A
                    0x78 => self.bit_n_r(7, "B"),   //0x78  BIT 7,B
                    0x79 => self.bit_n_r(7, "C"),   //0x79  BIT 7,C
                    0x7A => self.bit_n_r(7, "D"),   //0x7A  BIT 7,D
                    0x7B => self.bit_n_r(7, "E"),   //0x7B  BIT 7,E
                    0x7C => self.bit_n_r(7, "H"),   //0x7C  BIT 7,H
                    0x7D => self.bit_n_r(7, "L"),   //0x7D  BIT 7,L
                    0x7E => self.bit_hl_pointer(7), //0x7E  BIT 7,(HL)
                    0x7F => self.bit_n_r(7, "A"),   //0x7F  BIT 7,A
                    0x80 => self.res_n_r(0, "B"),   //0x80 RES 0,B
                    0x81 => self.res_n_r(0, "C"),   //0x81  RES 0,C
                    0x82 => self.res_n_r(0, "D"),   //0x82  RES 0,D
                    0x83 => self.res_n_r(0, "E"),   //0x83  RES 0,E
                    0x84 => self.res_n_r(0, "H"),   //0x84  RES 0,H
                    0x85 => self.res_n_r(0, "L"),   //0x85  RES 0,L
                    0x86 => self.res_hl_pointer(0), //0x86  RES 0,(HL)
                    0x87 => self.res_n_r(0, "A"),   //0x87  RES 0,A
                    0x88 => self.res_n_r(1, "B"),   //0x88  RES 1,B
                    0x89 => self.res_n_r(1, "C"),   //0x89  RES 1,C
                    0x8A => self.res_n_r(1, "D"),   //0x8A  RES 1,D
                    0x8B => self.res_n_r(1, "E"),   //0x8B  RES 1,E
                    0x8C => self.res_n_r(1, "H"),   //0x8C  RES 1,H
                    0x8D => self.res_n_r(1, "L"),   //0x8D  RES 1,L
                    0x8E => self.res_hl_pointer(1), //0x8E  RES 1,(HL)
                    0x8F => self.res_n_r(1, "A"),   //0x8F  RES 1,A
                    0x90 => self.res_n_r(2, "B"),   //0x90 RES 2,B
                    0x91 => self.res_n_r(2, "C"),   //0x91  RES 2,C
                    0x92 => self.res_n_r(2, "D"),   //0x92  RES 2,D
                    0x93 => self.res_n_r(2, "E"),   //0x93  RES 2,E
                    0x94 => self.res_n_r(2, "H"),   //0x94  RES 2,H
                    0x95 => self.res_n_r(2, "L"),   //0x95  RES 2,L
                    0x96 => self.res_hl_pointer(2), //0x96  RES 2,(HL)
                    0x97 => self.res_n_r(2, "A"),   //0x97  RES 2,A
                    0x98 => self.res_n_r(3, "B"),   //0x98  RES 3,B
                    0x99 => self.res_n_r(3, "C"),   //0x99  RES 3,C
                    0x9A => self.res_n_r(3, "D"),   //0x9A  RES 3,D
                    0x9B => self.res_n_r(3, "E"),   //0x9B  RES 3,E
                    0x9C => self.res_n_r(3, "H"),   //0x9C  RES 3,H
                    0x9D => self.res_n_r(3, "L"),   //0x9D  RES 3,L
                    0x9E => self.res_hl_pointer(3), //0x9E  RES 3,(HL)
                    0x9F => self.res_n_r(3, "A"),   //0x9F  RES 3,A
                    0xA0 => self.res_n_r(4, "B"),   //0xA0 RES 4,B
                    0xA1 => self.res_n_r(4, "C"),   //0xA1  RES 4,C
                    0xA2 => self.res_n_r(4, "D"),   //0xA2  RES 4,D
                    0xA3 => self.res_n_r(4, "E"),   //0xA3  RES 4,E
                    0xA4 => self.res_n_r(4, "H"),   //0xA4  RES 4,H
                    0xA5 => self.res_n_r(4, "L"),   //0xA5  RES 4,L
                    0xA6 => self.res_hl_pointer(4), //0xA6  RES 4,(HL)
                    0xA7 => self.res_n_r(4, "A"),   //0xA7  RES 4,A
                    0xA8 => self.res_n_r(5, "B"),   //0xA8  RES 5,B
                    0xA9 => self.res_n_r(5, "C"),   //0xA9  RES 5,C
                    0xAA => self.res_n_r(5, "D"),   //0xAA  RES 5,D
                    0xAB => self.res_n_r(5, "E"),   //0xAB  RES 5,E
                    0xAC => self.res_n_r(5, "H"),   //0xAC  RES 5,H
                    0xAD => self.res_n_r(5, "L"),   //0xAD  RES 5,L
                    0xAE => self.res_hl_pointer(5), //0xAE  RES 5,(HL)
                    0xAF => self.res_n_r(5, "A"),   //0xAF  RES 5,A
                    0xB0 => self.res_n_r(6, "B"),   //0xB0 RES 6,B
                    0xB1 => self.res_n_r(6, "C"),   //0xB1  RES 6,C
                    0xB2 => self.res_n_r(6, "D"),   //0xB2  RES 6,D
                    0xB3 => self.res_n_r(6, "E"),   //0xB3  RES 6,E
                    0xB4 => self.res_n_r(6, "H"),   //0xB4  RES 6,H
                    0xB5 => self.res_n_r(6, "L"),   //0xB5  RES 6,L
                    0xB6 => self.res_hl_pointer(6), //0xB6  RES 6,(HL)
                    0xB7 => self.res_n_r(6, "A"),   //0xB7  RES 6,A
                    0xB8 => self.res_n_r(7, "B"),   //0xB8  RES 7,B
                    0xB9 => self.res_n_r(7, "C"),   //0xB9  RES 7,C
                    0xBA => self.res_n_r(7, "D"),   //0xBA  RES 7,D
                    0xBB => self.res_n_r(7, "E"),   //0xBB  RES 7,E
                    0xBC => self.res_n_r(7, "H"),   //0xBC  RES 7,H
                    0xBD => self.res_n_r(7, "L"),   //0xBD  RES 7,L
                    0xBE => self.res_hl_pointer(7), //0xBE  RES 7,(HL)
                    0xBF => self.res_n_r(7, "A"),   //0xBF  RES 7,A
                    0xC0 => self.set_n_r(0, "B"),   //0xC0 SET 0,B
                    0xC1 => self.set_n_r(0, "C"),   //0xC1  SET 0,C
                    0xC2 => self.set_n_r(0, "D"),   //0xC2  SET 0,D
                    0xC3 => self.set_n_r(0, "E"),   //0xC3  SET 0,E
                    0xC4 => self.set_n_r(0, "H"),   //0xC4  SET 0,H
                    0xC5 => self.set_n_r(0, "L"),   //0xC5  SET 0,L
                    0xC6 => self.set_hl_pointer(0), //0xC6  SET 0,(HL)
                    0xC7 => self.set_n_r(0, "A"),   //0xC7  SET 0,A
                    0xC8 => self.set_n_r(1, "B"),   //0xC8  SET 1,B
                    0xC9 => self.set_n_r(1, "C"),   //0xC9  SET 1,C
                    0xCA => self.set_n_r(1, "D"),   //0xCA  SET 1,D
                    0xCB => self.set_n_r(1, "E"),   //0xCB  SET 1,E
                    0xCC => self.set_n_r(1, "H"),   //0xCC  SET 1,H
                    0xCD => self.set_n_r(1, "L"),   //0xCD  SET 1,L
                    0xCE => self.set_hl_pointer(1), //0xCE  SET 1,(HL)
                    0xCF => self.set_n_r(1, "A"),   //0xCF  SET 1,A
                    0xD0 => self.set_n_r(2, "B"),   //0xD0 SET 2,B
                    0xD1 => self.set_n_r(2, "C"),   //0xD1  SET 2,C
                    0xD2 => self.set_n_r(2, "D"),   //0xD2  SET 2,D
                    0xD3 => self.set_n_r(2, "E"),   //0xD3  SET 2,E
                    0xD4 => self.set_n_r(2, "H"),   //0xD4  SET 2,H
                    0xD5 => self.set_n_r(2, "L"),   //0xD5  SET 2,L
                    0xD6 => self.set_hl_pointer(2), //0xD6  SET 2,(HL)
                    0xD7 => self.set_n_r(2, "A"),   //0xD7  SET 2,A
                    0xD8 => self.set_n_r(3, "B"),   //0xD8  SET 3,B
                    0xD9 => self.set_n_r(3, "C"),   //0xD9  SET 3,C
                    0xDA => self.set_n_r(3, "D"),   //0xDA  SET 3,D
                    0xDB => self.set_n_r(3, "E"),   //0xDB  SET 3,E
                    0xDC => self.set_n_r(3, "H"),   //0xDC  SET 3,H
                    0xDD => self.set_n_r(3, "L"),   //0xDD  SET 3,L
                    0xDE => self.set_hl_pointer(3), //0xDE  SET 3,(HL)
                    0xDF => self.set_n_r(3, "A"),   //0xDF  SET 3,A
                    0xE0 => self.set_n_r(4, "B"),   //0xE0 SET 4,B
                    0xE1 => self.set_n_r(4, "C"),   //0xE1  SET 4,C
                    0xE2 => self.set_n_r(4, "D"),   //0xE2  SET 4,D
                    0xE3 => self.set_n_r(4, "E"),   //0xE3  SET 4,E
                    0xE4 => self.set_n_r(4, "H"),   //0xE4  SET 4,H
                    0xE5 => self.set_n_r(4, "L"),   //0xE5  SET 4,L
                    0xE6 => self.set_hl_pointer(4), //0xE6  SET 4,(HL)
                    0xE7 => self.set_n_r(4, "A"),   //0xE7  SET 4,A
                    0xE8 => self.set_n_r(5, "B"),   //0xE8  SET 5,B
                    0xE9 => self.set_n_r(5, "C"),   //0xE9  SET 5,C
                    0xEA => self.set_n_r(5, "D"),   //0xEA  SET 5,D
                    0xEB => self.set_n_r(5, "E"),   //0xEB  SET 5,E
                    0xEC => self.set_n_r(5, "H"),   //0xEC  SET 5,H
                    0xED => self.set_n_r(5, "L"),   //0xED  SET 5,L
                    0xEE => self.set_hl_pointer(5), //0xEE  SET 5,(HL)
                    0xEF => self.set_n_r(5, "A"),   //0xEF  SET 5,A
                    0xF0 => self.set_n_r(6, "B"),   //0xF0 SET 6,B
                    0xF1 => self.set_n_r(6, "C"),   //0xF1  SET 6,C
                    0xF2 => self.set_n_r(6, "D"),   //0xF2  SET 6,D
                    0xF3 => self.set_n_r(6, "E"),   //0xF3  SET 6,E
                    0xF4 => self.set_n_r(6, "H"),   //0xF4  SET 6,H
                    0xF5 => self.set_n_r(6, "L"),   //0xF5  SET 6,L
                    0xF6 => self.set_hl_pointer(6), //0xF6  SET 6,(HL)
                    0xF7 => self.set_n_r(6, "A"),   //0xF7  SET 6,A
                    0xF8 => self.set_n_r(7, "B"),   //0xF8  SET 7,B
                    0xF9 => self.set_n_r(7, "C"),   //0xF9  SET 7,C
                    0xFA => self.set_n_r(7, "D"),   //0xFA  SET 7,D
                    0xFB => self.set_n_r(7, "E"),   //0xFB  SET 7,E
                    0xFC => self.set_n_r(7, "H"),   //0xFC  SET 7,H
                    0xFD => self.set_n_r(7, "L"),   //0xFD  SET 7,L
                    0xFE => self.set_hl_pointer(7), //0xFE  SET 7,(HL)
                    0xFF => self.set_n_r(7, "A"),   //0xFF  SET 7,A
                    _ => return Err(instruction),
                }
            } else {
                match instruction.opcode {
                    0 => {}                                                  //0x00 NOP
                    0x01 => self.ld_nn(instruction.operands, "BC"),          //0x01 LD BC, d16
                    0x02 => self.ld_address_value("BC", "A"),                //0x02 LD (BC), A
                    0x03 => self.inc_nn("BC"),                               //0x03 INC BC
                    0x04 => self.inc("B"),                                   //0x04 INC B
                    0x05 => self.dec("B"),                                   //0x05 DEC B
                    0x06 => self.ld_r_d8("B", instruction),                  //0x06 LD B,d8
                    0x07 => self.rlc_r("A", false),                          //0x07 RLCA
                    0x08 => self.ld_a16_pointer_sp(instruction),             //0x08 LD (a16),SP
                    0x09 => self.add_hl_n("BC"),                             //0x09 ADD HL, BC
                    0x0A => self.ld_a_address("BC"),                         //0x0A LD A,(BC)
                    0x0B => self.dec_nn("BC"),                               //0x0B DEC BC
                    0x0C => self.inc("C"),                                   //0x0C INC C
                    0x0D => self.dec("C"),                                   //0x0D DEC C
                    0x0E => self.ld_r_d8("C", instruction),                  //0x0E LD C,d8
                    0x0F => self.rrc_r("A", false),                          //0x0F RRCA
                    0x10 => {}                                               //0x10 STOP
                    0x11 => self.ld_nn(instruction.operands, "DE"),          //0x11 LD DE, d16
                    0x12 => self.ld_address_value("DE", "A"),                //0x12 LD (DE),A
                    0x13 => self.inc_nn("DE"),                               //0x13 INC DE
                    0x14 => self.inc("D"),                                   //0x14 INC D
                    0x15 => self.dec("D"),                                   //0x15 DEC D
                    0x16 => self.ld_r_d8("D", instruction),                  //0x16 LD D,d8
                    0x17 => self.rl_r("A", false),                           //0x17 RLA
                    0x18 => self.jr_r8(instruction, JumpCondition::None),    //0x18 JR r8
                    0x19 => self.add_hl_n("DE"),                             //0x19 ADD HL, DE
                    0x1A => self.ld_a_address("DE"),                         //0x1A LD A,(DE)
                    0x1B => self.dec_nn("DE"),                               //0x1B DEC DE
                    0x1C => self.inc("E"),                                   //0x1C INC E
                    0x1D => self.dec("E"),                                   //0x1D DEC E
                    0x1E => self.ld_r_d8("E", instruction),                  //0x1E LD E,d8
                    0x1F => self.rr_r("A", false),                           //0x1F RRA
                    0x20 => self.jr_r8(instruction, JumpCondition::NotZero), //0x20 JR NZ,r8
                    0x21 => self.ld_nn(instruction.operands, "HL"),          //0x21 LD HL, d16
                    0x22 => self.ld_hl_pointer_dec_inc_a(true),              //0x22 LD (HL+), A
                    0x23 => self.inc_nn("HL"),                               //0x23 INC HL
                    0x24 => self.inc("H"),                                   //0x24 INC H
                    0x25 => self.dec("H"),                                   //0x25 DEC H
                    0x26 => self.ld_r_d8("H", instruction),                  //0x26 LD H,d8
                    0x27 => self.daa(),                                      //0x27 DAA
                    0x28 => self.jr_r8(instruction, JumpCondition::Zero),    //0x28 JR Z,r8
                    0x29 => self.add_hl_n("HL"),                             //0x29 ADD HL, HL
                    0x2A => {
                        self.ld_a_address("HL");
                        self.inc_nn("HL")
                    } //0x2A LD A,(HL+)
                    0x2B => self.dec_nn("HL"),                               //0x2B DEC HL
                    0x2C => self.inc("L"),                                   //0x2C INC L
                    0x2D => self.dec("L"),                                   //0x2D DEC L
                    0x2E => self.ld_r_d8("L", instruction),                  //0x2E LD L,d8
                    0x2F => self.cpl(),                                      //0x2F CPL
                    0x30 => self.jr_r8(instruction, JumpCondition::NotCarry), //0x30 JR NC,r8
                    0x31 => self.ld_nn(instruction.operands, "SP"),          //0x31 LD SP, d16
                    0x32 => self.ld_hl_pointer_dec_inc_a(false),             //0x32 LD (HL-), A
                    0x33 => self.inc_nn("SP"),                               //0x33 INC SP
                    0x34 => self.inc_hl_pointer(),                           //0x34 INC (HL)
                    0x35 => self.dec_hl_pointer(),                           //0x35 DEC (HL)
                    0x36 => self.ld_hl_pointer_d8(instruction),              //0x36 LD (HL),d8
                    0x37 => self.scf(),                                      //0x37 SCF
                    0x38 => self.jr_r8(instruction, JumpCondition::Carry),   //0x38 JR C,r8
                    0x39 => self.add_hl_n("SP"),                             //0x39 ADD HL, SP
                    0x3A => {
                        self.ld_a_address("HL");
                        self.dec_nn("HL")
                    } //0x3A LD A,(HL-)
                    0x3B => self.dec_nn("SP"),                               //0x3B DEC SP
                    0x3C => self.inc("A"),                                   //0x3C INC A
                    0x3D => self.dec("A"),                                   //0x3D DEC A
                    0x3E => self.ld_r_d8("A", instruction),                  //0x3E LD a,d8
                    0x3F => self.ccf(),                                      //0x3F CCF
                    0x40 => self.ld_r_r("B", "B"),                           //0x40 LD B,B
                    0x41 => self.ld_r_r("C", "B"),                           //0x41 LD B,C
                    0x42 => self.ld_r_r("D", "B"),                           //0x42 LD B,D
                    0x43 => self.ld_r_r("E", "B"),                           //0x43 LD B,E
                    0x44 => self.ld_r_r("H", "B"),                           //0x44 LD B,H
                    0x45 => self.ld_r_r("L", "B"),                           //0x45 LD B,L
                    0x46 => self.ld_r_hl("B"),                               //0x46 LD B,(HL)
                    0x47 => self.ld_r_r("A", "B"),                           //0x47 LD B,A
                    0x48 => self.ld_r_r("B", "C"),                           //0x48 LD C,B
                    0x49 => self.ld_r_r("C", "C"),                           //0x49 LD C,C
                    0x4A => self.ld_r_r("D", "C"),                           //0x4A LD C,D
                    0x4B => self.ld_r_r("E", "C"),                           //0x4B LD C,E
                    0x4C => self.ld_r_r("H", "C"),                           //0x4C LD C,H
                    0x4D => self.ld_r_r("L", "C"),                           //0x4D LD C,L
                    0x4E => self.ld_r_hl("C"),                               //0x4E LD C,(HL)
                    0x4F => self.ld_r_r("A", "C"),                           //0x4F LD C,A
                    0x50 => self.ld_r_r("B", "D"),                           //0x50 LD D,B
                    0x51 => self.ld_r_r("C", "D"),                           //0x51 LD D,C
                    0x52 => self.ld_r_r("D", "D"),                           //0x52 LD D,D
                    0x53 => self.ld_r_r("E", "D"),                           //0x53 LD D,E
                    0x54 => self.ld_r_r("H", "D"),                           //0x54 LD D,H
                    0x55 => self.ld_r_r("L", "D"),                           //0x55 LD D,L
                    0x56 => self.ld_r_hl("D"),                               //0x56 LD D,(HL)
                    0x57 => self.ld_r_r("A", "D"),                           //0x57 LD D,A
                    0x58 => self.ld_r_r("B", "E"),                           //0x58 LD E,B
                    0x59 => self.ld_r_r("C", "E"),                           //0x59 LD E,C
                    0x5A => self.ld_r_r("D", "E"),                           //0x5A LD E,D
                    0x5B => self.ld_r_r("E", "E"),                           //0x5B LD E,E
                    0x5C => self.ld_r_r("H", "E"),                           //0x5C LD E,H
                    0x5D => self.ld_r_r("L", "E"),                           //0x5D LD E,L
                    0x5E => self.ld_r_hl("E"),                               //0x5E LD E,(HL)
                    0x5F => self.ld_r_r("A", "E"),                           //0x5F LD E,A
                    0x60 => self.ld_r_r("B", "H"),                           //0x60 LD H,B
                    0x61 => self.ld_r_r("C", "H"),                           //0x61 LD H,C
                    0x62 => self.ld_r_r("D", "H"),                           //0x62 LD H,D
                    0x63 => self.ld_r_r("E", "H"),                           //0x63 LD H,E
                    0x64 => self.ld_r_r("H", "H"),                           //0x64 LD H,H
                    0x65 => self.ld_r_r("L", "H"),                           //0x65 LD H,L
                    0x66 => self.ld_r_hl("H"),                               //0x66 LD H,(HL)
                    0x67 => self.ld_r_r("A", "H"),                           //0x67 LD H,A
                    0x68 => self.ld_r_r("B", "L"),                           //0x68 LD L,B
                    0x69 => self.ld_r_r("C", "L"),                           //0x69 LD L,C
                    0x6A => self.ld_r_r("D", "L"),                           //0x6A LD L,D
                    0x6B => self.ld_r_r("E", "L"),                           //0x6B LD L,E
                    0x6C => self.ld_r_r("H", "L"),                           //0x6C LD L,H
                    0x6D => self.ld_r_r("L", "L"),                           //0x6D LD L,L
                    0x6E => self.ld_r_hl("L"),                               //0x6E LD L,(HL)
                    0x6F => self.ld_r_r("A", "L"),                           //0x6F LD L,A
                    0x70 => self.ld_address_value("HL", "B"),                //0x70 LD (HL),B
                    0x71 => self.ld_address_value("HL", "C"),                //0x71 LD (HL),C
                    0x72 => self.ld_address_value("HL", "D"),                //0x72 LD (HL),D
                    0x73 => self.ld_address_value("HL", "E"),                //0x73 LD (HL),E
                    0x74 => self.ld_address_value("HL", "H"),                //0x74 LD (HL),H
                    0x75 => self.ld_address_value("HL", "L"),                //0x75 LD (HL),L
                    0x76 => self.is_halted = true,                           //0x76 HALT
                    0x77 => self.ld_address_value("HL", "A"),                //0x77 LD (HL),A
                    0x78 => self.ld_r_r("B", "A"),                           //0x78 LD A,B
                    0x79 => self.ld_r_r("C", "A"),                           //0x79 LD A,C
                    0x7A => self.ld_r_r("D", "A"),                           //0x7A LD A,D
                    0x7B => self.ld_r_r("E", "A"),                           //0x7B LD A,E
                    0x7C => self.ld_r_r("H", "A"),                           //0x7C LD A,H
                    0x7D => self.ld_r_r("L", "A"),                           //0x7D LD A,L
                    0x7E => self.ld_r_hl("A"),                               //0x7E LD A,(HL)
                    0x7F => self.ld_r_r("A", "A"),                           //0x7F LD A,A
                    0x80 => self.add_a_r("B"),                               //0x80 ADD A,B
                    0x81 => self.add_a_r("C"),                               //0x81 ADD A,C
                    0x82 => self.add_a_r("D"),                               //0x82 ADD A,D
                    0x83 => self.add_a_r("E"),                               //0x83 ADD A,E
                    0x84 => self.add_a_r("H"),                               //0x84 ADD A,H
                    0x85 => self.add_a_r("L"),                               //0x85 ADD A,L
                    0x86 => self.add_a_hl(),                                 //0x86 ADD A,(HL)
                    0x87 => self.add_a_r("A"),                               //0x87 ADD A,A
                    0x88 => self.adc_a_r("B"),                               //0x88 ADC A,B
                    0x89 => self.adc_a_r("C"),                               //0x89 ADC A,C
                    0x8A => self.adc_a_r("D"),                               //0x8A ADC A,D
                    0x8B => self.adc_a_r("E"),                               //0x8B ADC A,E
                    0x8C => self.adc_a_r("H"),                               //0x8C ADC A,H
                    0x8D => self.adc_a_r("L"),                               //0x8D ADC A,L
                    0x8E => self.adc_a_hl(),                                 //0x8E ADC A,(HL)
                    0x8F => self.adc_a_r("A"),                               //0x8F ADC A,A
                    0x90 => self.sub_a_r("B"),                               //0x90 SUB B
                    0x91 => self.sub_a_r("C"),                               //0x91 SUB C
                    0x92 => self.sub_a_r("D"),                               //0x92 SUB D
                    0x93 => self.sub_a_r("E"),                               //0x93 SUB B
                    0x94 => self.sub_a_r("H"),                               //0x94 SUB H
                    0x95 => self.sub_a_r("L"),                               //0x95 SUB L
                    0x96 => self.sub_a_hl(),                                 //0x96 SUB (HL)
                    0x97 => self.sub_a_r("A"),                               //0x97 SUB A
                    0x98 => self.sbc_a_r("B"),                               //0x98 SBC A,B
                    0x99 => self.sbc_a_r("C"),                               //0x99 SBC A,C
                    0x9A => self.sbc_a_r("D"),                               //0x9A SBC A,D
                    0x9B => self.sbc_a_r("E"),                               //0x9B SBC A,E
                    0x9C => self.sbc_a_r("H"),                               //0x9C SBC A,H
                    0x9D => self.sbc_a_r("L"),                               //0x9D SBC A,L
                    0x9E => self.sbc_a_hl(),                                 //0x9E SBC A,(HL)
                    0x9F => self.sbc_a_r("A"),                               //0x9F SBC A,A
                    0xA0 => self.and_a_r("B"),                               //0xA0 AND B
                    0xA1 => self.and_a_r("C"),                               //0xA1 AND C
                    0xA2 => self.and_a_r("D"),                               //0xA2 AND D
                    0xA3 => self.and_a_r("E"),                               //0xA3 AND E
                    0xA4 => self.and_a_r("H"),                               //0xA4 AND H
                    0xA5 => self.and_a_r("L"),                               //0xA5 AND L
                    0xA6 => self.and_a_hl(),                                 //0xA6 AND (HL)
                    0xA7 => self.and_a_r("A"),                               //0xA7 AND A
                    0xA8 => self.xor_a_r("B"),                               //0xA8 XOR B
                    0xA9 => self.xor_a_r("C"),                               //0xA9 XOR C
                    0xAA => self.xor_a_r("D"),                               //0xAA XOR D
                    0xAB => self.xor_a_r("E"),                               //0xAB XOR E
                    0xAC => self.xor_a_r("H"),                               //0xAC XOR H
                    0xAD => self.xor_a_r("L"),                               //0xAD XOR L
                    0xAE => self.xor_a_hl(),                                 //0xAE XOR (HL)
                    0xAF => self.xor_a_r("A"),                               //0xAF XOR A
                    0xB0 => self.or_a_r("B"),                                //0xB0 OR B
                    0xB1 => self.or_a_r("C"),                                //0xB1 OR C
                    0xB2 => self.or_a_r("D"),                                //0xB2 OR D
                    0xB3 => self.or_a_r("E"),                                //0xB3 OR E
                    0xB4 => self.or_a_r("H"),                                //0xB4 OR H
                    0xB5 => self.or_a_r("L"),                                //0xB5 OR L
                    0xB6 => self.or_a_hl(),                                  //0xB6 OR (HL)
                    0xB7 => self.or_a_r("A"),                                //0xB7 OR A
                    0xB8 => self.cp_a_r("B"),                                //0xB8 CP B
                    0xB9 => self.cp_a_r("C"),                                //0xB9 CP C
                    0xBA => self.cp_a_r("D"),                                //0xBA CP D
                    0xBB => self.cp_a_r("E"),                                //0xBB CP E
                    0xBC => self.cp_a_r("H"),                                //0xBC CP H
                    0xBD => self.cp_a_r("L"),                                //0xBD CP L
                    0xBE => self.cp_a_hl(),                                  //0xBE CP (HL)
                    0xBF => self.cp_a_r("A"),                                //0xBF CP A
                    0xC0 => self.ret(JumpCondition::NotZero, false),         //0xC0 RET NZ
                    0xC1 => self.pop_rr("BC"),                               //0xC1 POP BC
                    0xC2 => self.jp_a16(instruction, JumpCondition::NotZero), //0xC2 JP NZ,a16
                    0xC3 => self.jp_a16(instruction, JumpCondition::None),   //0xC3 JP a16
                    0xC4 => self.call_a16(instruction, JumpCondition::NotZero), //0xC4 CALL NZ,a16
                    0xC5 => self.push_rr("BC"),                              //0xC5 PUSH BC
                    0xC6 => self.add_a_n(instruction.operands),              //0xC6 ADD A,d8
                    0xC7 => self.rst(0x0),                                   //0xC7 RST 00H
                    0xC8 => self.ret(JumpCondition::Zero, false),            //0xC8 RET Z
                    0xC9 => self.ret(JumpCondition::None, false),            //0xC9 RET
                    0xCA => self.jp_a16(instruction, JumpCondition::Zero),   //0xCA JP Z,a16
                    0xCB => {}                                               //0xCB CB PREFIX
                    0xCC => self.call_a16(instruction, JumpCondition::Zero), //0xCC CALL Z,a16
                    0xCD => self.call_a16(instruction, JumpCondition::None), //0xCD CALL a16
                    0xCE => self.adc_a_d8(instruction),                      //0xCE ADC A,d8
                    0xCF => self.rst(0x8),                                   //0xCF RST 08H
                    0xD0 => self.ret(JumpCondition::NotCarry, false),        //0xD0 RET NC
                    0xD1 => self.pop_rr("DE"),                               //0xD1 POP DE
                    0xD2 => self.jp_a16(instruction, JumpCondition::NotCarry), //0xD2 JP NC,a16
                    0xD3 => (),                                              //0xD3 UNDEFINED
                    0xD4 => self.call_a16(instruction, JumpCondition::NotCarry), //0xD4 CALL NC,a16
                    0xD5 => self.push_rr("DE"),                              //0xD5 PUSH DE
                    0xD6 => self.sub_a_n(instruction.operands),              //0xD6 SUB d8
                    0xD7 => self.rst(0x10),                                  //0xD7 RST 10H
                    0xD8 => self.ret(JumpCondition::Carry, false),           //0xD8 RET C
                    0xD9 => self.ret(JumpCondition::None, true),             //0xD9 RETI
                    0xDA => self.jp_a16(instruction, JumpCondition::Carry),  //0xDA JP C,a16
                    0xDB => (),                                              //0xDB UNDEFINED
                    0xDC => self.call_a16(instruction, JumpCondition::Carry), //0xDC CALL C,a16
                    0xDD => (),                                              //0xDD UNDEFINED
                    0xDE => self.sbc_a_d8(instruction),                      //0xDE SBC A,d8
                    0xDF => self.rst(0x18),                                  //0xDF RST 18H
                    0xE0 => self.ldh_a8_a(instruction),                      //0xE0 LDH (a8),A
                    0xE1 => self.pop_rr("HL"),                               //0xE1 POP HL
                    0xE2 => self.ld_c_pointer_a(),                           //0xE2 LD (C),A
                    0xE3 => (),                                              //0xE3 UNDEFINED
                    0xE4 => (),                                              //0xE4 UNDEFINED
                    0xE5 => self.push_rr("HL"),                              //0xE5 PUSH HL
                    0xE6 => self.and_a_n(instruction.operands),              //0xE6 AND d8
                    0xE7 => self.rst(0x20),                                  //0xE7 RST 20H
                    0xE8 => self.add_sp_r8(instruction),                     //0xE8 ADD SP,r8
                    0xE9 => self.jp_hl(),                                    //0xE9 JP (HL)
                    0xEA => self.ld_a16_pointer_a(instruction),              //0xEA LD (a16),A
                    0xEB => (),                                              //0xEB UNDEFINED
                    0xEC => (),                                              //0xEC UNDEFINED
                    0xED => (),                                              //0xED UNDEFINED
                    0xEE => self.xor_a_d8(instruction),                      //0xEE XOR d8
                    0xEF => self.rst(0x28),                                  //0xEF RST 28H
                    0xF0 => self.ldh_a_a8(instruction),                      //0xF0 LDH A,(a8)
                    0xF1 => self.pop_rr("AF"),                               //0xF1 POP AF
                    0xF2 => self.ld_a_c_pointer(),                           //0xF2 LD A,(C)
                    0xF3 => self.disable_interrupt(),                        //0xF3 DI
                    0xF4 => (),                                              //0xF4 UNDEFINED
                    0xF5 => self.push_rr("AF"),                              //0xF5 PUSH AF
                    0xF6 => self.or_a_n(instruction.operands),               //0xF6 OR d8
                    0xF7 => self.rst(0x30),                                  //0xF7 RST 30H
                    0xF8 => self.ld_hl_sp_r8(instruction),                   //0xF8 LD HL,SP+r8
                    0xF9 => self.ld_sp_hl(),                                 //0xF9 LD SP,HL
                    0xFA => self.ld_a_a16_pointer(instruction),              //0xFA LD A,(a16)
                    0xFB => self.enable_interrupt(),                         //0xFB EI
                    0xFC => (),                                              //0xFC UNDEFINED
                    0xFD => (),                                              //0xFD UNDEFINED
                    0xFE => self.cp_a_d8(instruction),                       //0xFE CP d8
                    0xFF => self.rst(0x38),                                  //0xFF RST 38H
                    _ => return Err(instruction),
                }
            }
            Ok(())
        }
    }
}
