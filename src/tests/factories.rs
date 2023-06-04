use crate::io::gamepad::gamepad::gamepad;
use crate::memory::cartridge::cartridge::Cartridge;
use crate::memory::mmu::mmu::MMU;
use crate::memory::op_codes_parser::op_codes_parser::{Instruction, Operand};
use crate::ppu::ppu::{Tile, TilePixelValue, PPU};

pub(crate) fn create_dummy_cartridge() -> Cartridge {
    let Cartridge: Cartridge = Cartridge {
        cartridge_info: None,
        rom: vec![],
    };
    let mut rom = vec![0; 0x108];
    rom[0x100] = 0x00;
    rom[0x101] = 0x3E;
    rom[0x102] = 0x0F;
    rom[0x103] = 0xCB;
    rom[0x104] = 0x7C;
    rom[0x105] = 0x21;
    rom[0x106] = 0xFE;
    rom[0x107] = 0xC0;
    Cartridge {
        cartridge_info: Cartridge.cartridge_info,
        rom, //NOP - LD A,0x0F
    }
}

pub(crate) fn create_dummy_tile() -> Tile {
    [
        [
            TilePixelValue::Zero,
            TilePixelValue::Two,
            TilePixelValue::Three,
            TilePixelValue::Three,
            TilePixelValue::Three,
            TilePixelValue::Three,
            TilePixelValue::Two,
            TilePixelValue::Three,
        ],
        [
            TilePixelValue::Zero,
            TilePixelValue::Three,
            TilePixelValue::Zero,
            TilePixelValue::Zero,
            TilePixelValue::Zero,
            TilePixelValue::Zero,
            TilePixelValue::Three,
            TilePixelValue::Zero,
        ],
        [
            TilePixelValue::Zero,
            TilePixelValue::Three,
            TilePixelValue::Zero,
            TilePixelValue::Zero,
            TilePixelValue::Zero,
            TilePixelValue::Zero,
            TilePixelValue::Three,
            TilePixelValue::Zero,
        ],
        [
            TilePixelValue::Zero,
            TilePixelValue::Three,
            TilePixelValue::Zero,
            TilePixelValue::Zero,
            TilePixelValue::Zero,
            TilePixelValue::Zero,
            TilePixelValue::Three,
            TilePixelValue::Zero,
        ],
        [
            TilePixelValue::Zero,
            TilePixelValue::Three,
            TilePixelValue::One,
            TilePixelValue::Three,
            TilePixelValue::Three,
            TilePixelValue::Three,
            TilePixelValue::Three,
            TilePixelValue::Zero,
        ],
        [
            TilePixelValue::Zero,
            TilePixelValue::One,
            TilePixelValue::One,
            TilePixelValue::One,
            TilePixelValue::Three,
            TilePixelValue::One,
            TilePixelValue::Three,
            TilePixelValue::Zero,
        ],
        [
            TilePixelValue::Zero,
            TilePixelValue::Three,
            TilePixelValue::One,
            TilePixelValue::Three,
            TilePixelValue::One,
            TilePixelValue::Three,
            TilePixelValue::Two,
            TilePixelValue::Zero,
        ],
        [
            TilePixelValue::Zero,
            TilePixelValue::Two,
            TilePixelValue::Three,
            TilePixelValue::Three,
            TilePixelValue::Three,
            TilePixelValue::Two,
            TilePixelValue::Zero,
            TilePixelValue::Zero,
        ],
    ]
}

pub(crate) fn create_dummy_gamepad() -> gamepad {
    gamepad::default()
}

pub(crate) fn create_dummy_ppu() -> PPU {
    PPU::new()
}

pub(crate) fn create_dummy_mmu(dummy_ppu: &mut PPU) -> MMU {
    let dummy_cartridge = create_dummy_cartridge();
    let mut dummy_mmu = MMU::new(Some(dummy_cartridge), dummy_ppu);
    dummy_mmu.PPU.video_ram = [1; 0x2000];
    dummy_mmu.external_ram = [2; 0x2000];
    dummy_mmu.work_ram = [3; 0x2000];
    dummy_mmu.io_registers = [4; 0x100];
    dummy_mmu.high_ram = [5; 0x80];
    dummy_mmu
}

pub(crate) fn create_dummy_instruction(operand_name: &str, operand_value: u16) -> Instruction {
    Instruction {
        opcode: 0,
        immediate: false,
        operands: vec![Operand {
            immediate: false,
            name: operand_name.parse().unwrap(),
            bytes: None,
            value: Some(operand_value),
            adjust: None,
        }],
        cycles: vec![],
        bytes: 0,
        mnemonic: "".to_string(),
        comment: None,
        prefixed: false,
    }
}
