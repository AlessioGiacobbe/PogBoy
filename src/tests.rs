use crate::mmu::mmu::MMU;
use crate::op_codes_parser::op_codes_parser::{Instruction, Operand};
use super::*;

fn create_dummy_cartridge() -> Cartridge {
    let Cartridge: Cartridge = read_cartridge("image.gb");
    Cartridge {
        cartridge_info: Cartridge.cartridge_info,
        rom: vec![0x00, 0x3E, 0x0F],    //NOP - LD A,0x0F
    }
}

fn create_dummy_mmu() -> MMU {
    let dummy_cartridge = create_dummy_cartridge();
    let mut dummy_mmu = MMU::new(Some(dummy_cartridge));
    dummy_mmu.video_ram = [1; 0x2000];
    dummy_mmu.external_ram = [2; 0x2000];
    dummy_mmu.work_ram = [3; 0x2000];
    dummy_mmu.io_registers = [4; 0x100];
    dummy_mmu.high_ram = [5; 0x80];
    dummy_mmu.interrupt_enabled = true;
    dummy_mmu
}

fn create_dummy_decoder() -> Decoder {
    let dummy_cartridge = create_dummy_cartridge();
    Decoder::new(dummy_cartridge)
}

fn create_dummy_cpu() -> CPU {
    let dummy_decoder = create_dummy_decoder();
    let dummy_mmu = create_dummy_mmu();
    CPU::new(Some(dummy_decoder), dummy_mmu)
}

#[test]
fn decoder_can_parse_correctly(){
    let dummy_decoder = create_dummy_decoder();
    let (_, nop_instruction) = dummy_decoder.decode(0);
    let (_, ld_a_d8_instruction) = dummy_decoder.decode(1);
    assert_eq!(nop_instruction.mnemonic, "NOP");
    let d8 = ld_a_d8_instruction.operands.into_iter().find(|operand| operand.name == "d8").unwrap();
    assert_eq!(d8.value.unwrap(), 0x0F);
    //TODO test 16 bit operand
}

#[test]
fn add_sets_right_flags() {
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0xFF);
    cpu.Registers.set_item("B", 0xFF);

    cpu.add_a_r("B");
    assert_eq!(cpu.Registers.get_item("A"), 254);   //should be rounded bitmasking with 0xFF
    assert_eq!(cpu.Registers.get_item("c"), 1);     //carry should be 1 since 0xFF + 0xFF > 0xFF
    assert_eq!(cpu.Registers.get_item("n"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 0);
    assert_eq!(cpu.Registers.get_item("h"), 1);


    cpu.Registers.set_item("A", 0x8);
    cpu.Registers.set_item("B", 0x8);
    cpu.add_a_r("B");
    assert_eq!(cpu.Registers.get_item("A"), 16);
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("n"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 0);
    assert_eq!(cpu.Registers.get_item("h"), 1); //should be setted, 8+8 > 0xF

    cpu.Registers.set_item("HL", 0xA000);   //should point to memory filled with 0x2
    cpu.Registers.set_item("A", 0x1);
    cpu.add_a_hl();
    assert_eq!(cpu.Registers.get_item("A"), 0x3);
}

#[test]
fn adc_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0xFF);
    cpu.Registers.set_item("B", 0xFF);
    cpu.adc_a_r("B");
    assert_eq!(cpu.Registers.get_item("A"), 254);

    cpu.Registers.set_item("B", 0x1);
    cpu.adc_a_r("B");
    assert_eq!(cpu.Registers.get_item("A"), 0); //should be (A (254) + B (1) + Carry (1)) && 0xFF == 256 && 0xFF == 0
    assert_eq!(cpu.Registers.get_item("z"), 1); //should be set since result is 0

    cpu.Registers.set_item("A", 0x1);
    cpu.Registers.set_item("c", 0x0);
    cpu.Registers.set_item("HL", 0xA000);   //should point to memory filled with 0x2
    cpu.adc_a_hl();
    assert_eq!(cpu.Registers.get_item("A"), 0x3);
}

#[test]
fn sub_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0x9);
    cpu.Registers.set_item("B", 0x2);
    cpu.sub_a_r("B");
    assert_eq!(cpu.Registers.get_item("A"), 7);

    cpu.Registers.set_item("A", 0xFF);
    cpu.Registers.set_item("B", 0xFF);
    cpu.sub_a_r("B");
    assert_eq!(cpu.Registers.get_item("A"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 1);
    assert_eq!(cpu.Registers.get_item("n"), 1); //should be set, operation is sub

    cpu.Registers.set_item("A", 0x0F);
    cpu.Registers.set_item("B", 0xFF);
    cpu.sub_a_r("B");
    assert_eq!(cpu.Registers.get_item("A"), 0x10); //should be set, B > A
    assert_eq!(cpu.Registers.get_item("c"), 1); //should be set, B > A

    cpu.Registers.set_item("A", 0x03);
    cpu.Registers.set_item("B", 0x0F);
    cpu.sub_a_r("B");
    assert_eq!(cpu.Registers.get_item("h"), 1); //should be set, (b & 0x0F) > (a & 0x0F)
}

