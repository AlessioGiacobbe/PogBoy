pub mod op {
    use crate::cpu::CPU::{CPU};
    use crate::cpu::CPU::{JumpCondition, HalfCarryOperationsMode};
    use crate::memory::op_codes_parser::op_codes_parser::{Instruction, Operand};
    use crate::run_cpu;

    impl CPU<'_> {

        pub(crate) fn ld_nn(&mut self, operands: Vec<Operand>, name: &str){
            let d16 = operands.into_iter().find(|operand| operand.name == "d16").expect("Operand d16 not found");
            self.Registers.set_item(name, d16.value.expect("Operand d16 has no value"))
        }

        pub(crate) fn inc_nn(&mut self, name: &str){
            let mut current_value = self.Registers.get_item(name) as i16;
            current_value = current_value.wrapping_add(1);
            self.Registers.set_item(name, current_value as u16);
        }

        pub(crate) fn dec_nn(&mut self, name: &str){
            let mut current_value = self.Registers.get_item(name) as i16;
            current_value = current_value.wrapping_sub(1);
            self.Registers.set_item(name, current_value as u16);
        }

        pub(crate) fn ld_r_r(&mut self, from: &str, to: &str){
            let from_value = self.Registers.get_item(from);
            self.Registers.set_item(to, from_value);
        }

        pub(crate) fn ld_r_hl(&mut self, to: &str){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.Registers.set_item(to, value_at_hl as u16);
        }

        pub(crate) fn add_a_n(&mut self, operands: Vec<Operand>){
            let d8 = operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            self.add_a(d8.value.expect("operand has no value") as u16);
        }

        pub(crate) fn add_a_r(&mut self, to_add: &str){
            let value_to_add = self.Registers.get_item(to_add) as u16;
            self.add_a(value_to_add);
        }

        pub(crate) fn ldh_a8_a(&mut self, Instruction: Instruction){
            let a8 = Instruction.operands.into_iter().find(|operand| operand.name == "a8").expect("Operand a8 not found").value.unwrap();
            let a = self.Registers.get_item("A");
            self.MMU.write_byte((0xFF00 + a8) as i32, a as u8);
        }

        pub(crate) fn ldh_a_a8(&mut self, Instruction: Instruction){
            let a8 = Instruction.operands.into_iter().find(|operand| operand.name == "a8").expect("Operand a8 not found").value.unwrap();
            self.Registers.set_item("A", self.MMU.read_byte((0xFF00 + a8) as i32) as u16)
        }

        pub(crate) fn ld_a16_pointer_a(&mut self, Instruction: Instruction){
            let a16 = Instruction.operands.into_iter().find(|operand| operand.name == "a16").expect("Operand a16 not found").value.unwrap();
            let a = self.Registers.get_item("A");
            self.MMU.write_byte(a16 as i32, a as u8);
        }

        pub(crate) fn ld_a_a16_pointer(&mut self, Instruction: Instruction){
            let a16 = Instruction.operands.into_iter().find(|operand| operand.name == "a16").expect("Operand a16 not found").value.unwrap();
            let value_at_a16 = self.MMU.read_byte(a16 as i32);
            self.Registers.set_item("A", value_at_a16 as u16);
        }

        pub(crate) fn ld_a16_pointer_sp(&mut self, Instruction: Instruction){
            let a16 = Instruction.operands.into_iter().find(|operand| operand.name == "a16").expect("Operand a16 not found").value.unwrap();
            let sp = self.Registers.get_item("SP");
            self.MMU.write_word(a16 as i32, sp);
        }

        pub(crate) fn ld_a_c_pointer(&mut self){
            let c = self.Registers.get_item("C");
            let value_at_c = self.MMU.read_byte((0xFF00 + c) as i32);
            self.Registers.set_item("A", value_at_c as u16);
        }

        pub(crate) fn ld_c_pointer_a(&mut self){
            let c = self.Registers.get_item("C");
            let a = self.Registers.get_item("A");
            self.MMU.write_byte((0xFF00 + c) as i32, a as u8);
        }

        pub(crate) fn add_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.add_a(value_at_hl as u16);
        }

        pub(crate) fn add_a(&mut self, value: u16){
            let current_value = self.Registers.get_item("A") as u16;
            let result = current_value + value;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", (result > 0xFF) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value as i16, value as i16, 0, HalfCarryOperationsMode::Add) as u16);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn adc_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.adc_a(value_at_hl as i16);
        }

        pub(crate) fn adc_a_r(&mut self, to_add: &str){
            let to_add = self.Registers.get_item(to_add) as i16;
            self.adc_a(to_add);
        }

        pub(crate) fn adc_a_d8(&mut self, Instruction: Instruction){
            let d8 = Instruction.operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found").value.expect("Operand should have a value");
            self.adc_a(d8 as i16);
        }

        pub(crate) fn adc_a(&mut self, value: i16) {
            let current_value = self.Registers.get_item("A") as i16;
            let carry = self.Registers.get_item("c") as i16;
            let result = value + current_value + carry;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", (result > 0xFF) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, value, carry, HalfCarryOperationsMode::Add) as u16);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn sub_a_r(&mut self, to_sub: &str) {
            let to_sub = self.Registers.get_item(to_sub) as i16;
            self.sub_a(to_sub)
        }

        pub(crate) fn sub_a_n(&mut self, operands: Vec<Operand>){
            let d8 = operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            self.sub_a(d8.value.expect("operand has no value") as i16);
        }

        pub(crate) fn sub_a_hl(&mut self) {
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.sub_a(value_at_hl as i16);
        }

        pub(crate) fn sub_a(&mut self, to_sub: i16){
            let current_value = self.Registers.get_item("A") as i16;
            let result = current_value - to_sub;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", (to_sub > current_value) as u16);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, to_sub, 0, HalfCarryOperationsMode::GreaterThan) as u16);
            self.Registers.set_item("n", 1);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn sbc_a_r(&mut self, to_sub: &str){
            let to_sub = self.Registers.get_item(to_sub) as i16;
            self.sbc_a(to_sub)
        }

        pub(crate) fn sbc_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.sbc_a(value_at_hl as i16);
        }

        pub(crate) fn sbc_a_d8(&mut self, Instruction: Instruction){
            let d8 = Instruction.operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found").value.expect("Operand should have a value");
            self.sbc_a(d8 as i16);
        }

        pub(crate) fn sbc_a(&mut self, value: i16) {
            let current_value = self.Registers.get_item("A") as i16;
            let carry = self.Registers.get_item("c") as i16;
            let result = current_value - value - carry;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", ((value + carry) > current_value) as u16);
            self.Registers.set_item("h", (((current_value & 0xF) - (value & 0xF) - carry) < 0) as u16);
            self.Registers.set_item("n", 1);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn add_sp_r8(&mut self, Instruction: Instruction){
            let r8 = Instruction.operands.into_iter().find(|operand| operand.name == "r8").expect("Operand r8 not found").value.unwrap() as i8;
            let sp = self.Registers.get_item("SP") as u32;
            self.Registers.set_item("z", 0);
            self.Registers.set_item("n", 0);

            let result = sp.wrapping_add(r8 as u32);

            self.Registers.set_item("c", (result & 0xFFFF0000) as u16);
            self.Registers.set_item("h", ((sp & 0xF).wrapping_add((r8 & 0xF) as u32) > 0xF) as u16);

            self.Registers.set_item("SP", result as u16)
        }

        pub(crate) fn and_a_r(&mut self, to_and: &str){
            let to_and = self.Registers.get_item(to_and) as i16;
            self.and_a_value(to_and);
        }

        pub(crate) fn and_a_n(&mut self, operands: Vec<Operand>){
            let d8 = operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            self.and_a_value(d8.value.expect("operand has no value") as i16);
        }

        pub(crate) fn and_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.and_a_value(value_at_hl as i16);
        }

        pub(crate) fn and_a_value(&mut self, to_and: i16){
            let current_value = self.Registers.get_item("A") as i16;
            let result = current_value & to_and;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", 0);
            self.Registers.set_item("h", 1);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn or_a_r(&mut self, to_or: &str){
            let to_or = self.Registers.get_item(to_or) as i16;
            self.or_a_value(to_or);
        }

        pub(crate) fn or_a_n(&mut self, operands: Vec<Operand>){
            let d8 = operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            self.or_a_value(d8.value.expect("operand has no value") as i16);
        }

        pub(crate) fn or_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.or_a_value(value_at_hl as i16);
        }

        pub(crate) fn or_a_value(&mut self, to_or: i16){
            let current_value = self.Registers.get_item("A") as i16;
            let result = current_value | to_or;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", 0);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn xor_a_r(&mut self, to_xor: &str) {
            let to_xor = self.Registers.get_item(to_xor) as i16;
            self.xor_a(to_xor);
        }

        pub(crate) fn xor_a_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);
            self.xor_a(value_at_hl as i16);
        }

        pub(crate) fn xor_a_d8(&mut self, Instruction: Instruction){
            let d8 = Instruction.operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found").value.expect("Operand should have a value");
            self.xor_a(d8 as i16);
        }

        pub(crate) fn xor_a(&mut self, value: i16){
            let current_value = self.Registers.get_item("A") as i16;
            let result = current_value ^ value;

            self.Registers.set_item("A", result as u16);
            self.Registers.set_item("c", 0);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
        }

        pub(crate) fn cp_a_r(&mut self, to_cp: &str) {
            let to_cp = self.Registers.get_item(to_cp);
            self.cp_a(to_cp as i16);
        }

        pub(crate) fn cp_a_d8(&mut self, Instruction: Instruction) {
            let d8 = Instruction.operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found").value.expect("Operand should have a value");
            self.cp_a(d8 as i16);
        }

        pub(crate) fn cp_a(&mut self, to_cp: i16) {
            let old_a = self.Registers.get_item("A");
            self.sub_a(to_cp);
            self.Registers.set_item("A", old_a);
        }

        pub(crate) fn cp_a_hl(&mut self){
            let old_a = self.Registers.get_item("A");
            self.sub_a_hl();
            self.Registers.set_item("A", old_a);
        }

        pub(crate) fn ld_sp_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            self.Registers.set_item("SP", hl)
        }

        pub(crate) fn ld_hl_sp_r8(&mut self, Instruction: Instruction){
            let r8 = Instruction.operands.into_iter().find(|operand| operand.name == "r8").expect("Operand r8 not found").value.unwrap() as i8;
            let sp = self.Registers.get_item("SP") as u32;
            self.Registers.set_item("z", 0);
            self.Registers.set_item("n", 0);

            let result: u32 = sp.wrapping_add(r8 as u32);

            self.Registers.set_item("c", (sp.wrapping_add(r8 as u32) > 0xFFFF) as u16);
            self.Registers.set_item("h", ((sp & 0xF).wrapping_add((r8 & 0xF) as u32) > 0xF) as u16);

            self.Registers.set_item("HL", result as u16)
        }

        pub(crate) fn ld_hl_pointer_d8(&mut self, instruction: Instruction){
            let d8 = instruction.operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            let hl = self.Registers.get_item("HL") as i32;
            self.MMU.write_byte(hl, d8.value.expect("d8 has no value") as u8);
        }

        pub(crate) fn ld_hl_pointer_dec_inc_a(&mut self, increase: bool){
            let mut hl = self.Registers.get_item("HL");
            let a = self.Registers.get_item("A");
            self.MMU.write_byte(hl as i32, a as u8);
            if increase {
                hl += 1;
            }else {
                hl -= 1;
            }
            self.Registers.set_item("HL", hl);
        }

        pub(crate) fn inc_hl_pointer(&mut self) {
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);

            let result = value_at_hl.wrapping_add(1);

            self.Registers.set_item("n", 0);
            self.Registers.set_item("h", CPU::calculate_half_carry(value_at_hl as i16, 0, 0, HalfCarryOperationsMode::Increment) as u16);
            self.MMU.write_byte(hl as i32, result);
            self.Registers.set_item("z", (self.MMU.read_byte(hl as i32) == 0) as u16);
        }

        pub(crate) fn inc(&mut self, to_inc: &str) {
            let current_value = self.Registers.get_item(to_inc) as i16;
            let result = current_value + 1;

            self.Registers.set_item("n", 0);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, 0, 0, HalfCarryOperationsMode::Increment) as u16);
            self.Registers.set_item(to_inc, result as u16);
            self.Registers.set_item("z", (self.Registers.get_item(to_inc) == 0) as u16);
        }

        pub(crate) fn dec_hl_pointer(&mut self) {
            let hl = self.Registers.get_item("HL");
            let value_at_hl = self.MMU.read_byte(hl as i32);

            let result = value_at_hl - 1;

            self.Registers.set_item("n", 1);
            self.Registers.set_item("h", CPU::calculate_half_carry(value_at_hl as i16, 0, 0, HalfCarryOperationsMode::Decrement) as u16);
            self.MMU.write_byte(hl as i32, result);
            self.Registers.set_item("z", (self.MMU.read_byte(hl as i32) == 0) as u16);
        }

        pub(crate) fn dec(&mut self, to_dec: &str) {
            let current_value = self.Registers.get_item(to_dec) as i16;
            let result = current_value - 1;

            self.Registers.set_item("n", 1);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value, 0, 0, HalfCarryOperationsMode::Decrement) as u16);
            self.Registers.set_item(to_dec, result as u16);
            self.Registers.set_item("z", (self.Registers.get_item(to_dec) == 0) as u16);
        }

        pub(crate) fn ld_r_d8(&mut self, destination: &str, instruction: Instruction){
            let d8 = instruction.operands.into_iter().find(|operand| operand.name == "d8").expect("Operand d8 not found");
            self.Registers.set_item(destination, d8.value.expect("Operand d8 has no value"))
        }

        pub(crate) fn ld_address_value(&mut self, address_pointer: &str, value: &str){
            let address = self.Registers.get_item(address_pointer);
            let value = self.Registers.get_item(value);
            self.MMU.write_byte(address as i32, value as u8)
        }

        pub(crate) fn ld_a_address(&mut self, address_pointer: &str){
            let address = self.Registers.get_item(address_pointer);
            let value = self.MMU.read_byte(address as i32);
            self.Registers.set_item("A", value as u16);
        }

        pub(crate) fn rr_r(&mut self, to_rr: &str, should_set_zero: bool){
            let current_value = self.Registers.get_item(to_rr) as u8;
            let result = self.rr_value(current_value, should_set_zero);
            self.Registers.set_item(to_rr, result as u16);
        }

        pub(crate) fn rr_hl_pointer(&mut self) {
            let value_at_hl = self.Registers.get_item("HL") as i32;
            let current_value = self.MMU.read_byte(value_at_hl);
            let result = self.rr_value(current_value, true);
            self.MMU.write_byte(value_at_hl, result);
        }

        pub(crate) fn rr_value(&mut self, value: u8, should_set_zero: bool) -> u8 {
            let carry = value & 1;  //0th bit
            let current_carry = self.Registers.get_item("c") as u8;
            let result = value >> 1 | current_carry << 7;

            self.Registers.set_item("c", carry as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);

            if should_set_zero {
                self.Registers.set_item("z",  (result == 0) as u16);
            }else{
                self.Registers.set_item("z",  0 as u16);
            }

            return result
        }

        //move left saving least significant bit to carry
        pub (crate) fn sla_r(&mut self, to_sla: &str){
            let current_value = self.Registers.get_item(to_sla) as u8;
            let result = self.sla_value(current_value);
            self.Registers.set_item(to_sla, result as u16);
        }

        pub(crate) fn sla_hl_pointer(&mut self) {
            let value_at_hl = self.Registers.get_item("HL") as i32;
            let current_value = self.MMU.read_byte(value_at_hl);
            let result = self.sla_value(current_value);
            self.MMU.write_byte(value_at_hl, result);
        }

        pub(crate) fn sla_value(&mut self, value: u8) -> u8{
            let carry = value & 0x80;
            let result = value << 1;

            self.Registers.set_item("c", (carry > 0) as u16);
            self.Registers.set_item("z", (result == 0) as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);

            return result;
        }

        //move right preserving most significant bit
        pub(crate) fn sra_r(&mut self, to_sra: &str){
            let current_value = self.Registers.get_item(to_sra) as u8;
            let result = self.sra_value(current_value);
            self.Registers.set_item(to_sra, result as u16);
        }

        pub(crate) fn sra_hl_pointer(&mut self) {
            let value_at_hl = self.Registers.get_item("HL") as i32;
            let current_value = self.MMU.read_byte(value_at_hl);
            let result = self.sra_value(current_value);
            self.MMU.write_byte(value_at_hl, result);
        }

        pub(crate) fn sra_value(&mut self, value: u8) -> u8{
            let carry = value & 0x1;
            let result = (value & 0x80) | value >> 1;

            self.Registers.set_item("c", (carry > 0) as u16);
            self.Registers.set_item("z", (result == 0) as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);

            result
        }

        //same as sra but MSB is not preserved
        pub(crate) fn srl_r(&mut self, to_srl: &str){
            let current_value = self.Registers.get_item(to_srl) as u8;
            let result = self.srl_value(current_value);
            self.Registers.set_item(to_srl, result as u16);
        }

        pub(crate) fn srl_hl_pointer(&mut self) {
            let value_at_hl = self.Registers.get_item("HL") as i32;
            let current_value = self.MMU.read_byte(value_at_hl);
            let result = self.srl_value(current_value);
            self.MMU.write_byte(value_at_hl, result);
        }

        pub(crate) fn srl_value(&mut self, value: u8) -> u8{
            let carry = value & 0x1;
            let result = value >> 1;

            self.Registers.set_item("c", (carry > 0) as u16);
            self.Registers.set_item("z", (result == 0) as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);

            result
        }

        //test bit at position n in r register
        pub(crate) fn bit_n_r(&mut self, position: u8, target_register: &str){
            let current_value = self.Registers.get_item(target_register) as u8;
            self.bit_n_value(position, current_value);
        }

        pub(crate) fn bit_hl_pointer(&mut self, position: u8) {
            let value_at_hl = self.Registers.get_item("HL") as i32;
            let current_value = self.MMU.read_byte(value_at_hl);
            self.bit_n_value(position, current_value);
        }

        pub(crate) fn bit_n_value(&mut self, position: u8, value: u8) {
            let value_at_bit = (value >> position) & 0x1;

            self.Registers.set_item("z", (value_at_bit == 0) as u16);
            self.Registers.set_item("h", 1);
            self.Registers.set_item("n", 0);
        }

        //reset bit at position n in register r
        pub(crate) fn res_n_r(&mut self, position: u8, target_register: &str){
            let current_value = self.Registers.get_item(target_register) as u8;
            let result = self.res_n_value(position, current_value);
            self.Registers.set_item(target_register, result as u16);
        }

        pub(crate) fn res_hl_pointer(&mut self, position: u8) {
            let value_at_hl = self.Registers.get_item("HL") as i32;
            let current_value = self.MMU.read_byte(value_at_hl);
            let result = self.res_n_value(position, current_value);
            self.MMU.write_byte(value_at_hl, result);
        }

        pub(crate) fn res_n_value(&mut self, position: u8, mut value: u8) -> u8 {
            value &= !(1 << position);  //create an inverse mask shifting 1 by x position and inverting it
            value
        }

        //set bit at position n in register r
        pub(crate) fn set_n_r(&mut self, position: u8, target_register: &str){
            let current_value = self.Registers.get_item(target_register) as u8;
            let result = self.set_n_value(position, current_value);
            self.Registers.set_item(target_register, result as u16);
        }

        pub(crate) fn set_hl_pointer(&mut self, position: u8) {
            let value_at_hl = self.Registers.get_item("HL") as i32;
            let current_value = self.MMU.read_byte(value_at_hl);
            let result = self.set_n_value(position, current_value);
            self.MMU.write_byte(value_at_hl, result);
        }

        pub(crate) fn set_n_value(&mut self, position: u8, mut value: u8) -> u8 {
            value |= 1 << position;  //or mask value with bit positioned at right place
            value
        }

        //swap most and least significant nibbles
        pub(crate) fn swap_r(&mut self, to_swap: &str) {
            let current_value = self.Registers.get_item(to_swap) as u8;
            let result = self.swap_value(current_value);
            self.Registers.set_item(to_swap, result as u16);
        }

        pub(crate) fn swap_hl_pointer(&mut self) {
            let value_at_hl = self.Registers.get_item("HL") as i32;
            let current_value = self.MMU.read_byte(value_at_hl);
            let result = self.swap_value(current_value);
            self.MMU.write_byte(value_at_hl, result);
        }

        pub(crate) fn swap_value(&mut self, value: u8) -> u8{
            let most_significant_nibble = value & 0xF0;
            let least_significant_nibble = value & 0x0F;
            let result = (most_significant_nibble >> 4) | (least_significant_nibble << 4);

            self.Registers.set_item("z", (result == 0) as u16);
            self.Registers.set_item("c", 0);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);

            result
        }

        //rrca is rotate right circular, no data is loss
        pub(crate) fn rrc_r(&mut self, to_rrc: &str, should_set_zero: bool){
            let current_value = self.Registers.get_item(to_rrc) as u8;
            let result = self.rrc_value(current_value, should_set_zero);
            self.Registers.set_item(to_rrc, result as u16);
        }

        pub(crate) fn rrc_hl_pointer(&mut self) {
            let value_at_hl = self.Registers.get_item("HL") as i32;
            let current_value = self.MMU.read_byte(value_at_hl);
            let result = self.rrc_value(current_value, true);
            self.MMU.write_byte(value_at_hl, result);
        }

        pub(crate) fn rrc_value(&mut self, value: u8, should_set_zero: bool) -> u8 {
            let carry = value & 1;
            let result = value.rotate_right(1);

            self.Registers.set_item("c", carry as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);

            if should_set_zero {
                self.Registers.set_item("z",  (result == 0) as u16);
            }else{
                self.Registers.set_item("z",  0 as u16);
            }
            return result
        }

        pub(crate) fn rl_r(&mut self, to_rl: &str, should_set_zero: bool){
            let current_value = self.Registers.get_item(to_rl) as u8;
            let result = self.rl_value(current_value, should_set_zero);
            self.Registers.set_item(to_rl, result as u16);
        }

        pub(crate) fn rl_hl_pointer(&mut self) {
            let value_at_hl = self.Registers.get_item("HL") as i32;
            let current_value = self.MMU.read_byte(value_at_hl);
            let result = self.rl_value(current_value, true);
            self.MMU.write_byte(value_at_hl, result);
        }

        pub(crate) fn rl_value(&mut self, value: u8, should_set_zero: bool) -> u8 {
            let carry = (value & 0x80) >> 7;
            let current_carry = self.Registers.get_item("c") as u8;
            let result = (value << 1) | current_carry;

            self.Registers.set_item("c", carry as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);

            if should_set_zero {
                self.Registers.set_item("z",  (result == 0) as u16);
            }else{
                self.Registers.set_item("z",  0 as u16);
            }

            return result
        }

        pub(crate) fn rst(&mut self, new_pc: u16){
            self.write_to_stack(self.Registers.get_item("PC"));
            self.Registers.set_item("PC", new_pc);
        }

        pub(crate) fn jr_r8(&mut self, Instruction: Instruction, JumpCondition: JumpCondition){
            let r8 = Instruction.operands.into_iter().find(|operand| operand.name == "r8").expect("Operand r8 not found").value.unwrap() as i8;
            let current_pc = self.Registers.get_item("PC");
            let should_jump = self.checkJumpCondition(&JumpCondition);
            if should_jump {
                if JumpCondition != JumpCondition::None {
                    self.clock += 4;    //12 - 8
                }
                self.Registers.set_item("PC", current_pc.wrapping_add(r8 as u16));
            }
        }

        pub(crate) fn jp_hl(&mut self){
            let hl = self.Registers.get_item("HL");
            self.Registers.set_item("PC", hl);
        }

        pub(crate) fn jp_a16(&mut self, Instruction: Instruction, JumpCondition: JumpCondition){
            let should_jump = self.checkJumpCondition( &JumpCondition);
            if should_jump {
                if JumpCondition != JumpCondition::None {
                    self.clock += 12;
                }
                let a16 = Instruction.operands.into_iter().find(|operand| operand.name == "a16").expect("Operand a16 not found").value.unwrap();
                self.Registers.set_item("PC", a16)
            }
        }

        pub(crate) fn call_a16(&mut self, Instruction: Instruction, CallCondition: JumpCondition){
            let should_call = self.checkJumpCondition(&CallCondition);
            if should_call {
                if CallCondition != JumpCondition::None {
                    self.clock += 12;
                }
                let a16 = Instruction.operands.into_iter().find(|operand| operand.name == "a16").expect("Operand a16 not found").value.unwrap();
                let pc = self.Registers.get_item("PC"); //is already pointing to next instruction
                self.write_to_stack(pc);
                self.Registers.set_item("PC", a16);
            }
        }

        pub(crate) fn ret(&mut self, ReturnCondition: JumpCondition, EnableInterrupts: bool){
            let should_return = self.checkJumpCondition(&ReturnCondition);
            if should_return {
                let jump_location = self.read_from_stack();
                self.Registers.set_item("PC", jump_location);
                if ReturnCondition != JumpCondition::None {
                    self.clock += 12;
                }
                if EnableInterrupts {
                    self.MMU.interrupt_master_enabled = 1;
                }
            }
        }


        pub(crate) fn rlc_r(&mut self, to_rlc: &str, should_set_zero: bool){
            let current_value = self.Registers.get_item(to_rlc) as u8;
            let result = self.rlc_value(current_value, should_set_zero);
            self.Registers.set_item(to_rlc,result as u16);
        }

        pub(crate) fn rlc_hl_pointer(&mut self) {
            let value_at_hl = self.Registers.get_item("HL") as i32;
            let current_value = self.MMU.read_byte(value_at_hl);
            let result = self.rlc_value(current_value, true);
            self.MMU.write_byte(value_at_hl, result);
        }

        pub(crate) fn rlc_value(&mut self, value: u8, should_set_zero: bool) -> u8 {
            let carry = (value & 0x80) >> 7;
            let result = value.rotate_left(1);

            self.Registers.set_item("c", carry as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("n", 0);

            if should_set_zero {
                self.Registers.set_item("z", (result == 0) as u16);
            } else {
                self.Registers.set_item("z", 0 as u16);
            }

            return result
        }

        pub(crate) fn daa(&mut self){
            let mut current_value = self.Registers.get_item("A") as u16;
            let negative = self.Registers.get_item("n") as u8;
            let carry = self.Registers.get_item("c") as u8;
            let half_carry = self.Registers.get_item("h") as u8;

            let mut result = 0;

            if half_carry != 0 {
                result |= 0x06
            }

            if carry != 0 {
                result |= 0x60
            }

            if negative != 0 {
                current_value -= result;
            } else {
                if (current_value & 0x0F) > 0x09 {
                    result |= 0x06
                }

                if current_value > 0x99 {
                    result |= 0x60
                }

                current_value += result;
            }

            self.Registers.set_item("A", current_value as u16);
            self.Registers.set_item("h", 0);
            self.Registers.set_item("z", (self.Registers.get_item("A") == 0) as u16);
            self.Registers.set_item("c", (result & 0x60 != 0) as u16);
        }

        //supplement carry (0 -> 1, 1 -> 0)
        pub(crate) fn ccf(&mut self){
            let carry = self.Registers.get_item("c") as u8;
            self.Registers.set_item("c", (carry == 0) as u16);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("h", 0);
        }

        //complement accumulator
        pub(crate) fn cpl(&mut self){
            let accumulator = self.Registers.get_item("A") as u8;
            self.Registers.set_item("A", !accumulator as u16);
            self.Registers.set_item("n", 1);
            self.Registers.set_item("h", 1);
        }

        pub(crate) fn scf(&mut self){
            self.Registers.set_item("c", 1);
            self.Registers.set_item("n", 0);
            self.Registers.set_item("h", 0);
        }

        pub(crate) fn add_hl_n(&mut self, to_add: &str){
            let current_value = self.Registers.get_item("HL") as u32;
            let to_add = self.Registers.get_item(to_add) as u32;
            let result: u32 = current_value + to_add;
            self.Registers.set_item("HL", result as u16);
            self.Registers.set_item("c", (result & 0x10000 != 0) as u16); //if true, result is bigger than 16 bit max value
            self.Registers.set_item("n", 0);
            self.Registers.set_item("h", CPU::calculate_half_carry(current_value as i16, to_add as i16, 0, HalfCarryOperationsMode::AddWords) as u16);
        }

        pub(crate) fn push_rr(&mut self, to_push: &str){
            let to_push = self.Registers.get_item(to_push);
            self.write_to_stack(to_push);
        }

        pub(crate) fn pop_rr(&mut self, pop_into: &str){
            let mut value = self.read_from_stack();
            self.Registers.set_item(pop_into, value)
        }

        pub(crate) fn write_to_stack(&mut self, value: u16){
            let mut sp = self.Registers.get_item("SP");
            sp -= 2;
            self.MMU.write_byte(sp as i32, (value & 0x00FF) as u8);
            //since value is 16 bits we have to mask and shift to get the first 8 bits
            self.MMU.write_byte((sp + 1) as i32, ((value & 0xFF00) >> 8)  as u8);
            self.Registers.set_item("SP", sp);
        }

        pub(crate) fn read_from_stack(&mut self) -> u16 {
            let sp = self.Registers.get_item("SP");
            let first_8_bits = self.MMU.read_byte(sp as i32) as u16;
            let last_8_bits = self.MMU.read_byte((sp + 1) as i32) as u16;
            self.Registers.set_item("SP", sp + 2);
            (first_8_bits | last_8_bits << 8) as u16
        }

        //half carry is carry calculated on the first half of a byte (from 3rd bit)
        pub(crate) fn calculate_half_carry(value: i16, second_operator: i16, carry: i16, mode: HalfCarryOperationsMode) -> bool{
            let rounded_value = value & 0xF;
            let rounded_second_operator = second_operator & 0xF;
            let rounded_word = value & 0xFFF;
            let rounded_second_word = value & 0xFFF;
            match mode {
                HalfCarryOperationsMode::Add => {
                    (rounded_second_operator + rounded_value + carry) > 0xF
                }
                HalfCarryOperationsMode::AddWords => {
                    (rounded_word +  rounded_second_word) > 0xFFF
                }
                HalfCarryOperationsMode::GreaterThan => {
                    (rounded_second_operator + carry) > rounded_value
                }
                HalfCarryOperationsMode::Increment => {
                    (rounded_value + 1) > 0xF
                }
                HalfCarryOperationsMode::Decrement => {
                    //we carry only if value is 0
                    rounded_value == 0
                }
            }
        }


        pub(crate) fn checkJumpCondition(&mut self, JumpCondition: &JumpCondition) -> bool {
            match JumpCondition {
                JumpCondition::Zero => self.Registers.get_item("z") == 1,
                JumpCondition::NotZero => self.Registers.get_item("z") == 0,
                JumpCondition::Carry => self.Registers.get_item("c") == 1,
                JumpCondition::NotCarry => self.Registers.get_item("c") == 0,
                JumpCondition::None => true
            }
        }
    }
}