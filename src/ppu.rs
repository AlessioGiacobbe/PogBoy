pub mod ppu {
    use piston_window::{clear, PistonWindow, rectangle, WindowSettings};
    use crate::mmu::mmu::MMU;

    const COLORS: [&'static str; 4] = ["#0f380f", "#306230", "#8bac0f", "#9bbc0f"];

    pub struct PPU {
        pub(crate) framebuffer: [u8; 160 * 144 * 3],
        pub(crate) alpha_framebuffer: [u8; 160 * 144 * 4],
        pub(crate) clock: u32,
        pub(crate) mode: u8,
    }

    impl PPU {

        pub(crate) fn new() -> PPU {
           PPU {
               framebuffer: [0; 69120],
               alpha_framebuffer: [0; 92160],
               clock: 0,
               mode: 0,
           }
        }

        pub(crate) fn step(&mut self, clock: u32) {
            self.clock += clock;

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

    }

}