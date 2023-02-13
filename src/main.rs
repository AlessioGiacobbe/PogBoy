extern crate core;

mod op_codes_parser;
mod cartridge;
mod cpu;
mod mmu;
mod ppu;
mod interrupt;

#[cfg(test)]
mod tests;

use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::{env, thread};
use std::path::Path;
use std::time::Duration;
use image;
use image::{RgbaImage};
use image::ColorType::{Rgb8, Rgba8};
use piston_window::{Button, image as draw_image, ButtonState, Context, Event, Input, Key, PistonWindow, Texture, TextureContext, TextureSettings, WindowSettings};
use crate::cartridge::cartridge::{Cartridge, read_cartridge};
use crate::cpu::CPU::CPU;
use crate::mmu::mmu::MMU;
use crate::ppu::ppu::{PPU, PPU_mode, tile_set_to_rgba_image};

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_name = args.last().unwrap().clone();

    let (cpu_sender, _) : (Sender<&Vec<u8>>, Receiver<&Vec<u8>>) = mpsc::channel();
    let (window_sender, cpu_receiver) : (Sender<bool>, Receiver<bool>) = mpsc::channel();

    let image_buffer = Arc::new(Mutex::new(RgbaImage::new(160, 144)));
    let image_buffer_reference = image_buffer.clone();

    let cpu_thread = thread::spawn(move|| run_cpu(cpu_sender, cpu_receiver, image_buffer_reference, rom_name));

    let mut window: PistonWindow = WindowSettings::new("Pog!", [160, 144]).exit_on_esc(true).build().unwrap();

    let (mut texture, mut texture_context) = {
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into()
        };
        let image_buffer = image::ImageBuffer::new(160, 144);
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

                window.draw_2d(&event, |c: Context, g, device| {

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

fn run_cpu(_: Sender<&Vec<u8>>, cpu_receiver: Receiver<bool>, image_buffer_reference: Arc<Mutex<RgbaImage>>, rom_name: String) {
    let cartridge: Cartridge = read_cartridge(&rom_name);

    let mut ppu: PPU = PPU::new();
    let mmu: MMU = MMU::new(Some(cartridge), &mut ppu);
    let mut cpu: CPU = CPU::new(mmu);

    loop {
        let clock = cpu.step();
        let current_cpu_mode = cpu.MMU.PPU.step(clock);

        if current_cpu_mode == PPU_mode::HBlank {
            let mut image_buffer = image_buffer_reference.lock().unwrap();
            (*image_buffer) = cpu.MMU.PPU.image_buffer.clone();
            //let tile_set_dump = tile_set_to_rgba_image(cpu.MMU.PPU.tile_set);
            //(*image_buffer) = tile_set_dump;
        }

        //TODO receive real input from window key press
        let received = cpu_receiver.try_recv();
        if received.is_ok() {
            let tile_set_dump: RgbaImage = tile_set_to_rgba_image(cpu.MMU.PPU.tile_set);
            image::save_buffer(&Path::new("last_tile_set.png"), &*tile_set_dump.into_vec(), 20 * 8, 20 * 8, Rgba8).expect("TODO: panic message");
            break
        }
    }
}
