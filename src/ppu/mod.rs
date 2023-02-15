pub mod ppu {
    use std::borrow::BorrowMut;
    use std::fmt::{Debug, Display, Formatter};
    use image::{Rgba, RgbaImage};

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

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) enum PPU_mode {
        HBlank = 0,
        VBlank = 1,
        OAM = 2,
        VRAM = 3
    }

    pub(crate) enum LCDCFlags {
        LCD_enabled = 7,
        Window_tile_map_area = 6,  //determines which background map to use 0=9800-9BFF, 1=9C00-9FFF
        Window_enable = 5,  //should display window
        BG_tile_set_area = 4,   //0=8800-97FF, 1=8000-8FFF
        BG_tile_map_area = 3,   //determines which tile map to use 0=9800-9BFF, 1=9C00-9FFF
        Obj_size = 2,   //sprite size 0=8x8, 1=8x16
        Obj_enable = 1,    //should sprites be displayed?
        Bg_enable = 0   //hide backgorund and window
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

    pub fn dump_tile_map(video_ram: [u8; 0x2000], base_offset: usize) -> String {
        let mut dump = "".to_owned();
        for tile_number in 0..(32*32) {
            if tile_number % 32 == 0 && tile_number != 0 {
                dump.push_str(" \n")
            }
            dump.push_str(&format!("{:03} - ", video_ram[base_offset + tile_number as usize]));
        }
        dump
    }

    //given a tile and a mutable RgbaImage reference, draw the tile into the image using default color mapping and given x,y offsets
    pub(crate) fn add_tile_to_rgba_image(Tile: Tile, rgba_image: &mut RgbaImage, (x_offset, y_offset) : (u32, u32)) {
        for (y, tile_row) in Tile.iter().enumerate() {
            for (x, tile_pixel) in tile_row.iter().enumerate() {
                let color_at_coordinate = COLORS[*tile_pixel as usize];
                rgba_image.put_pixel(x as u32 + x_offset, y as u32 + y_offset, Rgba(color_at_coordinate));
            }
        }
    }

    //dump a tile set into an RgbaImage
    pub(crate) fn tile_set_to_rgba_image(tile_set: [Tile; PPU_TILES_NUMBER]) -> RgbaImage {
        let (columns_number, rows_number): (u32, u32) =  (20, 20);
        let mut rgba_image = RgbaImage::new(rows_number * 8, columns_number * 8);

        for x_offset in 0..columns_number {
            for y_offset in 0..rows_number {
                let current_tile_index = ((y_offset*20) + x_offset) as usize;
                if current_tile_index < PPU_TILES_NUMBER {
                    let current_tile = tile_set[current_tile_index];
                    add_tile_to_rgba_image(current_tile, rgba_image.borrow_mut(), (x_offset * 8, y_offset * 8));
                }
            }
        }
        rgba_image
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

    pub(crate) const COLORS: [[u8; 4]; 4] = [ //cool red palette, colors should be based on palette map
        [124, 63, 88, 255], //#7c3f58
        [235, 107, 111, 255], //#eb6b6f
        [249, 168, 117, 255], //#f9a875
        [255, 246, 211, 255], //#fff6d3
    ];

    pub struct PPU {
        clock: u32,
        current_line: u32,
        pub(crate) mode: PPU_mode,
        pub(crate) lcd_control: u8,
        scroll_y: u8,
        scroll_x: u8,
        background_palette_data: u8,
        pub(crate) image_buffer: RgbaImage,
        pub(crate) video_ram: [u8; 0x2000],
        pub(crate) tile_set: [Tile; PPU_TILES_NUMBER],
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
               image_buffer: RgbaImage::new(SCREEN_HORIZONTAL_RESOLUTION, SCREEN_VERTICAL_RESOLUTION),
           }
        }

        pub(crate) fn step(&mut self, clock: u32) -> PPU_mode {
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
                        self.render_scanline();
                        self.mode = PPU_mode::HBlank;
                    }
                },
                _ => {}
            }
            return self.mode.clone()
        }

        pub(crate) fn render_scanline(&mut self) {
            //the tile map contains the index of the tile to be displayed
            let background_tile_map_starting_address: usize = if self.get_lcdc_value(LCDCFlags::BG_tile_map_area) { 0x1C00 } else { 0x1800 };

            //tile offset to render tiles sub-portions
            let mut x_tile_offset = self.scroll_x & 8;
            let y_tile_offset = (self.current_line + self.scroll_y as u32) & 7;


            for pixel in 0..SCREEN_HORIZONTAL_RESOLUTION {
                //viewport offsets used to retrieve right tile id from tile_map in vram
                // each row (y) is 32 tiles (from the total 256x256 viewport), each tile is 8 pixel (hence 8*32)
                let y_offset = ((self.current_line + self.scroll_y as u32) / 8 * 32) as usize;
                let x_offset = ((pixel as u8 + self.scroll_x) / 8) as usize;

                let mut tile_id = self.video_ram[(background_tile_map_starting_address + y_offset + x_offset) as usize] as u16;
                let mut tile = None;

                if !self.get_lcdc_value(LCDCFlags::BG_tile_set_area) {
                    let fixed_tile_id = 256_u16.wrapping_add((tile_id as i8) as u16);
                    tile = Some(self.tile_set[fixed_tile_id as usize]);
                }else {
                    tile = Some(self.tile_set[tile_id as usize]);
                }

                let color_number_at_coordinates: TilePixelValue = tile.unwrap()[y_tile_offset as usize][x_tile_offset as usize];

                let color_at_coordinate = self.get_color_from_bg_palette(color_number_at_coordinates);

                x_tile_offset = (x_tile_offset + 1) % 8;

                self.image_buffer.put_pixel(pixel, self.current_line, Rgba(color_at_coordinate));
            }
        }

        //given a TilePixelValue returns corresponding palette color, using palette map (stored at 0xFF47)
        pub(crate) fn get_color_from_bg_palette(&mut self, color_number: TilePixelValue) -> [u8; 4] {
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
            for tile_column in 0..8 {

                //for each bit we look into vram to get it's value
                //and mix it with the next byte data (address + 1)
                //keeping in mind that the next byte data will be the MSB

                let current_column_mask_position = 1 << (7 - tile_column);

                let bit_value_for_position = if (self.video_ram[address] & current_column_mask_position) >= 1  { 1 } else { 0 };
                let bit_value_for_next_position = if (self.video_ram[address + 1] & current_column_mask_position) >= 1 { 2 } else { 0 };

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

        pub(crate) fn print_lcdc_status(&self) {
            println!("LCD control (0xFF40) - 0x{:02x} \
            LCD enabled - {}, \
            Window tile map area - {}, \
            Window enabled - {}, \
            BG tile set area - {}, \
            BG tile map area - {}, \
            Obj size - {}, \
            Obj enabled - {}, \
            Bg enabled - {}",
                     self.lcd_control,
                     self.get_lcdc_value(LCDCFlags::LCD_enabled),
                     if self.get_lcdc_value(LCDCFlags::Window_tile_map_area) { "0x9C00" } else { "0x9800" },
                     self.get_lcdc_value(LCDCFlags::Window_enable),
                     if self.get_lcdc_value(LCDCFlags::BG_tile_set_area) { "0x8000" } else { "0x8800" },
                     if self.get_lcdc_value(LCDCFlags::BG_tile_map_area) { "0x9C00" } else { "0x9800" },
                     if self.get_lcdc_value(LCDCFlags::Obj_size) { "8x16" } else { "8x8" },
                     self.get_lcdc_value(LCDCFlags::Obj_enable),
                     self.get_lcdc_value(LCDCFlags::Bg_enable),
            )
        }
    }

}