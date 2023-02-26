use piston_window::Key;
use crate::cpu::CPU::{CPU, InterruptType};
use crate::tests::factories::{create_dummy_gamepad, create_dummy_mmu, create_dummy_ppu};

const INTERRUPT_ENABLED_ADDRESS: i32 = 0xFFFF;
const INTERRUPT_FLAG_ADDRESS: i32 = 0xFF0F;

#[test]
fn gamepad_works() {
    let mut dummy_gamepad = create_dummy_gamepad();

    assert_eq!(dummy_gamepad.read(), 0x0);

    dummy_gamepad.write(0x20);
    assert_eq!(dummy_gamepad.read(), 0xF);

    dummy_gamepad.write(0x20);
    dummy_gamepad.key_pressed(Key::Z);
    assert_eq!(dummy_gamepad.read(), 0b1110);

    dummy_gamepad.key_released(Key::Z);
    assert_eq!(dummy_gamepad.read(), 0xF);

    dummy_gamepad.key_pressed(Key::Z);
    dummy_gamepad.key_pressed(Key::X);
    assert_eq!(dummy_gamepad.read(), 0b1100);

    dummy_gamepad.write(0x10);
    dummy_gamepad.key_pressed(Key::Down);
    assert_eq!(dummy_gamepad.read(), 0b0111);
    dummy_gamepad.key_pressed(Key::Right);
    assert_eq!(dummy_gamepad.read(), 0b0110);
}


#[test]
fn interrupt_checks_works(){
    let mut dummy_ppu = create_dummy_ppu();
    let dummy_mmu = create_dummy_mmu(&mut dummy_ppu);
    let mut cpu = CPU::new(dummy_mmu);

    cpu.MMU.interrupt_master_enabled = 1;
    cpu.check_interrupts();
    assert_eq!(cpu.MMU.interrupt_master_enabled, 1);

    cpu.MMU.write_byte(INTERRUPT_ENABLED_ADDRESS, 0xFF);
    cpu.request_interrupt(InterruptType::VBlank);
    cpu.check_interrupts();
    assert_eq!(cpu.MMU.read_byte(INTERRUPT_FLAG_ADDRESS), 0xE0);    //initial value because 0x1 should be unset after interrupt handling
    assert_eq!(cpu.MMU.interrupt_master_enabled, 0);

    cpu.request_interrupt(InterruptType::Serial);
    cpu.check_interrupts();
    assert_eq!(cpu.MMU.read_byte(INTERRUPT_FLAG_ADDRESS), 0xE8);    //nothing happened because interrupt_master_enabled is still 0
    assert_eq!(cpu.MMU.interrupt_master_enabled, 0);

    cpu.MMU.interrupt_master_enabled = 1;
    cpu.MMU.write_byte(INTERRUPT_ENABLED_ADDRESS, 0x0);
    cpu.check_interrupts();
    assert_eq!(cpu.MMU.read_byte(INTERRUPT_FLAG_ADDRESS), 0xE8);    //nothing happened because interrupt_enabled is 0 (no interrupts allowed)

    cpu.MMU.write_byte(INTERRUPT_ENABLED_ADDRESS, 0xFF);
    cpu.check_interrupts();
    assert_eq!(cpu.MMU.read_byte(INTERRUPT_FLAG_ADDRESS), 0xE0);
}


#[test]
fn interrupt_handler_sets_right_pc_address_and_SP(){
    let mut dummy_ppu = create_dummy_ppu();
    let dummy_mmu = create_dummy_mmu(&mut dummy_ppu);
    let mut cpu = CPU::new(dummy_mmu);

    cpu.Registers.PC = 42; //dummy value
    cpu.MMU.interrupt_master_enabled = 1;
    cpu.MMU.write_byte(INTERRUPT_ENABLED_ADDRESS, 0xFF);
    cpu.request_interrupt(InterruptType::Serial);
    cpu.check_interrupts();
    assert_eq!(cpu.Registers.PC, 0x58);
    assert_eq!(cpu.read_from_stack(), 42);
}