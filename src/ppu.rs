pub mod ppu {
    use std::fmt::{Debug, Display, Formatter};
    use crate::cpu::CPU::CPU;

    //each tile is 8x8 pixels
    pub(crate) type Tile = [[TilePixelValues; 8]; 8];

    //each value should refer to a specific color depending on the current mapping
    //(it can be shifted to do cool stuff)
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub(crate) enum TilePixelValues {
        Zero = 0,
        One = 1,
        Two = 2,
        Three = 3
    }

    #[derive(Debug, Clone)]
    pub(crate) enum PPU_mode {
        HBlank = 0,
        VBlank = 1,
        OAM = 2,
        VRAM = 3
    }

    impl From<u8> for PPU_mode{
        fn from(value: u8) -> Self {
            match value {
                0 => PPU_mode::HBlank,
                1 => PPU_mode::VBlank,
                2 => PPU_mode::OAM,
                3 => PPU_mode::VRAM,
                _ => PPU_mode::HBlank
            }
        }
    }

    pub(crate) enum LCDCFlags {
        LCD_enabled = 7,
        Tile_map_area = 6,  //determines which background map to use 0=9800-9BFF, 1=9C00-9FFF
        Window_enable = 5,  //should display window
        BG_tile_area = 4,   //0=8800-97FF, 1=8000-8FFF
        BG_tile_map_area = 3,   //determines which tile map to use 0=9800-9BFF, 1=9C00-9FFF
        Obj_size = 2,   //sprite size 0=8x8, 1=8x16
        Obj_enable = 1,    //should sprites be displayed?
        Bg_enable = 0   //hide backgorund and window
    }


    impl Display for TilePixelValues {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let tile_pixel_corresponding_emoji = match self {
                TilePixelValues::Zero => "⚫",
                TilePixelValues::One => "⭕",
                TilePixelValues::Two => "🟤",
                TilePixelValues::Three => "⚪"
            };
            write!(f, "{}", tile_pixel_corresponding_emoji)
        }
    }

    fn create_empty_tile() -> Tile {
        [[TilePixelValues::Zero; 8]; 8]
    }

    fn print_tile(Tile: Tile) {
        for tile_row in Tile {

            let tile_line: String = tile_row.iter()
                .fold(String::new(), |mut result_string, tile_pixel| {
                    result_string.push_str(&format!("{}", tile_pixel));
                    result_string
                });

            println!("{}", tile_line)
        }
    }

    const PPU_TILES_NUMBER: usize = 384;
    const TOTAL_SCANLINES: u32 = 153;
    const VISIBLE_SCANLINES: u8 = 144;
    const HBLANK_DURATION_DOTS: u32 = 204;
    const VBLANK_DURATION_DOTS: u32 = 456;
    const OAM_DURATION_DOTS: u32 = 80;
    const VRAM_DURATION_DOTS: u32 = 172;


    pub struct PPU {
        clock: u32,
        current_line: u32,
        pub(crate) mode: PPU_mode,
        lcd_control: u8,
        pub(crate) video_ram: [u8; 0x2000],
        pub(crate) tile_set: [Tile; PPU_TILES_NUMBER]
    }

    impl Debug for PPU {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "PPU - mode : {:?} - clock : {}", self.mode, self.clock)
        }
    }

    impl PPU {

        pub(crate) fn new() -> PPU {
           PPU {
               video_ram: [0; 0x2000],
               current_line: 0,
               clock: 0,
               mode: PPU_mode::HBlank,
               tile_set: [create_empty_tile(); PPU_TILES_NUMBER],
               lcd_control: 0
           }
        }

        pub(crate) fn step(&mut self, clock: u32) {
            self.clock += clock;

            match self.mode {
                //horizontal blanking
                PPU_mode::HBlank => {
                    if self.clock >= HBLANK_DURATION_DOTS {
                        self.clock = 0;
                        self.current_line += 1;

                        if self.current_line == (VISIBLE_SCANLINES - 1) {
                            //last line, go to v blank
                            self.mode = PPU_mode::VBlank;
                        } else {
                            //scan another line
                            self.mode = PPU_mode::OAM;
                        }
                    }
                },
                //vertical blanking
                PPU_mode::VBlank => {
                    if self.clock >= VBLANK_DURATION_DOTS {
                        self.clock = 0;
                        self.current_line += 1;

                        if self.current_line > TOTAL_SCANLINES {
                            self.mode = PPU_mode::OAM;
                            self.current_line = 0;
                        }
                    }
                },
                // OAM read
                PPU_mode::OAM => {
                    if self.clock >= OAM_DURATION_DOTS {
                        self.clock = 0;
                        self.mode = PPU_mode::VRAM;
                    }
                },
                // VRAM read and complete line scan
                PPU_mode::VRAM => {
                    if self.clock >= VRAM_DURATION_DOTS {
                        self.clock = 0;
                        self.mode = PPU_mode::HBlank;
                        self.render_scan();
                    }
                },
                _ => {}
            }
        }

        pub(crate) fn render_scan(&mut self) {
        }

        pub(crate) fn update_tile(&mut self, address: usize) {
            //address is normalized removing LSB
            let address = address & 0x1FFE;

            //each tile occupies 16 bytes, we find tile index dividing the address by 16
            let tile_number = address / 16;

            //to find the selected tile row we subtract the tile base address to the given address
            //and divide by two since each row is composed by two bytes
            let tile_row = (address - (tile_number * 16)) / 2;

            //rewrite each bit in that row
            for tile_column in 0..7 {

                //for each bit we look into vram to get it's value
                //and mix it with the next byte data (address + 1)
                //keeping in mind that the next byte data will be the MSB

                let current_column_mask_position = 1 << (7 - tile_column);

                let bit_value_for_position = if (self.video_ram[address] & current_column_mask_position) > 1  { 1 } else { 0 };
                let bit_value_for_next_position = if (self.video_ram[address + 1] & current_column_mask_position) > 1 { 2 } else { 0 };

                let tile_value = match bit_value_for_position + bit_value_for_next_position {
                    0 => TilePixelValues::Zero,
                    1 => TilePixelValues::One,
                    2 => TilePixelValues::Two,
                    3 => TilePixelValues::Three,
                    value => panic!("{} Invalid tile value", value)
                };

                self.tile_set[tile_number][tile_row][tile_column] = tile_value;
            }

        }

        pub(crate) fn read_byte(&self, address: usize) -> u8 {
            match address {
                0x8000..=0x9FFF => {
                    self.video_ram[address - 0x8000]
                },
                0xFF40 => {
                    self.lcd_control
                },
                0xFF41 => {
                    self.mode.clone() as u8
                },
                0xFF44 => {
                    self.current_line as u8
                },
                _ => 0
            }
        }

        pub(crate) fn write_byte(&mut self, address: usize, value: u8) {
            match address {
                0x8000..=0x9FFF => {
                    self.video_ram[address - 0x8000] = value;
                    if address < 0x9800 {
                        self.update_tile(address);
                    }
                },
                0xFF40 => {
                    self.lcd_control = value;
                },
                0xFF41 => {
                    self.mode = PPU_mode::try_from(value).unwrap()
                },
                0xFF44 => {
                    self.current_line = value as u32
                },
                _ => ()
            }
        }

        pub(crate) fn get_lcdc_value(&self, lcdc_flag: LCDCFlags) -> bool{
            let bit_number = lcdc_flag as u8;
            return ((self.lcd_control >> bit_number) & 0x1) == 1
        }
    }

}