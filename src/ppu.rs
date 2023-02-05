pub mod ppu {
    use std::fmt::{Debug, Display, Formatter};
    use image::{ImageBuffer, Rgba};
    use crate::cpu::CPU::CPU;

    //each tile is 8x8 pixels
    pub(crate) type Tile = [[TilePixelValue; 8]; 8];

    //each value should refer to a specific color depending on the current mapping
    //(it can be shifted to do cool stuff)
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub(crate) enum TilePixelValue {
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

    impl From<u8> for TilePixelValue {
        fn from(value: u8) -> Self {
            match value {
                0 => TilePixelValue::Zero,
                1 => TilePixelValue::One,
                2 => TilePixelValue::Two,
                3 => TilePixelValue::Three,
                _ => TilePixelValue::Zero
            }
        }
    }

    pub(crate) enum LCDCFlags {
        LCD_enabled = 7,
        Window_tile_map_area = 6,  //determines which background map to use 0=9800-9BFF, 1=9C00-9FFF
        Window_enable = 5,  //should display window
        BG_tile_data_area = 4,   //0=8800-97FF, 1=8000-8FFF
        BG_tile_map_area = 3,   //determines which tile map to use 0=9800-9BFF, 1=9C00-9FFF
        Obj_size = 2,   //sprite size 0=8x8, 1=8x16
        Obj_enable = 1,    //should sprites be displayed?
        Bg_enable = 0   //hide backgorund and window
    }


    impl Display for TilePixelValue {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let tile_pixel_corresponding_emoji = match self {
                TilePixelValue::Zero => "⚫",
                TilePixelValue::One => "⭕",
                TilePixelValue::Two => "🟤",
                TilePixelValue::Three => "⚪"
            };
            write!(f, "{}", tile_pixel_corresponding_emoji)
        }
    }

    fn create_empty_tile() -> Tile {
        [[TilePixelValue::Zero; 8]; 8]
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

    const SCREEN_HORIZONTAL_RESOLUTION: u32 = 160;
    const SCREEN_VERTICAL_RESOLUTION: u32 = 144;

    pub(crate) const COLORS: [[u8; 3]; 4] = [ //cool red palette, colors should be based on palette map
        [124, 63, 88], //#7c3f58
        [235, 107, 111], //#eb6b6f
        [249, 168, 117], //#f9a875
        [255, 246, 211], //#fff6d3
    ];



    pub struct PPU {
        clock: u32,
        current_line: u32,
        pub(crate) mode: PPU_mode,
        lcd_control: u8,
        scroll_y: u8,
        scroll_x: u8,
        background_palette_data: u8,
        pub(crate) image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
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
               lcd_control: 0,
               scroll_y: 0,
               scroll_x: 0,
               background_palette_data: 0,
               image_buffer: image::ImageBuffer::new(SCREEN_HORIZONTAL_RESOLUTION, SCREEN_VERTICAL_RESOLUTION)
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

                        if self.current_line == (VISIBLE_SCANLINES - 1) as u32 {
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

        //we render a single line scan (horizontally)
        pub(crate) fn render_scan(&mut self) {
            //the tile map contains the index of the tile to be displayed
            let background_tile_map_starting_address = if self.get_lcdc_value(LCDCFlags::BG_tile_map_area) { 0x9C00 - 0x8000 } else { 0x9800 - 0x8000 };

            //current line and y scroll are represented in pixel, each tile is 8x8, we divide by 8 to get the tile number
            let map_starting_point = ((self.current_line + self.scroll_y as u32) & 255) / 8;

            //line offset is basically x offset divided by 8 because each tile is 8x8
            let line_offset = (self.scroll_x / 8) as u32;

            //retrieve tile id from tile map
            let mut tile_id = self.video_ram[(background_tile_map_starting_address + map_starting_point + line_offset) as usize] as u16;
            //if the first tile data set is used, we need to calculate the right id
            if self.get_lcdc_value(LCDCFlags::BG_tile_data_area) && tile_id < 128 { tile_id += 256 }

            //x and y indicates starting coordinates inside a tile
            let mut x = self.scroll_x & 7;
            let y = (self.current_line + self.scroll_y as u32) & 7;

            let mut buffer_offset = self.current_line * SCREEN_HORIZONTAL_RESOLUTION;


            for _ in 0..SCREEN_HORIZONTAL_RESOLUTION - 1 {
                let tile: Tile = self.tile_set[tile_id as usize];
                let color_at_coordinates: TilePixelValue = tile[y as usize][x as usize];

                //todo get x, y buffer coordinates from buffer_offset (right now buffer_offset is linear)
                //todo add color to buffer
                //self.image_buffer.put_pixel(10, 10, Rgba([255,255,255,255]));

                buffer_offset += 1;
                x += 1;

                //if tile is ended we need to get the next tile
                if x == 8 {
                    x = 0;
                    buffer_offset = (buffer_offset + 1) & 31;
                    tile_id = self.video_ram[(background_tile_map_starting_address + map_starting_point + line_offset) as usize] as u16;
                    if self.get_lcdc_value(LCDCFlags::BG_tile_data_area) && tile_id < 128 { tile_id += 256 }
                }
            }
        }

        //given a TilePixelValue returns corresponding palette color, using palette map (stored at 0xFF47)
        pub(crate) fn get_color_from_bg_palette(&mut self, mut color_number: TilePixelValue) -> [u8; 3] {
            let color_number = color_number as u8;
            if color_number < 4 {
                // get bits couples by moving right by number * 2 and mask with 3 (b11) to get the value
                let value_for_color = (self.background_palette_data >> (color_number * 2)) & 0x3;
                return COLORS[value_for_color as usize];
            }
            COLORS[0]
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
                    0 => TilePixelValue::Zero,
                    1 => TilePixelValue::One,
                    2 => TilePixelValue::Two,
                    3 => TilePixelValue::Three,
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
                0xFF42 => {
                    self.scroll_y
                },
                0xFF43 => {
                    self.scroll_x
                },
                0xFF44 => {
                    self.current_line as u8
                },
                0xFF47 => {
                    self.background_palette_data
                }
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
                0xFF42 => {
                    self.scroll_y = value;  
                },
                0xFF43 => {
                    self.scroll_x = value;
                },
                0xFF44 => {
                    self.current_line = value as u32
                },
                0xFF47 => {
                    self.background_palette_data = value;
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