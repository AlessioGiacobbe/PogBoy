extern crate core;

mod cpu;
mod io;
mod memory;
mod ppu;

#[cfg(test)]
mod tests;

use crate::cpu::CPU::InterruptType;
use crate::cpu::CPU::CPU;
use crate::memory::cartridge::cartridge::{read_cartridge, Cartridge, CartridgeInfo};
use crate::memory::mmu;
use crate::memory::mmu::mmu::MMU;
use crate::ppu::ppu::{
    dump_current_screen_tiles, dump_tile_map, tile_set_to_rgba_image, PpuMode, PPU,
};
use image;
use image::ColorType::{Rgb8, Rgba8};
use image::RgbaImage;
use piston_window::{image as draw_image, Button, ButtonState, Context, Event, Input, Key, PistonWindow, Texture, TextureContext, TextureSettings, WindowSettings, AdvancedWindow};
use std::fs::File;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};
use std::{env, fs, thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_name = args.last().unwrap().clone();

    let (cpu_sender, window_receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (window_sender, cpu_receiver): (Sender<(Key, ButtonState)>, Receiver<(Key, ButtonState)>) =
        mpsc::channel();

    let image_buffer = Arc::new(Mutex::new(RgbaImage::new(160, 144)));
    let image_buffer_reference = image_buffer.clone();

    let cpu_thread =
        thread::spawn(move || run_cpu(cpu_sender, cpu_receiver, image_buffer_reference, rom_name));

    let mut window: PistonWindow = WindowSettings::new("Pog!", [160, 144])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let (mut texture, mut texture_context) = {
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };
        let image_buffer = image::ImageBuffer::new(160, 144);
        let texture =
            Texture::from_image(&mut texture_context, &image_buffer, &TextureSettings::new())
                .unwrap();
        (texture, texture_context)
    };

    while let Some(event) = window.next() {
        match event {
            Event::Input(input, _) => match input {
                Input::Button(ButtonArgs) => match ButtonArgs.button {
                    Button::Keyboard(key) => {
                        window_sender.send((key, ButtonArgs.state)).unwrap();
                    }
                    _ => {}
                },
                Input::Close(_) => {
                    window_sender
                        .send((Key::Escape, ButtonState::Press))
                        .unwrap();
                },
                Input::Resize(resize_args) => {
                    //println!("{} - {}", resize_args.window_size[0], resize_args.window_size[1])
                }
                _ => {}
            },
            Event::Loop(_) => {
                let received = window_receiver.try_recv();
                if received.is_ok() {
                    window.set_title(received.unwrap());
                }
                window.draw_2d(&event, |c: Context, g, device| {
                    texture
                        .update(&mut texture_context, &*image_buffer.lock().unwrap())
                        .unwrap();
                    draw_image(&texture, c.transform, g);
                    texture_context.encoder.flush(device);
                });
            }
            _ => {}
        }
    }

    let _ = cpu_thread.join();
}

