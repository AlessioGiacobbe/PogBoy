pub mod ppu {
    use std::borrow::BorrowMut;
    use std::fmt::{Debug, Display, format, Formatter};
    use image::{Rgba, RgbaImage};
    use piston_window::math::add;

    const PPU_TILES_NUMBER: usize = 384;
    const PPU_SPRITES_NUMBER: usize = 40;
    const TOTAL_SCANLINES: u32 = 153;
    const VISIBLE_SCANLINES: u8 = 144;
    const HBLANK_DURATION_DOTS: u32 = 204;
    const VBLANK_DURATION_DOTS: u32 = 456;
    const OAM_DURATION_DOTS: u32 = 80;
    const VRAM_DURATION_DOTS: u32 = 172;

    const SCREEN_HORIZONTAL_RESOLUTION: u32 = 160;
    const SCREEN_VERTICAL_RESOLUTION: u32 = 144;

    const TILE_SIZE: u32 = 8;
    const TILES_IN_VISIBLE_LINE: u32 = SCREEN_HORIZONTAL_RESOLUTION / TILE_SIZE;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub(crate) struct Sprite {
        x: i32,
        y: i32,
        tile_number: u8,
        background_priority: bool,
        y_flip: bool,
        x_flip: bool,
        palette: bool
    }

    //each tile is 8x8 pixels
    pub(crate) type Tile = [[TilePixelValue; TILE_SIZE as usize]; TILE_SIZE as usize];

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
    pub(crate) enum PpuMode {
        HBlank = 0,
        VBlank = 1,
        OAM = 2,
        VRAM = 3
    }

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) enum StatInterruptType {
        HBlank = 3,
        VBlank = 4,
        OAM = 5,
        LYC_EQUALS_LY = 6
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

    #[derive(Copy, Clone)]
    pub struct TileRow([(u16, u16); (SCREEN_HORIZONTAL_RESOLUTION/TILE_SIZE) as usize]);

    impl Debug for TileRow {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for (tile_id, address) in self.0.iter() {
                write!(f, "[0x{:02X} - 0x{:04X}]", tile_id, address + 0x8000).expect("");
            }
            write!(f, "\n")
        }
    }

    impl From<u8> for PpuMode{
        fn from(value: u8) -> Self {
            match value {
                0 => PpuMode::HBlank,
                1 => PpuMode::VBlank,
                2 => PpuMode::OAM,
                3 => PpuMode::VRAM,
                _ => PpuMode::HBlank
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
        [[TilePixelValue::Zero; TILE_SIZE as usize]; TILE_SIZE as usize]
    }
    
    fn create_empty_sprite() -> Sprite {
        Sprite {
            x: -16,
            y: -8,
            tile_number: 0,
            background_priority: false,
            y_flip: false,
            x_flip: false,
            palette: false
        }
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

    pub fn dump_current_screen_tiles(mut ppu: &mut PPU) -> [TileRow; (SCREEN_VERTICAL_RESOLUTION/TILE_SIZE) as usize] {
        let mut screen_dump = [TileRow([(0,0); (SCREEN_HORIZONTAL_RESOLUTION/TILE_SIZE) as usize]); (SCREEN_VERTICAL_RESOLUTION/TILE_SIZE) as usize];
        for screen_line in 0..SCREEN_VERTICAL_RESOLUTION/TILE_SIZE {
            screen_dump[screen_line as usize] = ppu.render_background(screen_line*TILE_SIZE);
        }
        screen_dump
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

    pub(crate) const COLORS: [[u8; 4]; 4] = [ //cool red palette, colors should be based on palette map
        [124, 63, 88, 255], //#7c3f58           [248, 227, 196, 255],
        [235, 107, 111, 255], //#eb6b6f         [204, 52, 149, 255],
        [249, 168, 117, 255], //#f9a875        [107, 31, 177, 255],
        [255, 246, 211, 255], //#fff6d3        [11, 6, 48, 255],
    ];

    pub struct PPU {
        clock: u32,
        current_line: u32,  //ly
        current_line_compare: u32, //lyc
        pub(crate) lcd_status: u8,
        pub(crate) lcd_control: u8,
        scroll_y: u8,
        scroll_x: u8,
        window_x: u8,
        window_y: u8,
        background_palette_data: u8,
        obj_0_palette_data: u8,
        obj_1_palette_data: u8,
        pub(crate) image_buffer: RgbaImage,
        pub(crate) oam: [u8; 0x2000],
        pub(crate) video_ram: [u8; 0x2000],
        pub(crate) tile_set: [Tile; PPU_TILES_NUMBER],
        pub(crate) sprite_set: [Sprite; PPU_SPRITES_NUMBER]
    }

    impl Debug for PPU {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "PPU - mode : {:?} - clock : {}", 0, self.clock)
        }
    }

    impl PPU {
        pub(crate) fn new() -> PPU {
           PPU {
               video_ram: [0; 0x2000],
               current_line: 0,
               current_line_compare: 0,
               clock: 0,
               lcd_status: 0,
               tile_set: [create_empty_tile(); PPU_TILES_NUMBER],
               lcd_control: 0,
               scroll_y: 0,
               scroll_x: 0,
               window_x: 0,
               window_y: 0,
               background_palette_data: 0,
               obj_0_palette_data: 0,
               obj_1_palette_data: 0,
               oam: [0; 0x2000],
               image_buffer: RgbaImage::new(SCREEN_HORIZONTAL_RESOLUTION, SCREEN_VERTICAL_RESOLUTION),
               sprite_set: [create_empty_sprite(); PPU_SPRITES_NUMBER],
           }
        }

        pub(crate) fn set_lcdc(&mut self, value: u8) {
            self.lcd_control = value;

            if !self.get_lcdc_value(LCDCFlags::LCD_enabled) {
                self.clock = 0;
                //some checks should be performed before hard-changing ppu mode
                self.set_current_mode(PpuMode::HBlank);
                //next mode should be 2
                self.current_line = 0
            }
        }

        pub(crate) fn get_current_mode(&self) -> PpuMode {
            return PpuMode::try_from((self.lcd_status & 0b11)).unwrap()
        }

        pub(crate) fn set_current_mode_from_value(&mut self, mut value: u8) {
            value &= 0b1111000;
            self.lcd_status &= 0b1000_0111;
            self.lcd_status |= value;
        }

        pub(crate) fn set_current_mode(&mut self, ppu_mode: PpuMode) {
            self.lcd_status = (self.lcd_control & 0b00) | ppu_mode as u8;
        }

        fn should_try_to_request_lyc_ly_interrupt(&mut self) -> bool {
            if self.current_line == self.current_line_compare {
                return self.should_rise_lcdc_interrupt(StatInterruptType::LYC_EQUALS_LY);
            }
            return false;
        }

        fn should_rise_lcdc_interrupt(&mut self, stat_interrupt_type: StatInterruptType) -> bool {
            //check if stat interrupt type is enabled in lcd_status
            return (self.lcd_status & (1 << stat_interrupt_type as u8)) != 0
        }

        pub(crate) fn step(&mut self, clock: u32) -> (PpuMode, bool, bool) {
            self.clock += clock;
            let (mut should_rise_vblank_interrupt, mut should_rise_stat_interrupt) = (false, false);

            match self.get_current_mode() {
                //horizontal blanking
                PpuMode::HBlank => {
                    if self.clock >= HBLANK_DURATION_DOTS {
                        self.clock = 0;
                        self.current_line += 1;

                        if self.current_line == (VISIBLE_SCANLINES - 1) as u32 {
                            //last line, go to v blank
                            should_rise_vblank_interrupt = true;
                            self.set_current_mode(PpuMode::VBlank);
                            should_rise_stat_interrupt = self.should_rise_lcdc_interrupt(StatInterruptType::VBlank)
                        } else {
                            //scan another line
                            self.set_current_mode(PpuMode::OAM);
                        }

                        should_rise_stat_interrupt = self.should_rise_lcdc_interrupt(StatInterruptType::OAM);
                        should_rise_stat_interrupt = self.should_try_to_request_lyc_ly_interrupt();
                    }
                },
                //vertical blanking
                PpuMode::VBlank => {
                    if self.clock >= VBLANK_DURATION_DOTS {
                        self.clock = 0;
                        self.current_line += 1;

                        if self.current_line > TOTAL_SCANLINES {
                            self.set_current_mode(PpuMode::OAM);
                            should_rise_stat_interrupt = self.should_rise_lcdc_interrupt(StatInterruptType::OAM);
                            self.current_line = 0;
                        }
                    }
                    should_rise_stat_interrupt = self.should_try_to_request_lyc_ly_interrupt();
                },
                // OAM read
                PpuMode::OAM => {
                    if self.clock >= OAM_DURATION_DOTS {
                        self.clock = 0;
                        self.set_current_mode(PpuMode::VRAM);
                    }
                },
                // VRAM read and complete line scan
                PpuMode::VRAM => {
                    if self.clock >= VRAM_DURATION_DOTS {
                        self.clock = 0;
                        self.render_current_line();
                        self.set_current_mode(PpuMode::HBlank);
                        should_rise_stat_interrupt = self.should_rise_lcdc_interrupt(StatInterruptType::HBlank)
                    }
                },
                _ => {}
            }
            return (self.get_current_mode(), should_rise_vblank_interrupt, should_rise_stat_interrupt);
        }

        pub(crate) fn render_current_line(&mut self) {
            if self.get_lcdc_value(LCDCFlags::Bg_enable) {
                self.render_background(self.current_line);
            }

            if self.get_lcdc_value(LCDCFlags::Obj_enable) {
                self.render_sprites(self.current_line);
            }
        }

        pub(crate) fn render_background(&mut self, line: u32) -> TileRow {
            //the tile map contains the index of the tile to be displayed
            let background_tile_map_starting_address: usize = if self.get_lcdc_value(LCDCFlags::BG_tile_map_area) { 0x1C00 } else { 0x1800 };

            //tile offset to render tiles sub-portions
            let mut x_tile_offset = self.scroll_x & 7;
            let y_tile_offset = (line + self.scroll_y as u32) & 7;

            let mut used_tiles: TileRow = TileRow([(0,0); TILES_IN_VISIBLE_LINE as usize]);
            for pixel in 0..SCREEN_HORIZONTAL_RESOLUTION {
                //viewport offsets used to retrieve right tile id from tile_map in vram
                // each row (y) is 32 tiles (from the total 256x256 viewport), each tile is 8 pixel (hence 8*32)
                let y_offset = (((line + self.scroll_y as u32) / 8 % 32) * 32) as usize;
                let x_offset = ((pixel as u8 + self.scroll_x) / 8) as usize;

                let mut tile_id = self.video_ram[(background_tile_map_starting_address + y_offset + x_offset) as usize] as u16;
                used_tiles.0[(pixel / 8) as usize] = (tile_id, (background_tile_map_starting_address + y_offset + x_offset) as u16);
                let mut tile = None;

                if !self.get_lcdc_value(LCDCFlags::BG_tile_set_area) {
                    let fixed_tile_id = 256_u16.wrapping_add((tile_id as i8) as u16);
                    used_tiles.0[(pixel / 8) as usize] = (fixed_tile_id, (background_tile_map_starting_address + y_offset + x_offset) as u16);
                    tile = Some(self.tile_set[fixed_tile_id as usize]);
                }else {
                    tile = Some(self.tile_set[tile_id as usize]);
                }

                let color_number_at_coordinates: TilePixelValue = tile.unwrap()[y_tile_offset as usize][x_tile_offset as usize];

                let color_at_coordinate = self.get_color_from_palette(color_number_at_coordinates, self.background_palette_data);

                x_tile_offset = (x_tile_offset + 1) % 8;

                self.image_buffer.put_pixel(pixel, line, Rgba(color_at_coordinate));
            }
            used_tiles
        }

        pub(crate) fn render_sprites(&mut self, line: u32) {
            for sprite_index in 0..PPU_SPRITES_NUMBER-1 {
                let sprite = self.sprite_set[sprite_index];
                let sprite_height = if self.get_lcdc_value(LCDCFlags::Obj_size) { 16 } else { 8 };

                //check if sprite is in current line
                //todo 8 should be sprite height
                if sprite.y as i32 <= line as i32 && sprite.y as i32 + 8 > line as i32 {
                    let palette = if sprite.palette { self.obj_1_palette_data } else { self.obj_0_palette_data };

                    //current tile row
                    let current_tile = self.tile_set[sprite.tile_number as usize];
                    let tile_row = current_tile[(line - sprite.y as u32) as usize];
                    //todo check sprite y flip

                    for x in 0..8 {
                        //todo priority and background check
                        //if sprite is visible and sprite is within viewport and pixel is not transparent
                        if (sprite.x + x) > 0 && (sprite.x as i32 + 8 as i32) < 160 && tile_row[x as usize] != TilePixelValue::Zero {
                            //todo check x-flip
                            let color_number_at_coordinates: TilePixelValue = tile_row[x as usize];
                            let color_at_coordinate = self.get_color_from_palette(color_number_at_coordinates, palette);
                            self.image_buffer.put_pixel((sprite.x + x) as u32, line, Rgba(color_at_coordinate));
                        }
                    }
                }
            }
        }

        //given a TilePixelValue returns corresponding palette color, using palette map (stored at 0xFF47)
        pub(crate) fn get_color_from_palette(&mut self, color_number: TilePixelValue, palette: u8) -> [u8; 4] {
            let color_number = color_number as u8;
            if color_number < 4 {
                // get bits couples by moving right by number * 2 and mask with 3 (b11) to get the value
                let value_for_color = (palette >> (color_number * 2)) & 0x3;
                return COLORS[value_for_color as usize];
            }
            COLORS[0]
        }

        pub(crate) fn update_sprite(&mut self, address: usize, value: u8){
            let sprite_index = address >> 2; // /4

            if sprite_index < 40 {
                match address & 3 {
                    0 => self.sprite_set[sprite_index].y = value as i32 - 16,
                    1 => self.sprite_set[sprite_index].x = value as i32 - 8,
                    2 => self.sprite_set[sprite_index].tile_number = value,
                    3 => {
                        self.sprite_set[sprite_index].palette = (value & 0x10) == 1;
                        self.sprite_set[sprite_index].x_flip = (value & 0x20) == 1;
                        self.sprite_set[sprite_index].y_flip = (value & 0x40) == 1;
                        self.sprite_set[sprite_index].background_priority = (value & 0x80) == 1;
                    },
                    _ => {}
                }
            }
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
                0xFE00..=0xFEFF => {
                    if address < 0xFEA0 {
                        return self.oam[address - 0xFE00]
                    }
                    0
                },
                0xFF40 => {
                    self.lcd_control
                },
                0xFF41 => {
                    if self.current_line == self.current_line_compare {
                        return self.lcd_status | 1 << 2
                    }
                    self.lcd_status
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
                0xFF45 => {
                    self.current_line_compare as u8
                },
                0xFF47 => {
                    self.background_palette_data
                },
                0xFF48 => {
                    self.obj_0_palette_data
                },
                0xFF49 => {
                    self.obj_1_palette_data
                },
                0xFF4A => {
                    self.window_y
                },
                0xFF4B => {
                    self.window_x
                },
                _ => 0
            }
        }

        pub(crate) fn write_byte(&mut self, address: usize, value: u8) {
            match address {
                0x8000..=0x9FFF => {
                    //TODO implement bank switching, this could write to bank vram 0 or 1
                    self.video_ram[address - 0x8000] = value;
                    if address < 0x9800 {
                        self.update_tile(address);
                    }
                },
                0xFE00..=0xFEFF => {
                    if address < 0xFEA0 {
                        self.oam[address - 0xFE00] = value;
                        self.update_sprite(address - 0xFE00, value);
                    }
                },
                0xFF40 => {
                    self.lcd_control = value;
                },
                0xFF41 => {
                    self.lcd_status = value;
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
                0xFF45 => {
                    self.current_line_compare = value as u32
                },
                0xFF47 => {
                    //TODO should clear specific cache
                    self.background_palette_data = value;
                },
                0xFF48 => {
                    //TODO should clear specific cache
                    self.obj_0_palette_data = value;
                },
                0xFF49 => {
                    //TODO should clear specific cache
                    self.obj_1_palette_data = value;
                },
                0xFF4A => {
                    //TODO check window x/y usage in PPU
                    self.window_y = value;
                },
                0xFF4B => {
                    self.window_y = value;
                }
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