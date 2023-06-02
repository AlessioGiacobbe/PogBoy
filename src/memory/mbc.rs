pub mod mbc {
    use std::fmt::{Display, Formatter};

    #[derive(Clone, Debug)]
    pub struct MbcNone {
        rom: Vec<u8>,
        ram: Vec<u8>
    }

    #[derive(Clone, Debug)]
    pub struct Mbc1 {
        rom: Vec<u8>,
        ram: Vec<u8>,
        selected_rom_bank: usize,
        selected_ram_bank: usize,
        ram_selected: bool,
        ram_enabled: bool,
    }

    #[derive(Clone, Debug)]
    pub struct Mbc2 {
        rom: Vec<u8>,
        ram: Vec<u8>,
        selected_rom_bank: usize,
        ram_enabled: bool,
    }

    #[derive(Clone, Debug)]
    pub struct Mbc3 {

    }

    #[derive(Clone, Debug)]
    pub struct Mbc5 {

    }

    #[derive(Clone, Debug)]
    pub enum MbcType {
        None(MbcNone),
        Mbc1(Mbc1),
        Mbc2(Mbc2),
        Mbc3(Mbc3),
        Mbc5(Mbc5)
    }

    impl MbcNone {
        pub fn new(rom: Vec<u8>) -> Self {
            MbcNone {
                rom,
                ram: vec![0; 0x4000]
            }
        }

        pub fn read(&self, address: usize) -> u8 {
            return match address {
                0x0..=0x7FFF => self.rom[address],
                0x8000..=0xBFFF => self.ram[address-0x8000],
                _ => panic!("can't read address {}", address),
            };
        }

        pub fn write(&mut self, address: usize, value: u8) {
            if 0x7FFF < address && address < 0xC000 {
                self.ram[address-0x8000] = value;
            }
        }
    }

    impl Mbc1 {
        pub fn new(rom: Vec<u8>) -> Self {
            Mbc1 {
                rom,
                ram: vec![0; 0x8000],
                selected_rom_bank: 0,
                selected_ram_bank: 0,
                ram_selected: false,
                ram_enabled: false,
            }
        }

        pub fn read(&self, address: usize) -> u8 {
            return match address {
                0x0..=0x3FFF => self.rom[address],
                0x4000..=0x7FFF => {
                    let rom_bank = self.selected_rom_bank.max(1);
                    let rom_bank = if rom_bank == 0x20 || rom_bank == 0x40 || rom_bank == 0x60 { rom_bank + 1 } else { rom_bank };
                    let base_address = rom_bank as usize * 0x4000;
                    let offset = address - 0x4000;
                    let address = (base_address + offset) & (self.rom.len() - 1);
                    self.rom[address]
                },
                0xA000..=0xBFFF => {
                    if self.ram_enabled {
                        let base = self.selected_ram_bank as usize * 0x2000;
                        let offset = address - 0xa000;
                        let addr = (base + offset) & (self.rom.len() - 1);
                        return self.ram[addr];
                    }
                    0
                },
                _ => panic!("can't read address {}", address),
            };
        }

        pub fn write(&mut self, address: usize, value: u8) {
            match address {
                0..=0x1FFF => {
                    self.ram_enabled = value & 0xF == 0x0A;
                },
                0x2000..=0x3FFF => {
                    self.selected_rom_bank = (self.selected_rom_bank & !0x1F) | (value as usize & 0x1F);
                },
                0x4000..=0x5FFF => {
                    if self.ram_selected {
                        self.selected_ram_bank = value as usize & 0x3;
                    } else {
                        self.selected_rom_bank = (self.selected_rom_bank & !0x60) | ((value as usize & 0x3) << 5);
                    }
                },
                0x6000..=0x7FFF => {
                    self.ram_selected = value != 0x00;
                },
                0xA000..=0xBFFF => {
                    if self.ram_enabled {
                        let base = self.selected_ram_bank * 0x2000;
                        let offset = address as usize - 0xA000;
                        self.ram[base + offset] = value;
                    }
                },
                _ => {}
            }
        }
    }


    impl Mbc2 {
        pub fn new(rom: Vec<u8>) -> Self {
            Mbc2 {
                rom,
                ram: vec![0; 0x200],
                selected_rom_bank: 1,
                ram_enabled: false,
            }
        }

        pub fn read(&self, address: usize) -> u8 {
            1
        }

        pub fn write(&mut self, address: usize, value: u8) {

        }
    }


    impl Mbc3 {
        pub fn new(rom: Vec<u8>) -> Self {
            Mbc3 {}
        }

        pub fn read(&self, address: usize) -> u8 {
            1
        }

        pub fn write(&mut self, address: usize, value: u8) {

        }
    }

    impl Mbc5 {
        pub fn new(rom: Vec<u8>) -> Self {
            Mbc5 {}
        }

        pub fn read(&self, address:usize) -> u8 {
            1
        }

        pub fn write(&mut self, address: usize, value: u8) {

        }
    }


    impl MbcType {
        pub fn write(&mut self, address: usize, value: u8) {
            match self {
                MbcType::None(mbc) => mbc.write(address, value),
                MbcType::Mbc1(mbc) => mbc.write(address, value),
                MbcType::Mbc2(mbc) => mbc.write(address, value),
                MbcType::Mbc3(mbc) => mbc.write(address, value),
                MbcType::Mbc5(mbc) => mbc.write(address, value)
            }
        }

        pub fn read(&self, address: usize) -> u8 {
            match self {
                MbcType::None(mbc) => mbc.read(address),
                MbcType::Mbc1(mbc) => mbc.read(address),
                MbcType::Mbc2(mbc) => mbc.read(address),
                MbcType::Mbc3(mbc) => mbc.read(address),
                MbcType::Mbc5(mbc) => mbc.read(address)
            }
        }

        pub fn new(mbc_code: u8, rom: Vec<u8>) -> Self {
            match mbc_code {
                0x00 => MbcType::None(MbcNone::new(rom)),
                0x01..=0x03 => MbcType::Mbc1(Mbc1::new(rom)),
                0x05..=0x06 => MbcType::Mbc2(Mbc2::new(rom)),
                0x0F..=0x13 => MbcType::Mbc3(Mbc3::new(rom)),
                0x19..=0x1E => MbcType::Mbc5(Mbc5::new(rom)),
                _ => panic!("unhandled {} mbc code (not implemented)", mbc_code)
            }
        }
    }
}