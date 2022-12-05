extern crate core;

mod op_codes_parser;
mod cartridge;
mod decoder;
mod cpu;

use crate::cartridge::cartridge::{Cartridge, read_cartridge};
use crate::cpu::CPU::CPU;
use crate::decoder::decoder::Decoder;


fn main() {
    let Cartridge: Cartridge = read_cartridge("image.gb");
    //println!("{}", Cartridge.CartridgeInfo);
    let Decoder: Decoder = Decoder::new(Cartridge);
    //Decoder.disassemble(512, 16);
    //Decoder.disassemble(512, 16);
    //Decoder.disassemble(0, 0);
    let mut CPU: CPU = CPU::new(Decoder);
    CPU.run();
}
