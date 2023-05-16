pub mod op_codes_parser {
    use std::{fmt, u8};
    use std::collections::HashMap;
    use std::fmt::{Formatter};

    use serde_json::Value;

    #[derive(Debug, Clone)]
    pub enum AdjustTypes {
        POSITIVE,
        NEGATIVE
    }

    #[derive(Debug, Clone)]
    pub struct Operand {
        pub immediate: bool,
        pub name: String,
        pub bytes: Option<u8>,
        pub value: Option<u16>,
        pub adjust: Option<AdjustTypes>
    }

    #[derive(Debug, Clone)]
    pub struct Instruction {
        pub opcode: u8,
        pub immediate: bool,
        pub operands: Vec<Operand>,
        pub cycles: Vec<u32>,
        pub bytes: u8,
        pub mnemonic: String,
        pub comment: Option<&'static str>,
        pub prefixed: bool
    }

    impl fmt::Display for AdjustTypes {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "")
        }
    }


    impl fmt::Display for Operand {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let mut value_as_string= self.name.clone();
            match self.value {
                None => {}
                Some(_) => {
                    let byte_is_none = self.bytes != None;
                    let unwrapped_value = self.value.clone().unwrap();
                    if byte_is_none {
                        value_as_string = format!("{:#04X}", unwrapped_value)
                    } else {
                        value_as_string = format!("{}", unwrapped_value)
                    }
                }
            }


            let adjust_value = match self.adjust.clone() {
                None => {
                    ""
                }
                Some(AdjustType) => {
                    match AdjustType {
                        AdjustTypes::POSITIVE => { "+" }
                        AdjustTypes::NEGATIVE => { "-" }
                    }
                }
            };

            if self.immediate {
                write!(f, "{}{}", value_as_string, adjust_value)
            }else{
                write!(f, "({}{})", value_as_string, adjust_value)
            }
        }
    }

    impl fmt::Display for Instruction {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{: <6}", self.mnemonic)?;

            for (pos, operand) in self.operands.iter().enumerate() {
                if pos == self.operands.len()-1 {
                    write!(f, "{}", operand)?;
                }else{
                    write!(f, "{}, ", operand)?;
                }
            }

            if self.comment != None {
                write!(f, " ; {: <4}", self.comment.unwrap())?;
            }
            write!(f, "")
        }
    }


    pub fn get_instructions_from_json(json_op_codes: &Value, category: &str) -> HashMap<u8, Instruction> {
        json_op_codes[category].as_object().unwrap().into_iter().map(|op_object| {
            let op_code: String = op_object.0.to_owned();
            let op_info = op_object.1.as_object().unwrap();

            let op_code_without_prefix = op_code.trim_start_matches("0x");
            let op_code_as_int = u8::from_str_radix(op_code_without_prefix, 16).expect("invalid hex");


            let collection_of_op: Vec<Operand> = op_info["operands"].as_array().unwrap().into_iter().map(|operand| {
                let operand_object = operand.as_object().unwrap();
                let is_increment = operand_object.contains_key("increment");
                let is_decrement = operand_object.contains_key("decrement");
                Operand {
                    immediate: operand_object["immediate"].as_bool().expect("operand should be boolean"),
                    name: operand_object["name"].as_str().expect("operand should be string").parse().unwrap(),
                    bytes: if operand_object.contains_key("bytes") {
                        Some(operand_object["bytes"].as_i64().expect("invalid number").to_le_bytes()[0])
                    } else {
                        None
                    },
                    value: None,
                    adjust: if is_increment { Some(AdjustTypes::POSITIVE) } else if is_decrement { Some(AdjustTypes::NEGATIVE) } else { None }
                }
            }).collect::<Vec<Operand>>();

            let op_code_as_integer = u8::from_str_radix(op_code.trim_start_matches("0x"), 16).expect("invalid hex value");

            let cycles = {
                let mut cycles = vec![];
                for cycle in op_info["cycles"].as_array().unwrap() {
                    cycles.push(cycle.as_i64().unwrap().to_le_bytes()[0] as u32)
                };
                cycles
            };


            let instruction = Instruction {
                opcode: op_code_as_integer,
                immediate: op_info["immediate"].as_bool().expect("invalid bool"),
                operands: collection_of_op,
                cycles,
                bytes: op_info["bytes"].as_i64().expect("invalid number").to_le_bytes()[0],
                mnemonic: op_info["mnemonic"].as_str().expect("invalid string").parse().unwrap(),
                comment: None,
                prefixed: category == "cbprefixed"
            };
            (op_code_as_int, instruction)
        }).collect()
    }

}