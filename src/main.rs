extern crate core;

mod op_codes_parser;
mod cartridge;

use std::{fmt, fs, usize};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{Read};
use serde_json::{Value};
use crate::cartridge::cartridge::{CartridgeInfo, read_cartridge};
use crate::op_codes_parser::op_codes_parser::{AdjustTypes, Instruction};
use crate::op_codes_parser::op_codes_parser::get_instructions_from_json;

#[derive(Debug)]
struct Decoder {
    data: Vec<u8>,
    address: u32,
    unprefixed_op_codes: HashMap<u8, Instruction>,
    prefixed_op_codes: HashMap<u8, Instruction>,
}


fn main() {
    let op_codes_content = fs::read_to_string("./src/opcodes.json").expect("error reading file");
    let json_op_codes: Value = serde_json::from_str(&op_codes_content).unwrap();

    let unprefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
    let prefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
    let cartridge_header = read_cartridge("snake.gb");  //TODO should be parameter

    println!("{}", read_cartridge("snake.gb"));

    println!("{:?}", unprefixed_op_codes[&0]);

    let mut rom = File::open(format!("./src/{}", "snake.gb")).expect("rom not found");

    let mut rom_buffer: Vec<u8> = Vec::new();

    let decoder: Decoder = Decoder {
        data: rom_buffer,
        address: 0,
        unprefixed_op_codes,
        prefixed_op_codes
    };

    //println!("{:?}", decoder);
}
