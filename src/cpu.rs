use crate::memory::Memory;
use std::collections::HashMap;

const MOV_LIT_R1: u8 = 0x10;
const MOV_LIT_R2: u8 = 0x11;
const ADD_REG_REG: u8 = 0x12;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RegisterName {
    Ip,
    Acc,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

pub struct CPU {
    registers: HashMap<RegisterName, u16>,
    memory: Memory,
}

impl CPU {
    pub fn new(memory: Memory) -> Self {
        let mut registers = HashMap::new();
        registers.insert(RegisterName::Ip, 0);
        registers.insert(RegisterName::Acc, 0);
        registers.insert(RegisterName::R1, 0);
        registers.insert(RegisterName::R2, 0);
        registers.insert(RegisterName::R3, 0);
        registers.insert(RegisterName::R4, 0);
        registers.insert(RegisterName::R5, 0);
        registers.insert(RegisterName::R6, 0);
        registers.insert(RegisterName::R7, 0);
        registers.insert(RegisterName::R8, 0);
        CPU { registers, memory }
    }

    pub fn get_register_value(&mut self, name: RegisterName) -> u16 {
        *self.registers.get(&name).unwrap_or(&0)
    }

    pub fn set_register_value(&mut self, name: RegisterName, value: u16) {
        self.registers.insert(name, value);
    }

    pub fn fetch(&mut self) -> u8 {
        let ip = self.get_register_value(RegisterName::Ip);
        let instruction = self.memory.get_memory_u8(ip as usize);
        self.set_register_value(RegisterName::Ip, ip + 1);
        instruction
    }

    pub fn fetch_16(&mut self) -> u16 {
        let ip = self.get_register_value(RegisterName::Ip);
        let instruction = self.memory.get_memory_u16(ip as usize);
        self.set_register_value(RegisterName::Ip, ip + 2);
        instruction
    }

    pub fn execute(&mut self, instruction: u8) {
        match instruction {
            MOV_LIT_R1 => {
                let literal = self.fetch_16();
                self.set_register_value(RegisterName::R1, literal);
            }
            _ => panic!("Unknown instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_get_register_value() {
        let memory = Memory::new(10);
        let mut cpu = CPU::new(memory);

        let acc = cpu.get_register_value(RegisterName::Acc);

        assert_eq!(acc, 0);
    }

    #[test]
    fn should_be_able_to_set_register_value() {
        let memory = Memory::new(10);
        let mut cpu = CPU::new(memory);

        cpu.set_register_value(RegisterName::Acc, 1);
        cpu.set_register_value(RegisterName::Ip, 2);
        cpu.set_register_value(RegisterName::R1, 3);
        cpu.set_register_value(RegisterName::R2, 4);
        cpu.set_register_value(RegisterName::R3, 5);
        cpu.set_register_value(RegisterName::R4, 6);
        cpu.set_register_value(RegisterName::R5, 7);
        cpu.set_register_value(RegisterName::R6, 8);
        cpu.set_register_value(RegisterName::R7, 9);
        cpu.set_register_value(RegisterName::R8, 10);

        assert_eq!(cpu.get_register_value(RegisterName::Acc), 1);
        assert_eq!(cpu.get_register_value(RegisterName::Ip), 2);
        assert_eq!(cpu.get_register_value(RegisterName::R1), 3);
        assert_eq!(cpu.get_register_value(RegisterName::R2), 4);
        assert_eq!(cpu.get_register_value(RegisterName::R3), 5);
        assert_eq!(cpu.get_register_value(RegisterName::R4), 6);
        assert_eq!(cpu.get_register_value(RegisterName::R5), 7);
        assert_eq!(cpu.get_register_value(RegisterName::R6), 8);
        assert_eq!(cpu.get_register_value(RegisterName::R7), 9);
        assert_eq!(cpu.get_register_value(RegisterName::R8), 10);
    }

    #[test]
    fn should_fetch_instruction_and_increment_ip() {
        let mut memory = Memory::new(10);
        memory.set_memory(0, 0x12);

        let mut cpu = CPU::new(memory);
        cpu.set_register_value(RegisterName::Ip, 0);

        let instruction_fetched = cpu.fetch();

        assert_eq!(instruction_fetched, 0x12);
        assert_eq!(cpu.get_register_value(RegisterName::Ip), 1);
    }

    #[test]
    fn should_fetch_instruction_16_and_increment_ip_to_two() {
        let mut memory = Memory::new(10);
        memory.set_memory(0, 0x12);
        memory.set_memory(1, 0x34);

        let mut cpu = CPU::new(memory);
        cpu.set_register_value(RegisterName::Ip, 0);

        let instruction_fetched = cpu.fetch_16();

        assert_eq!(instruction_fetched, 0x1234);
        assert_eq!(cpu.get_register_value(RegisterName::Ip), 2);
    }

    #[test]
    fn should_execute_mov_lit_r1() {
        let mut memory = Memory::new(10);
        memory.set_memory(0, 0x12);
        memory.set_memory(1, 0x34);

        let mut cpu = CPU::new(memory);
        cpu.set_register_value(RegisterName::Ip, 0);

        cpu.execute(MOV_LIT_R1);

        assert_eq!(cpu.get_register_value(RegisterName::R1), 0x1234);
        assert_eq!(cpu.get_register_value(RegisterName::Ip), 2);
    }
}
