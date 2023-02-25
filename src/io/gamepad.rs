pub mod gamepad {
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
        pub(crate) selected_column: ColumnType,
        pub(crate) rows_value: (u8, u8)
    }

    impl Default for gamepad {
        fn default() -> Self {
            gamepad {
                selected_column: NotSelected,
                rows_value: (0xF, 0xF)
            }
        }
    }

    impl gamepad {
        pub fn read(&self) -> u8 {
            match self.selected_column {
                Direction => self.rows_value.1,
                Action => self.rows_value.0,
                NotSelected => 0,
                _ => 0
            }
        }

        pub fn write(&mut self, value: u8) {
            match value & 0x30 {
                0x10 => self.selected_column = Direction,
                0x20 => self.selected_column = Action,
                _ => self.selected_column = NotSelected
            }
        }

        pub fn get_column_and_bit_from_key(key: Key) -> (ColumnType, u8) {
            let mut value_for_key = match key {
                Key::Down => (Direction, 3),
                Key::Up => (Direction, 2),
                Key::Left => (Direction, 1),
                Key::Right => (Direction, 0),
                Key::Space => (Action, 3), //Start
                Key::Comma => (Action, 2), //Select
                Key::X => (Action, 1),   //B
                Key::Z => (Action, 0),   //A
                _ => panic!("key {:?} not supported", key)
            };
            value_for_key.1 = 1 << value_for_key.1;
            value_for_key
        }

        pub fn key_pressed(&mut self, key: Key) {
            match gamepad::get_column_and_bit_from_key(key) {
                (Action, bit_mask) => self.rows_value.0 = self.rows_value.0 & !bit_mask,
                (Direction, bit_mask) => self.rows_value.1 = self.rows_value.1 & !bit_mask,
                _ => {}
            };
        }

        pub fn key_released(&mut self, key: Key) {
            match gamepad::get_column_and_bit_from_key(key) {
                (Action, bit_mask) => self.rows_value.0 = self.rows_value.0 | bit_mask,
                (Direction, bit_mask) => self.rows_value.1 = self.rows_value.1 | bit_mask,
                _ => {}
            };
        }
    }
}