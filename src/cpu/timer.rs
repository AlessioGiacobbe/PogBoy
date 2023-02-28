pub mod timer {
    use crate::cpu::CPU::{CPU, InterruptType};

    const TIMER_FREQUENCIES: [i32; 4] = [4096, 262144, 65536, 16384];  //Hz

    impl CPU<'_> {

        pub fn is_timer_enabled(&mut self) -> bool{
            (self.MMU.timer_control & 0x4) != 0
        }

        pub fn get_timer_frequency(&mut self) -> i32 {
            let frequency_index = (self.MMU.timer_control & 0x3) as usize;
            TIMER_FREQUENCIES[frequency_index]
        }

        pub fn increment_timer(&mut self, clock: i32) {

            self.MMU.timer_divider_clock += clock;
            if self.MMU.timer_divider_clock > 0xFF {
                self.MMU.timer_divider_clock -= clock;
                self.MMU.timer_divider = self.MMU.timer_divider.wrapping_add(1);
            }

            if self.is_timer_enabled() {
                self.MMU.timer_clock += clock;
                let divider = self.get_timer_frequency();

                if self.MMU.timer_clock >= divider {
                    self.MMU.timer_clock -= divider;
                    self.MMU.timer_counter = self.MMU.timer_counter.wrapping_add(1);

                    if self.MMU.timer_counter == 0 {
                        self.MMU.timer_counter = self.MMU.timer_modulo;
                        self.request_interrupt(InterruptType::Timer);
                    }
                }

            }

        }
    }

}