pub mod interrupts {
    use crate::cpu::CPU::{CPU, InterruptType};
    use strum::IntoEnumIterator;

    impl CPU<'_> {
        pub(crate) fn request_interrupt(&mut self, interrupt_type: InterruptType) {
            self.MMU.interrupt_flag = self.MMU.interrupt_flag | (interrupt_type as u8)
        }

        pub fn disable_interrupt(&mut self) {
            self.MMU.interrupt_master_enabled = false;
        }

        pub fn enable_interrupt(&mut self) {
            self.MMU.interrupt_master_enabled = true;
        }

        pub(crate) fn check_interrupts(&mut self) -> u32 {
            if !self.MMU.interrupt_master_enabled {
                if self.is_halted {
                    if let Some(_) = self.should_handle_something() {
                        self.is_halted = false;
                    }
                }
                return 0;
            }

            match self.should_handle_something() {
                None => 0,
                Some(interrupt_type) => {
                    match interrupt_type {
                        InterruptType::VBlank => self.handle_interrupt(InterruptType::VBlank, 0x0040),
                        InterruptType::LCD_STAT => self.handle_interrupt(InterruptType::LCD_STAT, 0x0048),
                        InterruptType::Timer => self.handle_interrupt(InterruptType::Timer, 0x0050),
                        InterruptType::Serial => self.handle_interrupt(InterruptType::Serial, 0x0058),
                        InterruptType::Joypad => self.handle_interrupt(InterruptType::Joypad, 0x0060)
                    };
                    self.is_halted = false;
                    16
                }
            }
        }

        fn should_handle_something(&self) -> Option<InterruptType> {
            for interrupt_type in InterruptType::iter() {
                if self.should_handle(interrupt_type) { return Some(interrupt_type); }
            };
            return None
        }

        fn should_handle(&self, interrupt_type: InterruptType) -> bool {
            (self.MMU.interrupt_enabled & interrupt_type as u8) != 0 && (self.MMU.interrupt_flag & interrupt_type as u8) != 0
        }

        pub(crate) fn handle_interrupt(&mut self, interrupt_type: InterruptType, address: u16) {
            self.disable_interrupt();
            self.MMU.interrupt_flag ^= interrupt_type as u8;
            self.write_to_stack(self.Registers.PC);
            self.Registers.PC = address;
            self.MMU.interrupt_master_enabled = false;
        }
    }
}