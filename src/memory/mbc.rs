pub mod mbc {
    use std::fmt::{Display, Formatter};

    #[derive(Clone, Debug)]
    pub struct MbcNone {
        rom: Vec<u8>,
        ram: Vec<u8>,
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
        rom: Vec<u8>,
        ram: Vec<u8>,
        rom_bank: usize,
        enable: bool,
        select: u8,
        rtc_secs: u8,
        rtc_mins: u8,
        rtc_hours: u8,
        rtc_day_low: u8,
        rtc_day_high: u8,
        epoch: u64,
        prelatch: bool,
    }

    #[derive(Clone, Debug)]
    pub struct Mbc5 {
        rom: Vec<u8>,
        ram: Vec<u8>,
        selected_rom_bank: usize,
        selected_ram_bank: usize,
        ram_enabled: bool,
    }

    #[derive(Clone, Debug)]
    pub enum MbcType {
        None(MbcNone),
        Mbc1(Mbc1),
        Mbc2(Mbc2),
        Mbc3(Mbc3),
        Mbc5(Mbc5),
    }

    impl MbcNone {
        pub fn new(rom: Vec<u8>) -> Self {
            MbcNone {
                rom,
                ram: vec![0; 0x4000],
            }
        }

        pub fn read(&self, address: usize) -> u8 {
            return match address {
                0x0..=0x7FFF => self.rom[address],
                0x8000..=0xBFFF => self.ram[address - 0x8000],
                _ => panic!("can't read address {}", address),
            };
        }

        pub fn write(&mut self, address: usize, value: u8) {
            if 0x7FFF < address && address < 0xC000 {
                self.ram[address - 0x8000] = value;
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
                    let rom_bank = if rom_bank == 0x20 || rom_bank == 0x40 || rom_bank == 0x60 {
                        rom_bank + 1
                    } else {
                        rom_bank
                    };
                    let base_address = rom_bank as usize * 0x4000;
                    let offset = address - 0x4000;
                    let address = (base_address + offset) & (self.rom.len() - 1);
                    self.rom[address]
                }
                0xA000..=0xBFFF => {
                    if self.ram_enabled {
                        let base = self.selected_ram_bank as usize * 0x2000;
                        let offset = address - 0xa000;
                        let addr = (base + offset) & (self.rom.len() - 1);
                        return self.ram[addr];
                    }
                    0
                }
                _ => panic!("can't read address {}", address),
            };
        }

        pub fn write(&mut self, address: usize, value: u8) {
            match address {
                0..=0x1FFF => {
                    self.ram_enabled = value & 0xF == 0x0A;
                }
                0x2000..=0x3FFF => {
                    self.selected_rom_bank =
                        (self.selected_rom_bank & !0x1F) | (value as usize & 0x1F);
                }
                0x4000..=0x5FFF => {
                    if self.ram_selected {
                        self.selected_ram_bank = value as usize & 0x3;
                    } else {
                        self.selected_rom_bank =
                            (self.selected_rom_bank & !0x60) | ((value as usize & 0x3) << 5);
                    }
                }
                0x6000..=0x7FFF => {
                    self.ram_selected = value != 0x00;
                }
                0xA000..=0xBFFF => {
                    if self.ram_enabled {
                        let base = self.selected_ram_bank * 0x2000;
                        let offset = address as usize - 0xA000;
                        self.ram[base + offset] = value;
                    }
                }
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
            return match address {
                0x0..=0x3FFF => self.rom[address],
                0x4000..=0x7FFF => {
                    let base = self.selected_rom_bank.max(1);
                    let offset = address - 0x4000;
                    self.rom[base + offset]
                }
                0xA000..=0xA1FF => {
                    if self.ram_enabled {
                        return self.ram[address - 0xA000];
                    }
                    0
                }
                _ => 0,
            };
        }

        pub fn write(&mut self, address: usize, value: u8) {
            match address {
                0x0..=0x1FFF => {
                    if address & 0x100 == 0 {
                        self.ram_enabled = (value & 0x0f) == 0x0a;
                    }
                }
                0x2000..=0x3FFF => {
                    if address & 0x100 != 0 {
                        self.selected_rom_bank = (value as usize & 0xf).max(1);
                    }
                }
                0xA000..=0xA1FF => {
                    if self.ram_enabled {
                        self.ram[address - 0xA000] = value & 0xF
                    }
                }
                _ => {}
            }
        }
    }

    impl Mbc3 {
        pub fn new(rom: Vec<u8>) -> Self {
            let mut mbc3 = Self {
                rom,
                ram: vec![0; 0x8000],
                rom_bank: 0,
                enable: false,
                select: 0,
                rtc_secs: 0,
                rtc_mins: 0,
                rtc_hours: 0,
                rtc_day_low: 0,
                rtc_day_high: 0,
                epoch: 0,
                prelatch: false,
            };
            mbc3.update_epoch();
            mbc3
        }

        pub fn update_epoch(&mut self) {
            //TODO
            self.epoch = 0;
        }

        pub fn read(&self, address: usize) -> u8 {
            return match address {
                0x0..=0x3FFF => self.rom[address],
                0x4000..=0x7FFF => {
                    let rom_bank = self.rom_bank.max(1);
                    let base = rom_bank * 0x4000;
                    let offset = address - 0x4000;
                    self.rom[base + offset]
                }
                0xA000..=0xBFFF => match self.select {
                    x if x == 0x00 || x == 0x01 || x == 0x02 || x == 0x03 => {
                        let base = x as usize * 0x2000;
                        let offset = address as usize - 0xa000;
                        self.ram[base + offset]
                    }
                    0x08 => self.rtc_secs,
                    0x09 => self.rtc_mins,
                    0x0a => self.rtc_hours,
                    0x0b => self.rtc_day_low,
                    0x0c => self.rtc_day_high,
                    _ => 0,
                },
                _ => 0,
            };
        }

        pub fn write(&mut self, address: usize, value: u8) {
            match address {
                0..=0x1FFF => {
                    self.enable = value != 0x00;
                }
                0x2000..=0x3FFF => {
                    self.rom_bank = value as usize & 0x7f;
                }
                0x4000..=0x5FFF => {
                    self.select = value;
                }
                0x6000..=0x7FFF => {
                    //TODO
                }
                0xA000..=0xBFFF => match self.select {
                    x if x == 0x00 || x == 0x01 || x == 0x02 || x == 0x03 => {
                        let base = x as usize * 0x2000;
                        let offset = address as usize - 0xa000;
                        self.ram[base + offset] = value;
                    }
                    0x08 => {
                        self.rtc_secs = value;
                    }
                    0x09 => {
                        self.rtc_mins = value;
                    }
                    0x0a => {
                        self.rtc_hours = value;
                    }
                    0x0b => {
                        self.rtc_day_low = value;
                    }
                    0x0c => {
                        self.rtc_day_high = value;
                    }
                    s => unimplemented!("Unknown selector: {:02x}", s),
                },
                _ => {}
            }
        }
    }

    impl Mbc5 {
        pub fn new(rom: Vec<u8>) -> Self {
            Mbc5 {
                rom,
                ram: vec![0; 0x20000],
                selected_rom_bank: 0,
                selected_ram_bank: 0,
                ram_enabled: false,
            }
        }

        pub fn read(&self, address: usize) -> u8 {
            return match address {
                0..=0x3FFF => self.rom[address],
                0x4000..=0x7FFF => {
                    let base = self.selected_rom_bank * 0x4000;
                    let offset = address - 0x4000;
                    self.rom[base + offset]
                }
                0xA000..=0xBFFF => {
                    if self.ram_enabled {
                        let base = self.selected_ram_bank * 0x2000;
                        let offset = address - 0xA000;
                        self.ram[base + offset];
                    }
                    0
                }
                _ => 0,
            };
        }

        pub fn write(&mut self, address: usize, value: u8) {
            match address {
                0..=0x1FFF => {
                    self.ram_enabled = value & 0xF == 0xA;
                }
                0x2000..=0x2FFF => {
                    self.selected_rom_bank = (self.selected_rom_bank & !0xff) | value as usize;
                }
                0x3000..=0x3FFF => {
                    self.selected_rom_bank =
                        (self.selected_rom_bank & !0x100) | (value as usize & 1) << 8;
                }
                0x4000..=0x5FFF => {
                    self.selected_ram_bank = value as usize & 0xf;
                }
                0xA000..=0xBFFF => {
                    if self.ram_enabled {
                        let base = self.selected_ram_bank * 0x2000;
                        let offset = address - 0xa000;
                        self.ram[base + offset] = value;
                    }
                }
                _ => {}
            }
        }
    }

    impl MbcType {
        pub fn write(&mut self, address: usize, value: u8) {
            match self {
                MbcType::None(mbc) => mbc.write(address, value),
                MbcType::Mbc1(mbc) => mbc.write(address, value),
                MbcType::Mbc2(mbc) => mbc.write(address, value),
                MbcType::Mbc3(mbc) => mbc.write(address, value),
                MbcType::Mbc5(mbc) => mbc.write(address, value),
            }
        }

        pub fn read(&self, address: usize) -> u8 {
            match self {
                MbcType::None(mbc) => mbc.read(address),
                MbcType::Mbc1(mbc) => mbc.read(address),
                MbcType::Mbc2(mbc) => mbc.read(address),
                MbcType::Mbc3(mbc) => mbc.read(address),
                MbcType::Mbc5(mbc) => mbc.read(address),
            }
        }

        pub fn new(mbc_code: u8, rom: Vec<u8>) -> Self {
            match mbc_code {
                0x00 => MbcType::None(MbcNone::new(rom)),
                0x01..=0x03 => MbcType::Mbc1(Mbc1::new(rom)),
                0x05..=0x06 => MbcType::Mbc2(Mbc2::new(rom)),
                0x0F..=0x13 => MbcType::Mbc3(Mbc3::new(rom)),
                0x19..=0x1E => MbcType::Mbc5(Mbc5::new(rom)),
                _ => panic!("unhandled {} mbc code (not implemented)", mbc_code),
            }
        }
    }
}
