use crate::op_codes_parser::op_codes_parser::Instruction;
use super::*;

fn create_dummy_coder() -> Decoder {
    let Cartridge: Cartridge = read_cartridge("image.gb");
    let dummy_cartridge: Cartridge = Cartridge {
        cartridge_info: Cartridge.cartridge_info,
        data_buffer: vec![0x00, 0x3E, 0x0F],    //NOP - LD A,0x0F
    };
    Decoder::new(dummy_cartridge)
}

fn create_dummy_cpu() -> CPU {
    let dummy_decoder = create_dummy_coder();
    CPU::new(Some(dummy_decoder))
}

#[test]
fn decoder_can_parse_correctly(){
    let dummy_decoder = create_dummy_coder();
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

    cpu.add_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 254);   //should be rounded bitmasking with 0xFF
    assert_eq!(cpu.Registers.get_item("c"), 1);     //carry should be 1 since 0xFF + 0xFF > 0xFF
    assert_eq!(cpu.Registers.get_item("n"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 0);
    assert_eq!(cpu.Registers.get_item("h"), 1);


    cpu.Registers.set_item("A", 0x8);
    cpu.Registers.set_item("B", 0x8);
    cpu.add_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 16);
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("n"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 0);
    assert_eq!(cpu.Registers.get_item("h"), 1); //should be setted, 8+8 > 0xF
}

#[test]
fn adc_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0xFF);
    cpu.Registers.set_item("B", 0xFF);
    cpu.adc_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 254);

    cpu.Registers.set_item("B", 0x1);
    cpu.adc_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 0); //should be (A (254) + B (1) + Carry (1)) && 0xFF == 256 && 0xFF == 0
    assert_eq!(cpu.Registers.get_item("z"), 1); //should be set since result is 0
}

#[test]
fn sub_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0x9);
    cpu.Registers.set_item("B", 0x2);
    cpu.sub_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 7);

    cpu.Registers.set_item("A", 0xFF);
    cpu.Registers.set_item("B", 0xFF);
    cpu.sub_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 1);
    assert_eq!(cpu.Registers.get_item("n"), 1); //should be set, operation is sub

    cpu.Registers.set_item("B", 0xFF);
    cpu.Registers.set_item("A", 0x0F);
    cpu.sub_a("B");
    assert_eq!(cpu.Registers.get_item("c"), 1); //should be set, B > A

    cpu.Registers.set_item("B", 0x0F);
    cpu.Registers.set_item("A", 0x03);
    cpu.sub_a("B");
    assert_eq!(cpu.Registers.get_item("h"), 1); //should be set, (b & 0x0F) > (a & 0x0F)
}

#[test]
fn subc_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("c", 1);
    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x2);
    cpu.sbc_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 1);

    cpu.Registers.set_item("c", 1);
    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x3);
    cpu.sbc_a("B");
    assert_eq!(cpu.Registers.get_item("h"), 1); //should be set, (b & 0x0F + carry) > (a & 0x0F)
}

#[test]
fn and_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x2);
    cpu.and_a("B");

    assert_eq!(cpu.Registers.get_item("A"), 2);
    assert_eq!(cpu.Registers.get_item("n"), 0);
    assert_eq!(cpu.Registers.get_item("h"), 1);
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 0);

    cpu.Registers.set_item("A", 0x0);
    cpu.Registers.set_item("B", 0x0);
    cpu.and_a("B");
    assert_eq!(cpu.Registers.get_item("z"), 1);
}

#[test]
fn or_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x2);
    cpu.or_a("B");

    assert_eq!(cpu.Registers.get_item("A"), 3);
    assert_eq!(cpu.Registers.get_item("n"), 0);
    assert_eq!(cpu.Registers.get_item("h"), 0);
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 0);

    cpu.Registers.set_item("A", 0x0);
    cpu.Registers.set_item("B", 0x0);
    cpu.or_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 1);

    cpu.Registers.set_item("A", 0x2);
    cpu.Registers.set_item("B", 0x3);
    cpu.or_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 3);
}

#[test]
fn xor_sets_right_flags(){
    let mut cpu = create_dummy_cpu();
    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x2);
    cpu.xor_a("B");

    assert_eq!(cpu.Registers.get_item("A"), 1);
    assert_eq!(cpu.Registers.get_item("n"), 0);
    assert_eq!(cpu.Registers.get_item("h"), 0);
    assert_eq!(cpu.Registers.get_item("c"), 0);
    assert_eq!(cpu.Registers.get_item("z"), 0);

    cpu.Registers.set_item("A", 0x3);
    cpu.Registers.set_item("B", 0x3);
    cpu.xor_a("B");

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