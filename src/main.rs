extern crate core;

mod op_codes_parser;
mod cartridge;
mod cpu;
mod mmu;
mod ppu;
mod interrupt;

#[cfg(test)]
mod tests;

use std::borrow::Borrow;
use std::sync::mpsc;
use std::sync::mpsc::RecvError;
use std::thread;
use piston_window::{clear, Context, Event, Glyphs, Input, math, PistonWindow, rectangle, text, TextureSettings, WindowSettings};
use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::types::{Color, Matrix2d};
use crate::cartridge::cartridge::{Cartridge, read_cartridge};
use crate::cpu::CPU::CPU;
use crate::mmu::mmu::MMU;
use crate::ppu::ppu::PPU;

fn main() {

    let (cpu_sender, window_receiver) = mpsc::sync_channel(1);
    let (window_sender, cpu_receiver) = mpsc::channel();

    let cpu_thread = thread::spawn(move|| {
        let cartridge: Cartridge = read_cartridge("image.gb");
        let mut ppu: PPU = PPU::new();
        let mut mmu: MMU = MMU::new(Some(cartridge), &mut ppu);
        let mut cpu: CPU = CPU::new(mmu);

        loop {
            let clock = cpu.step();
            cpu.MMU.PPU.step(clock);
            let _ = cpu_sender.send(clock).expect("errore");

            //TODO receive real input from window key press
            let received = cpu_receiver.try_recv();
            if received.is_ok() {
                break
            }
        }
    });

    let mut window: PistonWindow = WindowSettings::new("Pog!", [160, 144]).exit_on_esc(true).build().unwrap();

    let mut glyphs: Glyphs = window.load_font("./src/assets/crash-a-like.ttf").unwrap();

    while let Some(event) = window.next() {
        match event {
            Event::Input(input, _) => {
                match input {
                    Input::Button(_) => {}
                    Input::Move(_) => {}
                    Input::Text(_) => {}
                    Input::Resize(_) => {}
                    Input::Focus(_) => {}
                    Input::Cursor(_) => {}
                    Input::FileDrag(_) => {}
                    Input::Close(_) => {
                        window_sender.send(true).unwrap();
                    }
                }
            }
            Event::Loop(_) => {
                match window_receiver.recv() {
                    Ok(cpu_info) => {
                        window.draw_2d(&event, |c: Context, mut g, device| {
                            clear([0.0, 0.0, 0.0, 1.0], g);
                            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 20).draw_pos(&*cpu_info.to_string(),
                                                                                     [10.0, 50.0],
                                                                                     &mut glyphs,
                                                                                     &c.draw_state,
                                                                                     c.transform,
                                                                                     g).expect("TODO: panic message");

                            glyphs.factory.encoder.flush(device);
                        });
                    },
                    Err(_) => {}
                }
            }
            _ => {}
        }

    }

    let _ = cpu_thread.join();

}
