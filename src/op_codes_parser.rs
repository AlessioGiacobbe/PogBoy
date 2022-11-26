pub mod op_codes_parser {
    use std::collections::HashMap;
    use std::{fmt, u8};
    use std::fmt::{Formatter, write};
    use serde_json::Value;


    #[derive(Debug, Clone)]
    pub enum AdjustTypes {
        POSITIVE,
        NEGATIVE
    }

    #[derive(Debug, Clone)]
    pub enum OperandValue {
        u8(u8),
        u16(u16)
    }

    #[derive(Debug, Clone)]
    pub struct Operand {
        immediate: bool,
        name: String,
        pub(crate) bytes: Option<u8>,
        pub(crate) value: Option<OperandValue>,
        adjust: Option<AdjustTypes>
    }

    #[derive(Debug, Clone)]
    pub struct Instruction {
        opcode: u8,
        immediate: bool,
        pub(crate) operands: Vec<Operand>,
        cycles: Vec<u8>,
        bytes: u8,
        mnemonic: String,
        comment: Option<&'static str>
    }

    impl fmt::Display for AdjustTypes {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "")
        }
    }


    impl fmt::Display for OperandValue {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match *self {
                OperandValue::u8(value) => {
                    write!(f, "{:?}", value)
                }
                OperandValue::u16(value) => {
                    write!(f, "{:?}", value)
                }
            }
        }
    }

    impl fmt::UpperHex for OperandValue {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match *self {
                OperandValue::u8(value) => {
                    write!(f, "{:#02X}", value)
                }
                OperandValue::u16(value) => {
                    write!(f, "{:#04X}", value)
                }
            }
        }
    }

    //TODO could be cleaner
    impl fmt::Display for Operand {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let mut valueAsString;
            match self.value {
                None => {
                    valueAsString = self.name.clone();
                }
                Some(_) => {
                    valueAsString = self.name.clone();
                    let byteIsNone = self.bytes != None;
                    let unwrappedValue = self.value.clone().unwrap();
                    match unwrappedValue {
                        OperandValue::u8(_) => {
                            if byteIsNone {
                                valueAsString = format!("{:#04X}", unwrappedValue)
                            } else {
                                valueAsString = format!("{}", unwrappedValue)
                            }
                        }
                        OperandValue::u16(_) => {
                            if byteIsNone {
                                valueAsString = format!("{:#04X}", unwrappedValue)
                            } else {
                                valueAsString = format!("{}", unwrappedValue)
                            }
                        }
                    }
                }
            }

            if self.immediate {
                write!(f, "{}", valueAsString)
            }else{
                write!(f, "({})", valueAsString)
            }
        }
    }

    //TODO could be cleaner
    impl fmt::Display for Instruction {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
                Operand {
                    immediate: operand_object["immediate"].as_bool().expect("operand should be boolean"),
                    name: operand_object["name"].as_str().expect("operand should be string").parse().unwrap(),
                    bytes: if operand_object.contains_key("bytes") {
                        Some(operand_object["bytes"].as_i64().expect("invalid number").to_le_bytes()[0])
                    } else {
                        None
                    },
                    value: None,
                    adjust: None
                }
            }).collect::<Vec<Operand>>();

            let op_code_as_integer = u8::from_str_radix(op_code.trim_start_matches("0x"), 16).expect("invalid hex value");

            let cycles = {
                let mut cycles = vec![];
                for cycle in op_info["cycles"].as_array().unwrap() {
                    cycles.push(cycle.as_i64().unwrap().to_le_bytes()[0])
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
                comment: None
            };
            (op_code_as_int, instruction)
        }).collect()
    }

}