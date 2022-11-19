mod op_codes_parser;

use std::{fs};
use std::collections::HashMap;
use serde_json::{Value};
use crate::op_codes_parser::op_codes_parser::Instruction;
use crate::op_codes_parser::op_codes_parser::get_instructions_from_json;

fn main() {
    let op_codes_content = fs::read_to_string("./src/opcodes.json").expect("LogRocket: error reading file");
    let json_op_codes: Value = serde_json::from_str(&op_codes_content).unwrap();

    let unprefixed_op_codes: HashMap<String, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
    let prefixed_op_codes: HashMap<String, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");

    println!("{:?}", unprefixed_op_codes["0x00"]);
    println!("{:?}", prefixed_op_codes["0xFF"]);
}
