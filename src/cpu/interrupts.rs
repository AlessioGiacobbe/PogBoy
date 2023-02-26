pub mod interrupts {
    use crate::cpu::CPU::{CPU, InterruptType};
    use strum::IntoEnumIterator;


    impl CPU<'_> {
        pub(crate) fn request_interrupt(&mut self, interrupt_type: InterruptType) {
            self.MMU.interrupt_flag = self.MMU.interrupt_flag | (interrupt_type as u8)
        }

        pub(crate) fn check_interrupts(&mut self) {
            let master_interrupts_enabled = self.MMU.interrupt_master_enabled != 0;
            let interrupts_enabled = self.MMU.interrupt_enabled != 0;
            let interrupts_flags_enabled = (self.MMU.interrupt_flag & 0x1F) != 0;

            if master_interrupts_enabled && interrupts_enabled && interrupts_flags_enabled {

                for interrupt_type in InterruptType::iter() {
                    let interrupt_bit = interrupt_type as u8;
                    if (self.MMU.interrupt_enabled & interrupt_bit != 0) && (self.MMU.interrupt_flag & interrupt_bit != 0) {
                        self.handle_interrupt(interrupt_type)
                    }
                }

            }
        }

        pub(crate) fn handle_interrupt(&mut self, interrupt_type: InterruptType) {
            self.MMU.interrupt_flag = self.MMU.interrupt_flag & !(interrupt_type as u8); //reset currently handled interrupt bit
            self.MMU.interrupt_master_enabled = 0;

            if self.is_halted {
                self.is_halted = false;
                self.Registers.PC += 1;
            }

            self.write_to_stack(self.Registers.PC);
            self.Registers.PC = match interrupt_type {
                InterruptType::VBlank => 0x40,
                InterruptType::LCD_STAT => 0x48,
                InterruptType::Timer => 0x50,
                InterruptType::Serial => 0x58,
                InterruptType::Joypad => 0x60
            };
            println!("handling {}", self.Registers.PC)
        }
    }
}