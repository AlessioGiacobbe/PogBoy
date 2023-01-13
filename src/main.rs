extern crate core;

mod op_codes_parser;
mod cartridge;
mod cpu;
mod mmu;
mod interrupt;

#[cfg(test)]
mod tests;

use crate::cartridge::cartridge::{Cartridge, read_cartridge};
use crate::cpu::CPU::CPU;
use crate::mmu::mmu::MMU;

fn main() {
    let cartridge: Cartridge = read_cartridge("image.gb");
    let mmu: MMU = MMU::new(Some(cartridge));
    let mut cpu: CPU = CPU::new(mmu);
    cpu.run();
}
