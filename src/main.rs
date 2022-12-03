extern crate core;

mod op_codes_parser;
mod cartridge;
mod disassembler;

use std::{fmt, fs};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::{File};
use std::io::{Read};
use serde_json::{Value};
use phf::phf_map;
use crate::cartridge::cartridge::{CartridgeInfo, read_cartridge};
use crate::disassembler::disassembler::{Disassembler};
use crate::op_codes_parser::op_codes_parser::{Instruction};
use crate::op_codes_parser::op_codes_parser::get_instructions_from_json;


struct Registers {
    AF: u16,
    BC: u16,
    DE: u16,
    HL: u16,
    PC: u16,
    SP: u16,
    LOW_REGISTERS: phf::Map<&'static str, &'static str>,
    HIGH_REGISTERS: phf::Map<&'static str, &'static str>,
    REGISTERS: [&'static str; 6],
    FLAGS: phf::Map<&'static str, u8>
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "AF : {} BC : {} DE : {} HL : {} PC : {} SP : {:b}", self.AF, self.BC, self.DE, self.HL, self.PC, self.SP)
    }
}

#[derive(Debug, Clone)]
pub enum RegistersValue {
    U8(u8),
    U16(u16)
}

const LOW_REGISTERS: phf::Map<&'static str, &'static str> = phf_map! {
         "F" => "AF",
         "C" => "BC",
         "E" => "DE",
         "L" => "HL"
};

    const HIGH_REGISTERS: phf::Map<&'static str, &'static str> = phf_map! {
         "A" => "AF",
         "B" => "BC",
         "D" => "DE",
         "H" => "HL"
    };

const REGISTERS: [&'static str; 6] = ["AF", "BC", "DE", "HL", "PC", "SP"];

const FLAGS: phf::Map<&'static str, u8> = phf_map! {
        "c" => 4,
        "h" => 5,
        "n" => 6,
        "z" => 7
};

impl Registers {
    fn get_register_value_from_name(&self, name: &str) -> u16 {
        return match name {
            "AF" => self.AF,
            "BC" => self.BC,
            "DE" => self.DE,
            "HL" => self.HL,
            "PC" => self.PC,
            "SP" => self.SP,
            _ => panic!("register {} does not exists", name)
        };
    }

    fn set_register_value_from_name(&mut self, name: &str, value: u16) {
        match name {
            "AF" => self.AF = value,
            "BC" => self.BC = value,
            "DE" => self.DE = value,
            "HL" => self.HL = value,
            "PC" => self.PC = value,
            "SP" => self.SP = value,
            _ => panic!("register {} does not exists", name)
        };
    }

    fn get_item(&self, item: &str) -> u16 {
        if LOW_REGISTERS.contains_key(item) {
            let register_name = LOW_REGISTERS[item];
            let register_value = self.get_register_value_from_name(register_name);
            return register_value & 255 // bitmask with 0xFF, get lower 8 bits
        }
        if HIGH_REGISTERS.contains_key(item) {
            let register_name = HIGH_REGISTERS[item];
            let register_value = self.get_register_value_from_name(register_name);
            return register_value >> 8; // shift right by 8 will get only the higher bits
        }
        if FLAGS.contains_key(item) {
            let bit_position = FLAGS[item];
            return self.AF >> bit_position & 1; // to get the bit at x position, shift right AF by x positions and get the last bit
        }
        if REGISTERS.contains(&item) {
            return self.get_register_value_from_name(item);
        }
        panic!("item {} not fonud", item);
    }

    fn set_item(&mut self, item: &str, value: u16) {
        if LOW_REGISTERS.contains_key(item) {
            let register_name = LOW_REGISTERS[item];
            let register_value = self.get_register_value_from_name(register_name);
            let updated_register_value = (register_value & 65280) | value;  // clear last 8 bits masking with 0xFF00 then OR with passed value
            self.set_register_value_from_name(register_name, updated_register_value);
            return;
        }
        if HIGH_REGISTERS.contains_key(item) {
            let register_name = HIGH_REGISTERS[item];
            let register_value = self.get_register_value_from_name(register_name);
            let updated_register_value = (register_value & 255) | value << 8;  // clear first 8 bits masking with 0x00FF then OR with passed value shifted to position
            self.set_register_value_from_name(register_name, updated_register_value);
            return;
        }
        if FLAGS.contains_key(item) {
            if value != 0 && value != 1 {
                panic!("invalid value {} to set registry flag", value);
            }
            let mut register_value = self.get_register_value_from_name("AF");
            let bit_position = FLAGS[item];
            if value == 1 {
                register_value |= (1 << bit_position);  // set flag at x position by OR-ing with 1 shifted by x positions
            }else{
                register_value &= !(1 << bit_position); // unset flag at x position by NAND (& !something) with 1 shifted by x positions
            }
            self.set_register_value_from_name("AF", register_value);
            return;
        }
        if REGISTERS.contains(&item) {
            self.set_register_value_from_name(item, value);
            return
        }
        panic!("item {} not fonud", item);
    }
}


fn main() {
    let op_codes_content = fs::read_to_string("./src/opcodes.json").expect("error reading file");
    let json_op_codes: Value = serde_json::from_str(&op_codes_content).unwrap();

    let unprefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
    let prefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
    let cartridge_header = read_cartridge("snake.gb");  //TODO should be parameter

    println!("{}", cartridge_header);

    //TODO should disassembler should accept Cartridge struct
    let mut rom = File::open(format!("./src/{}", "snake.gb")).expect("rom not found");
    let mut rom_buffer: Vec<u8> = Vec::new();
    rom.read_to_end(&mut rom_buffer).expect("can't read ROM");


    let decoder: Disassembler = Disassembler {
        data: rom_buffer,
        address: 0,
        unprefixed_op_codes,
        prefixed_op_codes
    };

    println!("{:?}", decoder.read(359, 2));
    println!("{:?}", decoder.decode(359).1);
    println!("{}", decoder.decode(359).1);

    decoder.disassemble(336, 16);

    let mut registers = Registers {
        AF: u16::from_str_radix("CEAF",   16).unwrap(),
        BC: u16::from_str_radix("BEAF",   16).unwrap(),
        DE: u16::from_str_radix("AFCE",   16).unwrap(),
        HL: u16::from_str_radix("AFAF",   16).unwrap(),
        PC: u16::from_str_radix("AFAF",   16).unwrap(),
        SP: u16::from_str_radix("AFAF",   16).unwrap(),
        LOW_REGISTERS,
        HIGH_REGISTERS,
        REGISTERS,
        FLAGS
    };

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
}
