extern crate core;

mod op_codes_parser;
mod cartridge;

use std::{fmt, fs, usize};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::{File, read};
use std::io::{Cursor, Read};
use bincode::config::BigEndian;
use serde_json::{Value};
use crate::cartridge::cartridge::{CartridgeInfo, read_cartridge};
use crate::op_codes_parser::op_codes_parser::{AdjustTypes, Instruction};
use crate::op_codes_parser::op_codes_parser::get_instructions_from_json;
use byteorder::{BigEndian as byteorderbigendian, ReadBytesExt};

const INSTRUCTIONS_PREFIX: u8 = 203; //0xCB

#[derive(Debug)]
struct Decoder {
    data: Vec<u8>,
    address: u32,
    unprefixed_op_codes: HashMap<u8, Instruction>,
    prefixed_op_codes: HashMap<u8, Instruction>,
}

impl Decoder {

    fn decode(&self, mut address: i32) {
        let mut op_code = Self::read(&self, address, 1);
        address = address + 1;
        let instruction = {
            if op_code[0] == INSTRUCTIONS_PREFIX {
                op_code = Self::read(&self, address, 1);
                address = address + 1;
                self.prefixed_op_codes.get(&op_code[0])
            }else{
                self.unprefixed_op_codes.get(&op_code[0])
            }
        };
    }

    fn read(&self, address: i32, count: u8) -> &[u8] {
        let end_address = address + i32::from(count);
        if end_address >= 0 && end_address <= self.data.len() as i32 {
            &self.data[address as usize..end_address as usize]
        }else{
            panic!("{} address out of bound!", self.data.len() as i32)
        }
    }
}


fn main() {
    let op_codes_content = fs::read_to_string("./src/opcodes.json").expect("error reading file");
    let json_op_codes: Value = serde_json::from_str(&op_codes_content).unwrap();

    let unprefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
    let prefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
    let cartridge_header = read_cartridge("snake.gb");  //TODO should be parameter

    let mut rom = File::open(format!("./src/{}", "snake.gb")).expect("rom not found");
    let mut rom_buffer: Vec<u8> = Vec::new();
    rom.read_to_end(&mut rom_buffer);


    let decoder: Decoder = Decoder {
        data: rom_buffer,
        address: 0,
        unprefixed_op_codes,
        prefixed_op_codes
    };

    println!("{:?}", decoder.read(64, 2));
    println!("{:?}", decoder.decode(64));
    println!("{:?}", decoder.decode(63));
}
