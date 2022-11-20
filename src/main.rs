mod op_codes_parser;
mod cartridge;

use std::{fmt, fs, usize};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{Read};
use serde_json::{Value};
use crate::cartridge::cartridge::{CartridgeInfo, read_cartridge};
use crate::op_codes_parser::op_codes_parser::Instruction;
use crate::op_codes_parser::op_codes_parser::get_instructions_from_json;




fn main() {
    let op_codes_content = fs::read_to_string("./src/opcodes.json").expect("error reading file");
    let json_op_codes: Value = serde_json::from_str(&op_codes_content).unwrap();

    let unprefixed_op_codes: HashMap<String, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
    let prefixed_op_codes: HashMap<String, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
    let cartridge_header = read_cartridge("snake.gb");  //TODO should be parameter

    println!("{}", read_cartridge("snake.gb"));

    println!("{:?}", unprefixed_op_codes["0x00"]);
}
