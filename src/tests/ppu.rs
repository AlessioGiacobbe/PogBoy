use crate::ppu::ppu::{Tile, TilePixelValue, COLORS};
use crate::tests::factories::{create_dummy_mmu, create_dummy_ppu, create_dummy_tile};

#[test]
fn tiles_are_generated_correctly() {
    let mut dummy_ppu = create_dummy_ppu();
    let mut dummy_mmu = create_dummy_mmu(&mut dummy_ppu);

    let dummy_tile: Tile = create_dummy_tile();

    //write dummy tile as bytes into tileset position 0
    dummy_mmu.write_byte(0x8000, 0x3D);
    dummy_mmu.write_byte(0x8001, 0x7F);
    dummy_mmu.write_byte(0x8002, 0x42);
    dummy_mmu.write_byte(0x8003, 0x42);
    dummy_mmu.write_byte(0x8004, 0x42);
    dummy_mmu.write_byte(0x8005, 0x42);
    dummy_mmu.write_byte(0x8006, 0x42);
    dummy_mmu.write_byte(0x8007, 0x42);
    dummy_mmu.write_byte(0x8008, 0x7E);
    dummy_mmu.write_byte(0x8009, 0x5E);
    dummy_mmu.write_byte(0x800A, 0x7E);
    dummy_mmu.write_byte(0x800B, 0x0A);
    dummy_mmu.write_byte(0x800C, 0x7C);
    dummy_mmu.write_byte(0x800D, 0x56);
    dummy_mmu.write_byte(0x800E, 0x38);
    dummy_mmu.write_byte(0x800F, 0x7C);

    let tile = dummy_mmu.PPU.tile_set[0];

    for (tile_row, _) in tile.iter().enumerate() {
        for (tile_column, _) in tile[tile_row].iter().enumerate() {
            assert_eq!(
                tile[tile_row][tile_column],
                dummy_tile[tile_row][tile_column]
            )
        }
    }
}

#[test]
fn color_from_bg_palette_is_loaded_correctly() {
    let mut dummy_ppu = create_dummy_ppu();
    let mut dummy_mmu = create_dummy_mmu(&mut dummy_ppu);
    let ppu_colors = COLORS;

    dummy_mmu.write_byte(0xFF47, 0xFF);
    assert_eq!(
        dummy_mmu
            .PPU
            .get_color_from_bg_palette(TilePixelValue::Zero),
        ppu_colors[3]
    );

    dummy_mmu.write_byte(0xFF47, 0x1B); //0b00-01-10-11
    assert_eq!(
        dummy_mmu
            .PPU
            .get_color_from_bg_palette(TilePixelValue::Zero),
        ppu_colors[3]
    );
    assert_eq!(
        dummy_mmu.PPU.get_color_from_bg_palette(TilePixelValue::One),
        ppu_colors[2]
    );
    assert_eq!(
        dummy_mmu.PPU.get_color_from_bg_palette(TilePixelValue::Two),
        ppu_colors[1]
    );
    assert_eq!(
        dummy_mmu
            .PPU
            .get_color_from_bg_palette(TilePixelValue::Three),
        ppu_colors[0]
    );
}
