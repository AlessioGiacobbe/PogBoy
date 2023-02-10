extern crate core;

mod op_codes_parser;
mod cartridge;
mod cpu;
mod mmu;
mod ppu;
mod tests;
mod interrupt;

#[cfg(test)]
mod tests;

use std::any::Any;
use std::borrow::Borrow;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, RecvError, Sender, SyncSender};
use std::thread;
use imgui::*;
use image;
use gfx;
use image::{ImageBuffer, Rgba, RgbaImage};
use piston_window::{Button, image as draw_image, ButtonState, clear, Context, Event, Glyphs, Input, Key, math, PistonWindow, rectangle, text, Texture, TextureContext, TextureSettings, WindowSettings};
use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::types::{Color, Matrix2d};
use crate::cartridge::cartridge::{Cartridge, read_cartridge};
use crate::cpu::CPU::CPU;
use crate::mmu::mmu::MMU;
use crate::ppu::ppu::PPU;

fn main() {

    let (cpu_sender, window_receiver) : (Sender<&Vec<u8>>, Receiver<&Vec<u8>>) = mpsc::channel();
    let (window_sender, cpu_receiver) : (Sender<bool>, Receiver<bool>) = mpsc::channel();

    let image_buffer = Arc::new(Mutex::new(RgbaImage::new(160, 144)));
    let image_buffer_reference = image_buffer.clone();

    let cpu_thread = thread::spawn(move|| run_cpu(cpu_sender, cpu_receiver, image_buffer_reference));

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
                                                println!("{:?}", *image_buffer.lock().unwrap());
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

                //println!("array condiviso {:?}", .unwrap())

                window.draw_2d(&event, |c: Context, mut g, device| {
                    clear([0.0, 0.0, 0.0, 1.0], g);

                    texture.update(&mut texture_context, &*image_buffer.lock().unwrap()).unwrap();

                    draw_image(&texture, c.transform, g);
                    texture_context.encoder.flush(device);
                });
            }
            _ => {}
        }

    }

    let _ = cpu_thread.join();

}

fn run_cpu(cpu_sender: Sender<&Vec<u8>>, cpu_receiver: Receiver<bool>, image_buffer_reference: Arc<Mutex<RgbaImage>>) {
    let cartridge: Cartridge = read_cartridge("image.gb");

    let mut ppu: PPU = PPU::new();
    let mut mmu: MMU = MMU::new(Some(cartridge), &mut ppu);
    let mut cpu: CPU = CPU::new(mmu);

    loop {
        let clock = cpu.step();

        let mut image_buffer = image_buffer_reference.lock().unwrap();
        (*image_buffer) = cpu.MMU.PPU.image_buffer.clone();
        cpu.MMU.PPU.step(clock);

        //TODO receive real input from window key press
        let received = cpu_receiver.try_recv();
        if received.is_ok() {
            break
        }
    }
}
