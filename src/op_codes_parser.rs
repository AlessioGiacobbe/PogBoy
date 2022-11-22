pub mod op_codes_parser {
    use std::collections::HashMap;
    use std::{fmt, u8};
    use std::fmt::Formatter;
    use serde_json::Value;


    #[derive(Debug)]
    pub enum AdjustTypes {
        POSITIVE,
        NEGATIVE
    }

    #[derive(Debug)]
    pub struct Operand {
        immediate: bool,
        name: String,
        bytes: u8,
        value: Option<u8>,
        adjust: Option<AdjustTypes>
    }

    #[derive(Debug)]
    pub struct Instruction {
        opcode: u8,
        immediate: bool,
        operands: Vec<Operand>,
        cycles: Vec<u8>,
        bytes: u8,
        mnemonic: String,
        comment: &'static str
    }

    impl fmt::Display for AdjustTypes {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "")
        }
    }

    impl fmt::Display for Operand {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{name} ha {bytes} bytes", name = self.name, bytes = self.bytes)
        }
    }


    pub fn get_instructions_from_json(json_op_codes: &Value, category: &str) -> HashMap<u8, Instruction> {
        json_op_codes[category].as_object().unwrap().into_iter().map(|op_object| {
            let op_code: String = op_object.0.to_owned();
            let op_info = op_object.1.as_object().unwrap();

            let op_code_without_prefix = op_code.trim_start_matches("0x");
            let op_code_as_int =     u8::from_str_radix(op_code_without_prefix, 16).expect("invalid hex");


            let collection_of_op: Vec<Operand> = op_info["operands"].as_array().unwrap().into_iter().map(|operand| {
                let operand_object = operand.as_object().unwrap();
                Operand {
                    immediate: operand_object["immediate"].as_bool().expect("operand should be boolean"),
                    name: operand_object["name"].as_str().expect("operand should be string").parse().unwrap(),
                    bytes: op_info["bytes"].as_i64().expect("invalid number").to_le_bytes()[0],
                    value: None,
                    adjust: None
                }
            }).collect::<Vec<Operand>>();

            let op_code_as_integer = u8::from_str_radix(op_code.trim_start_matches("0x"), 16).expect("invalid hex value");

            println!("{:?}", op_info);

            let instruction = Instruction {
                opcode: op_code_as_integer,
                immediate: op_info["immediate"].as_bool().expect("invalid bool"),
                operands: collection_of_op,
                cycles: vec![],
                bytes: op_info["bytes"].as_i64().expect("invalid number").to_le_bytes()[0],
                mnemonic: op_info["mnemonic"].as_str().expect("invalid string").parse().unwrap(),
                comment: ""
            };
            (op_code_as_int, instruction)
        }).collect()
    }

}