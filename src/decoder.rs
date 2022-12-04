pub mod decoder {
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::Read;
    use byteorder::{LittleEndian as byteorderLittleEndian, ReadBytesExt};
    use serde_json::Value;
    use crate::cartridge::cartridge::read_cartridge;
    use crate::op_codes_parser::op_codes_parser::{get_instructions_from_json, Instruction, Operand, OperandValue};

    const INSTRUCTIONS_PREFIX: u8 = 203; //0xCB

    #[derive(Debug)]
    pub struct Decoder {
        pub(crate) data: Vec<u8>,
        pub(crate) address: u32,
        pub(crate) unprefixed_op_codes: HashMap<u8, Instruction>,
        pub(crate) prefixed_op_codes: HashMap<u8, Instruction>,
    }

    impl Decoder {

        pub(crate) fn new() -> Decoder {
            let op_codes_content = fs::read_to_string("./src/opcodes.json").expect("error reading file");
            let json_op_codes: Value = serde_json::from_str(&op_codes_content).unwrap();

            let unprefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");
            let prefixed_op_codes: HashMap<u8, Instruction> = get_instructions_from_json(&json_op_codes,"unprefixed");

            let mut rom = File::open(format!("./src/{}", "snake.gb")).expect("rom not found");
            let mut rom_buffer: Vec<u8> = Vec::new();
            rom.read_to_end(&mut rom_buffer).expect("can't read ROM");


            Decoder {
                data: rom_buffer,
                address: 0,
                unprefixed_op_codes,
                prefixed_op_codes
            }
        }

        pub(crate) fn read(&self, address: i32, count: u8) -> &[u8] {
            let end_address = address + i32::from(count);
            if end_address >= 0 && end_address <= self.data.len() as i32 {
                &self.data[address as usize..end_address as usize]
            }else{
                panic!("{} address out of bound!", self.data.len() as i32)
            }
        }

        pub(crate) fn decode(&self, mut address: i32) -> (i32, Instruction) {
            let mut op_code = Self::read(&self, address, 1);
            address = address + 1;
            let instruction = {
                if op_code[0] == INSTRUCTIONS_PREFIX {
                    op_code = Self::read(&self, address, 1);
                    address = address + 1;
                    self.prefixed_op_codes.get(&op_code[0]).unwrap()
                }else{
                    self.unprefixed_op_codes.get(&op_code[0]).unwrap()
                }
            };

            let new_operands: Vec<Operand> = {
                let mut new_operands: Vec<Operand> = vec![];
                for operand in instruction.operands.iter() {
                    if operand.bytes != None {
                        let bytes = operand.bytes.unwrap();
                        let mut operand_value = Self::read(&self, address, bytes);
                        address = address + i32::from(bytes);
                        let mut operand_to_be_pushed = operand.clone();
                        let operand_value: OperandValue = match bytes {
                            1 => OperandValue::U8(operand_value.read_u8().unwrap()),
                            2 => OperandValue::U16(operand_value.read_u16::<byteorderLittleEndian>().unwrap()),
                            _ => panic!("no operand value")
                        };
                        operand_to_be_pushed.value = Some(operand_value);
                        new_operands.push(operand_to_be_pushed);
                    }else{
                        new_operands.push(operand.clone());
                    }
                };
                new_operands
            };

            let mut decoded_instruction = (*instruction).clone();
            decoded_instruction.operands = new_operands;
            (address, decoded_instruction)
        }

        pub(crate) fn disassemble(&self, mut address: i32, quantity: i32){
            for _ in 0..quantity{
                let (new_address, instruction) = self.decode(address);
                println!("{:#04X}       {}", address, instruction);
                address = new_address;
            }
        }
    }

}