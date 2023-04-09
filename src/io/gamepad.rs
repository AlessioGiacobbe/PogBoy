pub mod gamepad {
    use std::collections::HashMap;
    use piston_window::Key;
    use crate::io::gamepad::gamepad::ColumnType::{Action, Direction, NotSelected};

    #[derive(Debug)]
    pub enum ColumnType {
        Action,
        Direction,
        NotSelected
    }

    #[derive(Debug)]
    pub struct gamepad {
        pub(crate) value: u8,
        pub(crate) selected_column: ColumnType,
        pub(crate) rows_value: (u8, u8),
        pub(crate) pressed_buttons: HashMap<String, (u8, u8)>
    }

    impl Default for gamepad {
        fn default() -> Self {
            gamepad {
                value: 0,
                selected_column: NotSelected,
                rows_value: (0xF, 0xF),
                pressed_buttons: HashMap::new()
            }
        }
    }

    impl gamepad {


        pub fn read(& self) -> u8 {
            let mut result = self.value | 0b11001111;
            for pressed_button in self.pressed_buttons.iter() {
                let (_, (line, mask)) = pressed_button;
                if line & result == 0 {
                    result &= (0xFF & !mask);
                }
            }
            result
        }

        pub fn write(&mut self, value: u8) {
            self.value = value & 0b00110000;
        }

        pub fn get_line_and_mask_from_key(key: Key) -> (u8, u8) {
            return match key {
                Key::Down => (0x10, 0x08),
                Key::Up => (0x10, 0x04),
                Key::Left => (0x10, 0x02),
                Key::Right => (0x10, 0x01),
                Key::Space => (0x20, 0x08), //Start
                Key::Comma => (0x20, 0x04), //Select
                Key::X => (0x20, 0x02),   //B
                Key::Z => (0x20, 0x01),   //A
                _ => panic!("key {:?} not supported", key)
            };
        }

        pub fn key_pressed(&mut self, key: Key) {
            let (line, mask) = gamepad::get_line_and_mask_from_key(key);
            self.pressed_buttons.insert((line+mask).to_string(), (line, mask));
        }

        pub fn key_released(&mut self, key: Key) {
            let (line, mask) = gamepad::get_line_and_mask_from_key(key);
            self.pressed_buttons.remove(&*(line + mask).to_string());
        }
    }
}