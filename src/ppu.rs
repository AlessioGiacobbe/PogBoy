pub mod ppu {
    use std::fmt::{Debug, Display, Formatter};
    use crate::cpu::CPU::CPU;

    const COLORS: [&'static str; 4] = ["#0f380f", "#306230", "#8bac0f", "#9bbc0f"];

    type Tile = [[TilePixelValues; 8]; 8];

    #[derive(Copy,Clone, Debug)]
    enum TilePixelValues {
        Zero = 0,
        One = 1,
        Two = 2,
        Three = 3
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

    pub struct PPU {
        framebuffer: [u8; 160 * 144 * 3],
        alpha_framebuffer: [u8; 160 * 144 * 4],
        clock: u32,
        current_line: u32,
        mode: u8,
        pub(crate) video_ram: [u8; 0x2000],
        tile_set: [Tile; 384]
    }

    impl Debug for PPU {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "PPU - mode : {} - clock : {}", self.mode, self.clock)
        }
    }

    impl PPU {

        pub(crate) fn new() -> PPU {
           PPU {
               framebuffer: [0; 69120],
               alpha_framebuffer: [0; 92160],
               video_ram: [0; 0x2000],
               current_line: 0,
               clock: 0,
               mode: 0,
               tile_set: [create_empty_tile(); 384]
           }
        }

        pub(crate) fn step(&mut self, clock: u32) {
            self.clock += clock;

            match self.mode {
                //horizontal blanking
                0 => {
                    if self.clock >= 204 {
                        self.clock = 0;
                        self.current_line += 1;

                        if self.current_line == 143 {
                            //last line, go to v blank
                            self.mode = 1;
                        } else {
                            //scan another line
                            self.mode = 2;
                        }
                    }
                },
                //vertical blanking
                1 => {
                    if self.clock >= 456 {
                        self.clock = 0;
                        self.current_line += 1;

                        if self.current_line > 153 {
                            self.mode = 2;
                            self.current_line = 0;
                        }
                    }
                },
                // OAM read
                2 => {
                    if self.clock >= 80 {
                        self.clock = 0;
                        self.mode = 3;
                    }
                },
                // VRAM read and complete line scan
                3 => {
                    if self.clock >= 172 {
                        self.clock = 0;
                        self.mode = 0;
                        self.render_scan();
                    }
                },
                _ => {}
            }
        }

        pub(crate) fn render_scan(&mut self) {
        }

        pub(crate) fn update_tile(&mut self, address: usize, value: u8) {
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

                let byte_value_for_position = if (self.video_ram[address] & current_column_mask_position) > 1  { 1 } else { 0 };
                let byte_value_for_next_position = if (self.video_ram[address + 1] & current_column_mask_position) > 1 { 2 } else { 0 };

                let tile_value = match byte_value_for_position + byte_value_for_next_position {
                    0 => TilePixelValues::Zero,
                    1 => TilePixelValues::One,
                    2 => TilePixelValues::Two,
                    3 => TilePixelValues::Three,
                    value => panic!("{} Invalid tile value", value)
                };

                self.tile_set[tile_number][tile_row][tile_column] = tile_value;
            }

            print_tile(self.tile_set[0]);
        }

        pub(crate) fn read_byte(&self, address: usize) -> u8 {
            match address {
                0x8000..=0x9FFF => {
                    self.video_ram[address - 0x8000]
                }
                _ => 0
            }
        }

        pub(crate) fn write_byte(&mut self, address: usize, value: u8) {
            match address {
                0x8000..=0x9FFF => {
                    self.video_ram[address - 0x8000] = value;
                    self.update_tile(address, value);
                }
                _ => ()
            }
        }
    }

}