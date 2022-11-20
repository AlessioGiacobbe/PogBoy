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