#[test]
fn subc_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("c", 1);
    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x2);
    cpu.sbc_a_r("B");
    assert_eq!(cpu.Registers.get_item("A"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 1);

    cpu.Registers.set_item("c", 1);
    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x3);
    cpu.sbc_a_r("B");
    assert_eq!(cpu.Registers.get_item("h"), 1); //should be set, (b & 0x0F + carry) > (a & 0x0F)
}

#[test]
fn and_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x2);
    cpu.and_a_r("B");

    assert_eq!(cpu.Registers.get_item("A"), 2);
    assert_eq!(cpu.Registers.get_item("n"), 0);
    assert_eq!(cpu.Registers.get_item("h"), 1);
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 0);

    cpu.Registers.set_item("A", 0x0);
    cpu.Registers.set_item("B", 0x0);
    cpu.and_a_r("B");
    assert_eq!(cpu.Registers.get_item("z"), 1);
}

#[test]
fn or_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x2);
    cpu.or_a_r("B");

    assert_eq!(cpu.Registers.get_item("A"), 3);
    assert_eq!(cpu.Registers.get_item("n"), 0);
    assert_eq!(cpu.Registers.get_item("h"), 0);
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 0);

    cpu.Registers.set_item("A", 0x0);
    cpu.Registers.set_item("B", 0x0);
    cpu.or_a_r("B");
    assert_eq!(cpu.Registers.get_item("A"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 1);

    cpu.Registers.set_item("A", 0x2);
    cpu.Registers.set_item("B", 0x3);
    cpu.or_a_r("B");
    assert_eq!(cpu.Registers.get_item("A"), 3);
}

#[test]
fn xor_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x2);
    cpu.xor_a_r("B");

    assert_eq!(cpu.Registers.get_item("A"), 1);
    assert_eq!(cpu.Registers.get_item("n"), 0);
    assert_eq!(cpu.Registers.get_item("h"), 0);
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 0);

    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x3);
    cpu.xor_a_r("B");

    assert_eq!(cpu.Registers.get_item("A"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 1);
}

#[test]
fn cp_sets_right_flags(){
    //same as sub but we check that A didn't change
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0xFF);
    cpu.Registers.set_item("B", 0xFF);
    cpu.cp_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 0xFF);
    assert_eq!(cpu.Registers.get_item("z"), 1);
    assert_eq!(cpu.Registers.get_item("n"), 1); //should be set, operation is sub

    cpu.Registers.set_item("B", 0xFF);
    cpu.Registers.set_item("A", 0x0F);
    cpu.cp_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 0x0F);
    assert_eq!(cpu.Registers.get_item("c"), 1); //should be set, B > A

    cpu.Registers.set_item("B", 0x0F);
    cpu.Registers.set_item("A", 0x03);
    cpu.cp_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 0x03);
    assert_eq!(cpu.Registers.get_item("h"), 1); //should be set, (b & 0x0F) > (a & 0x0F)
}

#[test]
fn inc_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0xFF);

    let previous_c_value = cpu.Registers.get_item("c");
    cpu.inc("A");
    assert_eq!(cpu.Registers.get_item("c"), previous_c_value); //should not be affected
    assert_eq!(cpu.Registers.get_item("n"), 0); //should be resetted
    assert_eq!(cpu.Registers.get_item("z"), 1);
    assert_eq!(cpu.Registers.get_item("A"), 0);

    cpu.Registers.set_item("A", 0x0F);
    cpu.inc("A");
    assert_eq!(cpu.Registers.get_item("h"), 1);
    assert_eq!(cpu.Registers.get_item("A"), 0x10);
}


#[test]
fn dec_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0x1);

    cpu.dec("A");
    assert_eq!(cpu.Registers.get_item("n"), 1);
    assert_eq!(cpu.Registers.get_item("z"), 1);
    assert_eq!(cpu.Registers.get_item("A"), 0);

    cpu.Registers.set_item("A", 0x10);
    cpu.dec("A");
    assert_eq!(cpu.Registers.get_item("h"), 1);
    assert_eq!(cpu.Registers.get_item("A"), 0xF);

    cpu.Registers.set_item("A", 0x0);
    cpu.dec("A");
    assert_eq!(cpu.Registers.get_item("h"), 1);
    assert_eq!(cpu.Registers.get_item("A"), 0xFF);
}

#[test]
fn right_rotations_works(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0x03);

    cpu.rrca();
    assert_eq!(cpu.Registers.get_item("c"), 1);
    assert_eq!(cpu.Registers.get_item("A"), 0x81);

    cpu.Registers.set_item("A", 0x06);
    cpu.rrca();
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("A"), 0x03);

    cpu.Registers.set_item("A", 0x03);
    cpu.Registers.set_item("c", 1);
    cpu.rra();
    assert_eq!(cpu.Registers.get_item("c"), 1);
    assert_eq!(cpu.Registers.get_item("A"), 0x81);  //carry is putted on 7th position

    cpu.Registers.set_item("A", 0x04);
    cpu.Registers.set_item("c", 0);
    cpu.rra();
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("A"), 0x02);
}

