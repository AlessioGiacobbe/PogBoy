pub mod Registers {
    use std::fmt;
    use std::default;
    use std::fmt::Formatter;
    use phf::phf_map;

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
        "z" => 7,    //ZERO
        "n" => 6,   //ADD/SUB
        "h" => 5,   //HALF CARRY
        "c" => 4,   //CARRY
    };

    pub struct Registers {
        pub(crate) AF: u16,
        pub(crate) BC: u16,
        pub(crate) DE: u16,
        pub(crate) HL: u16,
        pub(crate) PC: u16,
        pub(crate) SP: u16,
        pub(crate) LOW_REGISTERS: phf::Map<&'static str, &'static str>,
        pub(crate) HIGH_REGISTERS: phf::Map<&'static str, &'static str>,
        pub(crate) REGISTERS: [&'static str; 6],
        pub(crate) FLAGS: phf::Map<&'static str, u8>
    }

    impl fmt::Display for Registers {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            writeln!(f, "REGISTERS - AF : 0x{:04X} BC : 0x{:04X} DE : 0x{:04X} HL : 0x{:04X} PC : 0x{:04X} SP : {:b}", self.AF, self.BC, self.DE, self.HL, self.PC, self.SP);
            write!(f, "FLAGS - z:{} n:{} h:{} c:{}", self.get_item("z"), self.get_item("n"), self.get_item("h"), self.get_item("c"))
        }
    }

    impl Registers {
        pub(crate) fn new() -> Registers {
            Registers {
                AF: 0,
                BC: 0,
                DE: 0,
                HL: 0,
                PC: 0x200,    // 0x100 : rom entry point
                SP: 0,
                LOW_REGISTERS,
                HIGH_REGISTERS,
                REGISTERS,
                FLAGS
            }
        }

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

        pub(crate) fn get_item(&self, item: &str) -> u16 {
            if LOW_REGISTERS.contains_key(item) {
                let register_name = LOW_REGISTERS[item];
                let register_value = self.get_register_value_from_name(register_name);
                return register_value & 0xFF // bitmask with 0xFF, get lower 8 bits
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

        pub(crate) fn set_item(&mut self, item: &str, value: u16) {
            if LOW_REGISTERS.contains_key(item) {
                let value = value & 0xFF;
                let register_name = LOW_REGISTERS[item];
                let register_value = self.get_register_value_from_name(register_name);
                let updated_register_value = (register_value & 0xFF00) | value as u16;  // clear last 8 bits masking with 0xFF00 then OR with passed value
                self.set_register_value_from_name(register_name, updated_register_value);
                return;
            }
            if HIGH_REGISTERS.contains_key(item) {
                let value = value & 0xFF;
                let register_name = HIGH_REGISTERS[item];
                let register_value = self.get_register_value_from_name(register_name);
                let updated_register_value = (register_value & 0xFF) | value << 8 as u16;  // clear first 8 bits masking with 0x00FF then OR with passed value shifted to position
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
}
