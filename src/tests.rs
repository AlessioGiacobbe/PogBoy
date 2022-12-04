fn test_little_big_endian(){
    let hex_value = u32::from_str_radix("C0FFEE", 16).unwrap();
    for to_le_byte in hex_value.to_le_bytes().into_iter() {
        println!("{:#04x}", to_le_byte);
    }

    println!("---");

    for to_be_byte in hex_value.to_be_bytes().into_iter() {
        println!("{:#04x}", to_be_byte);
    }
}

/*
TODO create real tests
            registers.set_item("F", 8);
            println!("{:#01x}", registers.get_item("AF"));

            registers.set_item("A", 8);
            println!("{:#01x}", registers.get_item("AF"));

            registers.set_item("AF", 44975);
            println!("{:#01x}", registers.get_item("AF"));


            println!("{:#01x}", registers.get_item("B"));
            println!("{:#01x}", registers.get_item("D"));
            println!("{:#01x}", registers.get_item("c"));
            println!("{:#01x}", registers.get_item("BC"));
            println!("{:#01x}", registers.get_item("AF"));

            registers.set_item("AF", 0);
            registers.set_item("c", 1);
            registers.set_item("h", 1);
            registers.set_item("n", 1);
            registers.set_item("z", 1);
            registers.set_item("AF", 65535);
            registers.set_item("c", 0);
            registers.set_item("h", 0);
            registers.set_item("n", 0);
            registers.set_item("z", 0);


    println!("{:?}", decoder.read(359, 2));
    println!("{:?}", decoder.decode(359).1);
    println!("{}", decoder.decode(359).1);
 */