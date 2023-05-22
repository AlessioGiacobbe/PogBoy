pub mod mbc {
    use std::fmt::{Display, Formatter};

    #[derive(Clone, Debug)]
    pub struct MbcNone {
        rom: Vec<u8>,
        ram: Vec<u8>
    }

    #[derive(Clone, Debug)]
    pub struct Mbc1 {

    }

    #[derive(Clone, Debug)]
    pub struct Mbc2 {

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
        pub fn new() -> Self {
            Mbc1 {}
        }

        pub fn read(&self, address: usize) -> u8 {
            1
        }

        pub fn write(&mut self, address: usize, value: u8) {

        }
    }


    impl Mbc2 {
        pub fn new() -> Self {
            Mbc2 {}
        }

        pub fn read(&self, address: usize) -> u8 {
            1
        }

        pub fn write(&mut self, address: usize, value: u8) {

        }
    }


    impl Mbc3 {
        pub fn new() -> Self {
            Mbc3 {}
        }

        pub fn read(&self, address: usize) -> u8 {
            1
        }

        pub fn write(&mut self, address: usize, value: u8) {

        }
    }

    impl Mbc5 {
        pub fn new() -> Self {
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
                MbcType::Mbc3(mbc) => mbc.write(address, value)
            }
        }

        pub fn read(&self, address: usize) -> u8 {
            match self {
                MbcType::None(mbc) => mbc.read(address),
                MbcType::Mbc1(mbc) => mbc.read(address),
                MbcType::Mbc2(mbc) => mbc.read(address),
                MbcType::Mbc3(mbc) => mbc.read(address)
            }
        }

        pub fn new(mbc_code: u8, rom: Vec<u8>) -> Self {
            match mbc_code {
                0x00 => MbcType::None(MbcNone::new(rom)),
                0x01..=0x03 => MbcType::Mbc1(Mbc1::new()),
                0x05..=0x06 => MbcType::Mbc2(Mbc2::new()),
                0x0F..=0x13 => MbcType::Mbc3(Mbc3::new()),
                0x19..=0x1E => MbcType::Mbc5(Mbc5::new()),
                _ => panic!("unhandled {} mbc code (not implemented)", mbc_code)
            }
        }
    }
}