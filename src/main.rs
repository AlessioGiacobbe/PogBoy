extern crate core;

mod op_codes_parser;
mod cartridge;
mod decoder;
mod cpu;
mod mmu;

#[cfg(test)]
mod tests;

use crate::cartridge::cartridge::{Cartridge, read_cartridge};
use crate::cpu::CPU::CPU;
use crate::decoder::decoder::Decoder;
use crate::mmu::mmu::MMU;

fn main() {
    let cartridge: Cartridge = read_cartridge("image.gb");
    let decoder: Decoder = Decoder::new(cartridge.clone());
    let mmu: MMU = MMU::new(Some(cartridge));
    let mut cpu: CPU = CPU::new(Some(decoder), mmu);
    cpu.run();
}
