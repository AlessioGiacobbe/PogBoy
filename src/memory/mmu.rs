pub mod mmu {
    use std::collections::HashMap;
    use std::fmt::{Debug};
    use std::fs;
    use serde_json::Value;
    use crate::ppu::ppu::PPU;
    use crate::io::gamepad;
    use crate::memory::cartridge::cartridge::Cartridge;
    use crate::memory::op_codes_parser::op_codes_parser::{get_instructions_from_json, Instruction, Operand};

    const INSTRUCTIONS_PREFIX: u8 = 0xCB;

    #[derive(Debug)]
    pub struct MMU<'a> {
        pub bios: [u8; 256],
        pub cartridge: Cartridge,
        pub PPU: &'a mut PPU,
        pub gamepad: gamepad::gamepad::gamepad,
        pub external_ram: [u8; 0x2000],
        pub work_ram: [u8; 0x2000],
        pub io_registers: [u8; 0x100],
        pub high_ram: [u8; 0x80],
        pub is_past_bios: bool,

        pub interrupt_master_enabled: u8, //ime
        pub interrupt_enabled: u8, //ie
        pub interrupt_flag: u8, //if

        pub unprefixed_op_codes: HashMap<u8, Instruction>,
        pub prefixed_op_codes: HashMap<u8, Instruction>,
    }


    impl<'a> MMU<'_> {

        pub fn new(Cartridge: Option<Cartridge>, PPU: &mut PPU) -> MMU {
            let op_codes_content = fs::read_to_string("./src/opcodes.json").expect("error reading file");
            let json_op_codes: Value = serde_json::from_str(&op_codes_content).unwrap();

            let unprefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
            let prefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"cbprefixed");

            MMU {
                bios: [0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
                    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
                    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
                    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
                    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
                    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
                    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
                    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
                    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xF2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
                    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
                    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
                    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
                    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
                    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x4C,
                    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
                    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50],
                cartridge: Cartridge.expect("Empty cartridge"),
                PPU,
                gamepad: gamepad::gamepad::gamepad::default(),
                external_ram: [0; 0x2000],
                work_ram: [0; 0x2000],
                io_registers: [0; 0x100],
                high_ram: [0; 0x80],
                is_past_bios: false,

                interrupt_master_enabled: 0,
                interrupt_enabled: 0,
                interrupt_flag: 0,

                unprefixed_op_codes,
                prefixed_op_codes
            }
        }

        pub fn decode(&self, mut address: i32) -> (i32, Instruction) {
            let mut op_code = self.read_byte(address);
            address = address + 1;
            let instruction = {
                if op_code == INSTRUCTIONS_PREFIX {
                    op_code = self.read_byte(address);
                    address = address + 1;
                    self.prefixed_op_codes.get(&op_code).unwrap()
                }else{
                    self.unprefixed_op_codes.get(&op_code).unwrap()
                }
            };

            let new_operands: Vec<Operand> = {
                let mut new_operands: Vec<Operand> = vec![];
                for operand in instruction.operands.iter() {
                    if operand.bytes != None {
                        let bytes = operand.bytes.unwrap();
                        let mut operand_to_be_pushed = operand.clone();
                        let operand_value: u16 = match bytes {
                            1 => {
                                self.read_byte(address) as u16
                            },
                            2 => {
                                let first_byte = self.read_byte(address) as u16;
                                let second_byte = self.read_byte(address + 1) as u16;
                                (second_byte << 8) + first_byte
                            },
                            _ => panic!("no operand value")
                        };
                        operand_to_be_pushed.value = Some(operand_value);
                        new_operands.push(operand_to_be_pushed);
                        address = address + i32::from(bytes);
                    }else{
                        new_operands.push(operand.clone());
                    }
                };
                new_operands
            };

            let mut decoded_instruction = (*instruction).clone();
            decoded_instruction.operands = new_operands;
            (address, decoded_instruction)
        }

        pub fn disassemble(&self, mut address: i32, quantity: i32, current_address: i32){
            println!();
            println!("-------------");
            for _ in 0..quantity{
                let (new_address, instruction) = self.decode(address);
                if current_address == address {
                    print!("-> ");
                }
                println!("{:#04X}       {}", address, instruction);
                address = new_address;
            }
            println!("-------------");
        }


        pub fn read_byte(&self, address: i32) -> u8{
            let address = address as usize;
            return match address {
                //BIOS
                0..=0xFF => {
                    if self.is_past_bios {
                        self.cartridge.rom[address]
                    } else {
                        self.bios[address]
                    }
                }
                //ROM bank 0
                0x100..=0x3FFF => {
                    self.cartridge.rom[address]
                },
                //ROM bank 1-NN
                0x4000..=0x7FFF => {
                    self.cartridge.rom[address]
                },
                //VRAM
                0x8000..=0x9FFF => {
                    self.PPU.read_byte(address)
                },
                //External RAM
                0xA000..=0xBFFF => {
                    self.external_ram[address - 0xA000]
                },
                //WRAM (Work RAM)
                0xC000..=0xDFFF => {
                    self.work_ram[address - 0xC000]
                },
                //ECHO RAM (use is prohibited by Nintendo!)
                0xE000..=0xFDFF => {
                    panic!("tried to access echo ram")
                },
                //Sprite attribute table
                0xFE00..=0xFE9F => {
                    self.PPU.read_byte(address)
                },
                //Not usable (prohibited!)
                0xFEA0..=0xFEFF => {
                    return 0xFF;
                },
                0xFF00 => {
                    return self.gamepad.read()
                },
                0xFF0F => {
                    return self.interrupt_flag  //IF
                },
                0xFF40 => {
                    self.PPU.read_byte(address)
                }
                0xFF41 => {
                    self.PPU.read_byte(address)
                },
                0xFF42 => {
                    self.PPU.read_byte(address)
                },
                0xFF43 => {
                    self.PPU.read_byte(address)
                },
                0xFF44 => {
                    self.PPU.read_byte(address)
                },
                0xFF47 => {
                    self.PPU.read_byte(address)
                },
                0xFF50 => {
                    self.is_past_bios as u8
                },
                //I/O Registers
                0xFF01..=0xFF7F => {
                    self.io_registers[address - 0xFF00]
                },
                //High RAM
                0xFF80..=0xFFFE=> {
                    self.high_ram[address - 0xFF80]
                },
                //Master interrupt Enable register
                0xFFFF => {
                    self.interrupt_master_enabled as u8
                },
                _ => {
                    panic!("Address {} out of range!", address)
                }
            }
        }

        pub fn write_byte(&mut self, address: i32, value: u8){
            //println!("Writing byte {} to 0x{:04x}", value, address);

            let address = address as usize;
            return match address {
                //BIOS
                0..=0x0FF => {
                    self.bios[address] = value;
                }
                //ROM bank 0
                0x100..=0x3FFF => {
                    self.cartridge.rom[address] = value;
                },
                //ROM bank 1-NN
                0x4000..=0x7FFF => {
                    self.cartridge.rom[address] = value;
                },
                //VRAM
                0x8000..=0x9FFF => {
                    self.PPU.write_byte(address, value);
                },
                //External RAM
                0xA000..=0xBFFF => {
                    self.external_ram[address - 0xA000] = value;
                },
                //WRAM (Work RAM)
                0xC000..=0xDFFF => {
                    self.work_ram[address - 0xC000] = value;
                },
                //ECHO RAM (use is prohibited by Nintendo!)
                0xE000..=0xFDFF => {
                    panic!("tried to write echo ram")
                },
                //Sprite attribute table
                0xFE00..=0xFE9F => {
                    self.PPU.write_byte(address, value)
                },
                //Not usable (prohibited!)
                0xFEA0..=0xFEFF => {
                    //panic!("{} address not usable", address)
                }
                0xFF00 => {
                    self.gamepad.write(value)
                },
                //timer
                0xFF05 => {

                },
                //timer
                0xFF06 => {

                },
                //timer controller
                0xFF07 => {

                },
                0xFF0F => {
                    self.interrupt_flag = 0xE0 | value  // IF, most significant first 3 bits are always 1, hence 0xE0
                },
                //I/O Registers
                0xFF01..=0xFF04 => {
                    self.io_registers[address - 0xFF00] = value
                },
                //I/O Registers
                0xFF08..=0xFF3F => {
                    self.io_registers[address - 0xFF00] = value
                },
                //PPU special registers
                0xFF40..=0xFF4F => {
                    self.PPU.write_byte(address, value)
                },
                0xFF50 => {
                    self.is_past_bios = value == 1;
                },
                0xFF51..=0xFF7F => {
                    self.io_registers[address - 0xFF00] = value
                }
                //High RAM
                0xFF80..=0xFFFE=> {
                    self.high_ram[address - 0xFF80] = value
                },
                //Interrupt Enable register
                0xFFFF => {
                    self.interrupt_enabled = value
                },
                _ => {
                    panic!("Address {} out of range!", address)
                }
            }
        }

        pub fn write_word(&mut self, address: i32, value: u16){
            self.write_byte(address, (value & 0x00FF) as u8);
            self.write_byte(address + 1, ((value & 0xFF00) >> 8) as u8);
        }

        pub fn read_word(&mut self, address: i32) -> u16{
            let first_8_bits = self.read_byte(address as i32) as u16;
            let last_8_bits = self.read_byte((address + 1) as i32) as u16;
            (first_8_bits | last_8_bits << 8) as u16
        }
    }

}