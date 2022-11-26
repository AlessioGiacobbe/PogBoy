extern crate core;

mod op_codes_parser;
mod cartridge;
mod disassembler;

use std::{fmt, fs, usize};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::{File, read};
use std::io::{Cursor, Read};
use bincode::config::{BigEndian, LittleEndian};
use serde_json::{Value};
use crate::disassembler::disassembler::{Disassembler};
use crate::cartridge::cartridge::{CartridgeInfo, read_cartridge};
use crate::op_codes_parser::op_codes_parser::{AdjustTypes, Instruction, Operand, OperandValue};
use crate::op_codes_parser::op_codes_parser::get_instructions_from_json;
use byteorder::{BigEndian as byteorderBigEndian, LittleEndian as byteorderLittleEndian, ReadBytesExt};


fn main() {
    let op_codes_content = fs::read_to_string("./src/opcodes.json").expect("error reading file");
    let json_op_codes: Value = serde_json::from_str(&op_codes_content).unwrap();

    let unprefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
    let prefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
    let cartridge_header = read_cartridge("snake.gb");  //TODO should be parameter

    let mut rom = File::open(format!("./src/{}", "snake.gb")).expect("rom not found");
    let mut rom_buffer: Vec<u8> = Vec::new();
    rom.read_to_end(&mut rom_buffer);


    let decoder: Disassembler = Disassembler {
        data: rom_buffer,
        address: 0,
        unprefixed_op_codes,
        prefixed_op_codes
    };

    println!("{:?}", decoder.read(359, 2));
    println!("{:?}", decoder.decode(359).1);
    println!("{}", decoder.decode(359).1);

    decoder.disassemble(336, 16)
}
