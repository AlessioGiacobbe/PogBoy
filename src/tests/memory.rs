use crate::cpu::CPU::CPU;
use crate::tests::factories::{create_dummy_mmu, create_dummy_ppu};

#[test]
fn decoder_can_parse_correctly(){
    let mut dummy_ppu = create_dummy_ppu();
    let dummy_mmu = create_dummy_mmu(&mut dummy_ppu);
    let (next_address, nop_instruction) = dummy_mmu.decode(0x100);
    let (next_address, ld_a_d8_instruction) = dummy_mmu.decode(next_address);
    let (next_address, bit_7_h) = dummy_mmu.decode(next_address);  //CB PREFIXED
    let (_, ld_hl_d16_instruction) = dummy_mmu.decode(next_address);
    println!("{} NOP INSTRUCT2iO", nop_instruction);
    assert_eq!(nop_instruction.mnemonic, "NOP");
    let d8 = ld_a_d8_instruction.operands.into_iter().find(|operand| operand.name == "d8").unwrap();
    assert_eq!(d8.value.unwrap(), 0x0F);
    assert_eq!(bit_7_h.prefixed, true);
    let d16 = ld_hl_d16_instruction.operands.into_iter().find(|operand| operand.name == "d16").unwrap();
    assert_eq!(d16.value.unwrap(), 0xC0FE);
}


#[test]
fn memory_can_read_and_write(){
    let mut dummy_ppu = create_dummy_ppu();
    let mut dummy_mmu = create_dummy_mmu(&mut dummy_ppu);

    assert_eq!(dummy_mmu.read_byte(0x0), 0x31);
    dummy_mmu.write_byte(0x0, 0xFF);
    assert_eq!(dummy_mmu.read_byte(0x0), 0xFF);

    //TODO implement PPU memory tests
    assert_eq!(dummy_mmu.read_byte(0x8000), 0x1);
    dummy_mmu.write_byte(0x8000, 0xFF);
    assert_eq!(dummy_mmu.read_byte(0x8000), 0xFF);

    assert_eq!(dummy_mmu.read_byte(0xA000), 0x2);
    dummy_mmu.write_byte(0xA000, 0xFF);
    assert_eq!(dummy_mmu.read_byte(0xA000), 0xFF);

    assert_eq!(dummy_mmu.read_byte(0xC000), 0x3);
    dummy_mmu.write_byte(0xC000, 0xFF);
    assert_eq!(dummy_mmu.read_byte(0xC000), 0xFF);

    assert_eq!(dummy_mmu.read_byte(0xFF80), 0x5);
    dummy_mmu.write_byte(0xFF80, 0xFF);
    assert_eq!(dummy_mmu.read_byte(0xFF80), 0xFF);

    assert_eq!(dummy_mmu.read_byte(0xFFFF), 0);
    dummy_mmu.write_byte(0xFFFF, 0x0);
    assert_eq!(dummy_mmu.read_byte(0xFFFF), 0x0);

    dummy_mmu.write_word(0xA000, 0xC0FE);
    assert_eq!(dummy_mmu.read_word(0xA000), 0xC0FE);
}


#[test]
fn push_and_pop_works(){
    let mut dummy_ppu = create_dummy_ppu();
    let dummy_mmu = create_dummy_mmu(&mut dummy_ppu);
    let mut cpu = CPU::new(dummy_mmu);

    assert_eq!(cpu.Registers.get_item("SP"), 0xFFFE);
    cpu.write_to_stack(0xC0FE);
    assert_eq!(cpu.Registers.get_item("SP"), 0xFFFE - 2);
    assert_eq!(cpu.read_from_stack(), 0xC0FE);

    cpu.write_to_stack(0xFEAB);
    assert_eq!(cpu.read_from_stack(), 0xFEAB);

    cpu.Registers.set_item("BC", 0xFFEE);
    cpu.push_rr("BC");
    cpu.pop_rr("DE");
    assert_eq!(cpu.Registers.get_item("DE"), 0xFFEE)
}