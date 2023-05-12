pub mod interrupts {
    use crate::cpu::CPU::{CPU, InterruptType};
    use strum::IntoEnumIterator;

    impl CPU<'_> {
        pub(crate) fn request_interrupt(&mut self, interrupt_type: InterruptType) {
            self.MMU.interrupt_flag = self.MMU.interrupt_flag | (interrupt_type as u8)
        }

        pub(crate) fn check_interrupts(&mut self) -> bool {
            if self.MMU.interrupt_queued {
                return false;
            }

            if ((self.MMU.interrupt_flag & 0b11111) & (self.MMU.interrupt_enabled
             & 0b11111)) != 0 {
                if self.handle_interrupt(InterruptType::VBlank, 0x0040){
                    self.MMU.interrupt_queued = true;
                }else if self.handle_interrupt(InterruptType::LCD_STAT, 0x0048) {
                    self.MMU.interrupt_queued = true;
                }else if self.handle_interrupt(InterruptType::Timer, 0x0050){
                    self.MMU.interrupt_queued = true;
                }else if self.handle_interrupt(InterruptType::Serial, 0x0058) {
                    self.MMU.interrupt_queued = true;
                }else if self.handle_interrupt(InterruptType::Joypad, 0x0060){
                    self.MMU.interrupt_queued = true;
                }else {
                    self.MMU.interrupt_queued = false;
                }
                return true;
            }else{
                self.MMU.interrupt_queued = false;
            }
            return false;
        }

        pub(crate) fn handle_interrupt(&mut self, interrupt_type: InterruptType, address: u16) -> bool {
            if (self.MMU.interrupt_enabled & interrupt_type as u8) != 0 && (self.MMU.interrupt_flag & interrupt_type as u8) != 0 {
                if self.is_halted {
                    let current_pc = self.Registers.get_item("PC");
                    self.Registers.set_item("PC", current_pc + 1)
                }

                if self.MMU.interrupt_master_enabled != 0 {
                    self.MMU.interrupt_flag ^= interrupt_type as u8;
                    self.write_to_stack(self.Registers.PC);
                    self.Registers.PC = address;
                    self.MMU.interrupt_master_enabled = 0;
                }
                return true;
            }
            return false;
        }
    }
}