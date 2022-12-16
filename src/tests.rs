use super::*;

fn test_little_big_endian(){
    let hex_value = u32::from_str_radix("C0FFEE", 16).unwrap();
    for to_le_byte in hex_value.to_le_bytes().into_iter() {
        println!("{:#04x}", to_le_byte);
    }

    for to_be_byte in hex_value.to_be_bytes().into_iter() {
        println!("{:#04x}", to_be_byte);
    }
}

fn create_dummy_cpu() -> CPU {
    let Cartridge: Cartridge = read_cartridge("image.gb");
    let Decoder: Decoder = Decoder::new(Cartridge);
    CPU::new(Some(Decoder))
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
    cpu.add_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 254);

    cpu.Registers.set_item("B", 0x1);
    cpu.adc_a("B");
    assert_eq!(cpu.Registers.get_item("A"), 0); //should be (A (254) + B (1) + Carry (1)) && 0xFF = 256 && 0xFF == 0
    assert_eq!(cpu.Registers.get_item("z"), 1); //should be set since result is 0
}