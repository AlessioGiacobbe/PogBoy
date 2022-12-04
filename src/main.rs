mod op_codes_parser;
mod cartridge;
mod decoder;
mod cpu;

use crate::cartridge::cartridge::{Cartridge, read_cartridge};
use crate::cpu::CPU::CPU;
use crate::decoder::decoder::Decoder;


fn main() {
    let Cartridge: Cartridge = read_cartridge("snake.gb");
    let Decoder: Decoder = Decoder::new(Cartridge);
    let CPU: CPU = CPU::new(Decoder);
}
