pub mod cartridge {
    use std::fmt;
    use std::fmt::Formatter;
    use std::fs::File;
    use std::io::Read;
    use serde::{Serialize, Deserialize};

    #[derive(Clone, Debug)]
    pub struct Cartridge {
        pub cartridge_info: Option<CartridgeInfo>,
        pub rom: Vec<u8>
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CartridgeInfo {
        entry_point: [u8; 4],
        nintendo_logo: [u16; 24],
        title: [u8; 15],
        //unused
        cgb_flag: u8,
        //licensee code ASCII, considered if old_license_code is 0x33
        new_licensee_code: [u8; 2],
        //supports sgb?
        sgb_flag: u8,
        //defines hardware on cartridge
        cartridge_type: u8,
        //rome size, given by 32 KiB × (1 << <value>)
        rom_size: u8,
        ram_size: u8,
        //japan or not :)
        destination_code: u8,
        old_licensee_code: u8,
        //rom version number, usually 0
        mask_rom_version: u8,
        //value to check header checksum against
        header_checksum: u8,
        //used only for pokemon
        global_checksum: u16,
    }

    impl fmt::Display for CartridgeInfo {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Cartridge : {{
            entry_point: 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X}
            cgb: {:?}
            game_title: {:?}
            sgb_flag: {}
            cartridge_type: {}
            rom_size: {}
            ram_size: {}
            dest_code: {}
            header_checksum: 0x{:04X}
            global_checksum: 0x{:04X}
        }}",
                   self.entry_point[0], self.entry_point[1], self.entry_point[2], self.entry_point[3],
                   self.cgb_flag,
                   self.game_title(),
                   self.sgb_flag,
                   self.cartridge_type,
                   self.rom_size,
                   self.ram_size,
                   if self.destination_code == 0 {"Jap"} else {"Non-Jap"},
                   self.header_checksum,
                   self.global_checksum)
        }
    }

    impl CartridgeInfo {
        fn game_title(&self) -> &str {
            match std::str::from_utf8(&self.title) {
                Ok(value) => {
                    value
                }
                Err(error) => {
                    println!("Can't read rom name from header! :( {}", error);
                    "BAD HEADER"
                }
            }
        }
    }

    impl Cartridge {
        pub fn set_item(&self, value: u8, address: usize){
            //TODO
        }

        pub fn get_item(&self, address: usize) -> u8{
            self.rom[address]
        }
    }

    const HEX_HEADER_START_ADDRESS: usize = 0x100;
    const HEX_HEADER_END_ADDRESS: usize = 0x14F;

    pub fn read_cartridge(file_name: &str) -> Cartridge {
        let mut rom = File::open(format!("./src/roms/{}", file_name)).expect("rom not found");

        let mut rom_buffer = Vec::new();
        rom.read_to_end(&mut rom_buffer).expect("Can't read ROM");

        let rom_header: &[u8] = &rom_buffer[HEX_HEADER_START_ADDRESS..HEX_HEADER_END_ADDRESS+1];

        let cartridge_info: CartridgeInfo = bincode::deserialize(rom_header).unwrap();

        Cartridge {
            cartridge_info: Some(cartridge_info),
            rom: rom_buffer
        }
    }
}