#[test]
fn left_rotations_works(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0x81);
    cpu.Registers.set_item("c", 0);

    cpu.rlca();
    assert_eq!(cpu.Registers.get_item("c"), 1);
    assert_eq!(cpu.Registers.get_item("A"), 0x3);

    cpu.rla();
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("A"), 0x7);      //carry is putted in 0th position
}

#[test]
fn add_hl_nn_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("HL", 0xFFFF);
    cpu.Registers.set_item("BC", 0x0001);

    cpu.add_hl_n("BC");
    assert_eq!(cpu.Registers.get_item("HL"), 0x0000);
    assert_eq!(cpu.Registers.get_item("c"), 1);
    assert_eq!(cpu.Registers.get_item("h"), 1);


    cpu.Registers.set_item("HL", 0x0FFF);
    cpu.Registers.set_item("BC", 0x0001);

    cpu.add_hl_n("BC");
    assert_eq!(cpu.Registers.get_item("HL"), 0x1000);
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("h"), 1);
}

#[test]
fn memory_can_read_and_write(){
    let mut dummy_mmu = create_dummy_mmu();
    assert_eq!(dummy_mmu.read_byte(0x0), 0x31);
    dummy_mmu.write_byte(0x0, 0xFF);
    assert_eq!(dummy_mmu.read_byte(0x0), 0xFF);

    assert_eq!(dummy_mmu.read_byte(0x8000), 0x1);
    dummy_mmu.write_byte(0x8000, 0xFF);
    assert_eq!(dummy_mmu.read_byte(0x8000), 0xFF);

    assert_eq!(dummy_mmu.read_byte(0xA000), 0x2);
    dummy_mmu.write_byte(0xA000, 0xFF);
    assert_eq!(dummy_mmu.read_byte(0xA000), 0xFF);

    assert_eq!(dummy_mmu.read_byte(0xC000), 0x3);
    dummy_mmu.write_byte(0xC000, 0xFF);
    assert_eq!(dummy_mmu.read_byte(0xC000), 0xFF);

    assert_eq!(dummy_mmu.read_byte(0xFF00), 0x4);
    dummy_mmu.write_byte(0xFF00, 0xFF);
    assert_eq!(dummy_mmu.read_byte(0xFF00), 0xFF);

    assert_eq!(dummy_mmu.read_byte(0xFF80), 0x5);
    dummy_mmu.write_byte(0xFF80, 0xFF);
    assert_eq!(dummy_mmu.read_byte(0xFF80), 0xFF);

    assert_eq!(dummy_mmu.read_byte(0xFFFF), 1);
    dummy_mmu.write_byte(0xFFFF, 0x0);
    assert_eq!(dummy_mmu.read_byte(0xFFFF), 0x0);
}

#[test]
fn ld_hl_works(){
    let mut cpu = create_dummy_cpu();

    cpu.Registers.set_item("HL", 0xC000);
    cpu.Registers.set_item("B", 0x4);
    cpu.ld_address_value("HL","B");
    assert_eq!(cpu.MMU.read_byte(0xC000), 0x4);

    cpu.Registers.set_item("BC", 0xC001);
    cpu.Registers.set_item("A", 0x3);
    cpu.ld_address_value("BC", "A");
    assert_eq!(cpu.MMU.read_byte(0xC001), 0x3);
}

#[test]
fn inc_and_dec_hl_pointer_works(){
    let mut cpu = create_dummy_cpu();

    cpu.Registers.set_item("HL", 0xC000);
    cpu.MMU.write_byte(0xC000, 0x5);
    cpu.inc_hl_pointer();
    assert_eq!(cpu.MMU.read_byte(0xC000), 0x6);

    cpu.dec_hl_pointer();
    assert_eq!(cpu.MMU.read_byte(0xC000), 0x5);
}

#[test]
fn memory_pointer_ops_works(){
    let mut cpu = create_dummy_cpu();
    let ld_hl_pointer_0xF_instruction = Instruction {
        opcode: 0,
        immediate: false,
        operands: vec![
            Operand {
                immediate: false,
                name: "d8".to_string(),
                bytes: None,
                value: Some(0xF),
                adjust: None
            }
        ],
        cycles: vec![],
        bytes: 0,
        mnemonic: "".to_string(),
        comment: None,
        prefixed: false
    };

    cpu.Registers.set_item("HL", 0xC000);
    cpu.ld_hl_pointer_d8(ld_hl_pointer_0xF_instruction);
    assert_eq!(cpu.MMU.read_byte(0xC000), 0xF);

    cpu.Registers.set_item("HL", 0xC000);
    cpu.Registers.set_item("A", 3);
    cpu.ld_hl_pointer_dec_inc_a(true);
    assert_eq!(cpu.MMU.read_byte(0xC000), 0x3);
    assert_eq!(cpu.Registers.get_item("HL"), 0xC001);
}

#[test]
fn push_and_pop_works(){
    let mut cpu = create_dummy_cpu();

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

