extern crate core;

mod op_codes_parser;
mod cartridge;
mod decoder;
mod cpu;

use std::{fmt, fs};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::{File};
use std::io::{Read};
use serde_json::{Value};
use crate::cartridge::cartridge::{Cartridge, CartridgeInfo, read_cartridge};
use crate::decoder::decoder::{Decoder};
use crate::op_codes_parser::op_codes_parser::{Instruction};
use crate::op_codes_parser::op_codes_parser::get_instructions_from_json;
use crate::cpu::CPU::CPU;

fn main() {
    let Cartridge: Cartridge = read_cartridge("snake.gb");
    let Decoder: Decoder = Decoder::new(Cartridge);
    let cpu: CPU = CPU::new(Decoder);
}
