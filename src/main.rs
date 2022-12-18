extern crate core;

mod op_codes_parser;
mod cartridge;
mod decoder;
mod cpu;
#[cfg(test)]
mod tests;

use crate::cartridge::cartridge::{Cartridge, read_cartridge};
use crate::cpu::CPU::CPU;
use crate::decoder::decoder::Decoder;

fn main() {
    let cartridge: Cartridge = read_cartridge("image.gb");
    let decoder: Decoder = Decoder::new(cartridge);
    //Decoder.disassemble(512, 16);
    //Decoder.disassemble(512, 16);
    //Decoder.disassemble(0, 0);
    let mut cpu: CPU = CPU::new(Some(decoder));
    cpu.run();
}
