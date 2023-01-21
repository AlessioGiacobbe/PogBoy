extern crate core;

mod op_codes_parser;
mod cartridge;
mod cpu;
mod mmu;
mod ppu;
mod interrupt;

#[cfg(test)]
mod tests;

use piston_window::{clear, Event, PistonWindow, rectangle, WindowSettings};
use crate::cartridge::cartridge::{Cartridge, read_cartridge};
use crate::cpu::CPU::CPU;
use crate::mmu::mmu::MMU;
use crate::ppu::ppu::PPU;

fn main() {
    let cartridge: Cartridge = read_cartridge("image.gb");
    let mmu: MMU = MMU::new(Some(cartridge));
    let ppu: PPU = PPU::new();
    let mut cpu: CPU = CPU::new(mmu, ppu);

    let mut window: PistonWindow = WindowSettings::new("Pog!", [160, 144]).exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        match event {
            Event::Input(_, _) => {}
            Event::Loop(_) => {
                cpu.step();

                window.draw_2d(&event, |context, graphics, _device| {
                });
            }
            _ => {}
        }

    }
}