fn run_cpu(
    cpu_sender: Sender<String>,
    cpu_receiver: Receiver<(Key, ButtonState)>,
    image_buffer_reference: Arc<Mutex<RgbaImage>>,
    rom_name: String,
) {
    let cartridge: Cartridge = read_cartridge(&rom_name);
    if let cartridge_info = cartridge.cartridge_info.unwrap() {
        cpu_sender.send(cartridge_info.game_title().to_owned()).expect("Can't read cartridge title");
    }

    let mut ppu: PPU = PPU::new();
    let mmu: MMU = MMU::new(Some(cartridge), &mut ppu);
    let mut cpu: CPU = CPU::new(mmu);

    //cpu.MMU.disassemble(0x300, 0x400, 0x359);
    let mut cycles_delta = 0;
    let cycles_per_frame = 69905;
    let mut time_ref = Instant::now();

    'main: loop {
        let (clock, mut clock_delta) = cpu.step();
        clock_delta += cpu.check_interrupts();
        let (current_ppu_mode, should_rise_vblank_interrupt, should_rise_stat_interrupt) =
            cpu.MMU.PPU.step(clock_delta);
        cpu.increment_timer(clock_delta as i32);

        cycles_delta += clock_delta;

        if cycles_delta >= cycles_per_frame {
            let mut image_buffer = image_buffer_reference.lock().unwrap();
            (*image_buffer) = cpu.MMU.PPU.image_buffer.clone();

            let elapsed = Instant::now().duration_since(time_ref);
            time_ref = Instant::now();

            if elapsed.as_micros() < 16670 {
                let sleep_time = time::Duration::from_micros((16670 - elapsed.as_micros()) as u64);
                thread::sleep(sleep_time);
            }

            cycles_delta -= cycles_per_frame;
        }

        if should_rise_vblank_interrupt {
            cpu.request_interrupt(InterruptType::VBlank)
        }

        if should_rise_stat_interrupt {
            cpu.request_interrupt(InterruptType::LCD_STAT)
        }

        let received = cpu_receiver.try_recv();
        if received.is_ok() {
            let (key, state) = received.unwrap();
            match state {
                ButtonState::Press => {
                    match key {
                        Key::Escape => break 'main,
                        Key::L => {
                            //toggle cpu logging
                            cpu.logging = !cpu.logging;
                        }
                        Key::T => {
                            //toggle tileset area
                            if cpu.MMU.read_byte(0xFF40) == 0x91 {
                                cpu.MMU.write_byte(0xFF40, 0x81);
                            } else {
                                cpu.MMU.write_byte(0xFF40, 0x91);
                            }
                        }
                        Key::D => {
                            println!("{}", cpu);

                            //dump current instruction
                            cpu.MMU.disassemble(
                                (cpu.Registers.get_item("PC") - 10) as i32,
                                20,
                                cpu.Registers.get_item("PC") as i32,
                            );

                            //dump current tileset
                            let tile_set_dump: RgbaImage =
                                tile_set_to_rgba_image(cpu.MMU.PPU.tile_set);
                            image::save_buffer(
                                &Path::new("last_tile_set.png"),
                                &*tile_set_dump.into_vec(),
                                20 * 8,
                                20 * 8,
                                Rgba8,
                            )
                            .expect("TODO: panic message");

                            //dump lcdc status
                            cpu.MMU.PPU.print_lcdc_status();

                            let first_tile_map = dump_tile_map(cpu.MMU.PPU.video_ram, 0x1800);
                            fs::write("tm1.txt", first_tile_map).expect("Unable to write file");

                            let second_tile_map = dump_tile_map(cpu.MMU.PPU.video_ram, 0x1C00);
                            fs::write("tm2.txt", second_tile_map).expect("Unable to write file");

                            let current_screen_tiles =
                                format!("{:?}", dump_current_screen_tiles(cpu.MMU.PPU));
                            fs::write("current_screen_tiles.txt", current_screen_tiles)
                                .expect("Unable to write file");

                            //dump interrupt related flags
                            println!(
                                "Interrupts: IF: {:02X}, IE: {:02X}, IME: {}",
                                cpu.MMU.interrupt_flag,
                                cpu.MMU.interrupt_enabled,
                                cpu.MMU.interrupt_master_enabled
                            )
                        }
                        Key::Down
                        | Key::Up
                        | Key::Left
                        | Key::Right
                        | Key::Space
                        | Key::Comma
                        | Key::X
                        | Key::Z => {
                            cpu.MMU.gamepad.key_pressed(key);
                            cpu.request_interrupt(InterruptType::Joypad);
                        }
                        _ => {}
                    }
                }
                ButtonState::Release => match key {
                    Key::Down
                    | Key::Up
                    | Key::Left
                    | Key::Right
                    | Key::Space
                    | Key::Comma
                    | Key::X
                    | Key::Z => cpu.MMU.gamepad.key_released(key),
                    _ => {}
                },
            }
        }
    }
}
