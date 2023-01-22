pub mod ppu {
    use std::fmt::{Debug, Display, Formatter};
    use crate::cpu::CPU::CPU;

    const COLORS: [&'static str; 4] = ["#0f380f", "#306230", "#8bac0f", "#9bbc0f"];

    pub struct PPU {
        pub(crate) framebuffer: [u8; 160 * 144 * 3],
        pub(crate) alpha_framebuffer: [u8; 160 * 144 * 4],
        pub(crate) clock: u32,
        pub(crate) mode: u8,
        pub(crate) video_ram: [u8; 0x2000]
    }

    impl Debug for PPU {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "PPU - mode : {} - clock : {}", self.mode, self.clock)
        }
    }

    impl PPU {

        pub(crate) fn new() -> PPU {
           PPU {
               framebuffer: [0; 69120],
               alpha_framebuffer: [0; 92160],
               video_ram: [0; 0x2000],
               clock: 0,
               mode: 0,
           }
        }

        pub(crate) fn step(&mut self, clock: u32) {
            self.clock += clock;
            println!("Clocckando {}", self.clock);

            match self.mode {
                0 => {

                },
                1 => {

                },
                2 => {

                },
                3 => {

                },
                _ => {}
            }
        }

        pub(crate) fn read_byte(& self, address: usize) -> u8 {
            0
        }

        pub(crate) fn write_byte(&mut self, address: usize, value: u8) {

        }
    }

}