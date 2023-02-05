extern crate core;

mod op_codes_parser;
mod cartridge;
mod cpu;
mod mmu;
mod ppu;
mod interrupt;

#[cfg(test)]
mod tests;

use std::any::Any;
use std::borrow::Borrow;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvError, Sender, SyncSender};
use std::thread;
use image;
use gfx;
use image::{ImageBuffer, Rgba};
use piston_window::{Button, image as draw_image, ButtonState, clear, Context, Event, Glyphs, Input, Key, math, PistonWindow, rectangle, text, Texture, TextureContext, TextureSettings, WindowSettings};
use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::types::{Color, Matrix2d};
use crate::cartridge::cartridge::{Cartridge, read_cartridge};
use crate::cpu::CPU::CPU;
use crate::mmu::mmu::MMU;
use crate::ppu::ppu::PPU;

fn main() {

    let (cpu_sender, window_receiver) : (Sender<ImageBuffer<Rgba<u8>, Vec<u8>>>, Receiver<ImageBuffer<Rgba<u8>, Vec<u8>>>) = mpsc::channel();
    let (window_sender, cpu_receiver) : (Sender<bool>, Receiver<bool>) = mpsc::channel();

    let cpu_thread = thread::spawn(move|| run_cpu(cpu_sender, cpu_receiver));

    let mut window: PistonWindow = WindowSettings::new("Pog!", [160, 144]).exit_on_esc(true).build().unwrap();

    let (mut texture, mut texture_context) = {
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into()
        };
        let mut image_buffer = image::ImageBuffer::new(160, 144);
        let texture = Texture::from_image(
            &mut texture_context,
            &image_buffer,
            &TextureSettings::new()
        ).unwrap();
        (texture, texture_context)
    };

    let mut acc = 0;
    while let Some(event) = window.next() {
        match event {
            Event::Input(input, _) => {
                match input {
                    Input::Button(ButtonArgs) => {
                        match ButtonArgs.state {
                            ButtonState::Press => {
                                match ButtonArgs.button {
                                    Button::Keyboard(Key) => {
                                        match Key {
                                            Key::Escape => {
                                                window_sender.send(true).unwrap();
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            ButtonState::Release => {}
                        }
                    }
                    Input::Close(_) => {
                        window_sender.send(true).unwrap();
                    }
                    _ => {}
                }
            }
            Event::Loop(_) => {
                match window_receiver.try_recv() {
                    Ok(current_image_buffer) => {
                        window.draw_2d(&event, |c: Context, mut g, device| {
                            clear([0.0, 0.0, 0.0, 1.0], g);

                            texture.update(
                                &mut texture_context,
                                &current_image_buffer).unwrap();

                            draw_image(&texture, c.transform, g);
                            texture_context.encoder.flush(device);
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

fn run_cpu(cpu_sender: Sender<ImageBuffer<Rgba<u8>, Vec<u8>>>, cpu_receiver: Receiver<bool>) {
    let cartridge: Cartridge = read_cartridge("image.gb");
    let mut ppu: PPU = PPU::new();
    let mut mmu: MMU = MMU::new(Some(cartridge), &mut ppu);
    let mut cpu: CPU = CPU::new(mmu);

    loop {
        let clock = cpu.step();
        cpu.MMU.PPU.step(clock);
        let _ = cpu_sender.send(cpu.MMU.PPU.image_buffer.clone()).expect("error sending to window");

        //TODO receive real input from window key press
        let received = cpu_receiver.try_recv();
        if received.is_ok() {
            break
        }
    }
}
