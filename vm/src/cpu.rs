use crate::instruction::*;
use crate::memory::Memory;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RegisterName {
    Ip = 0,
    Acc = 1,
    R1 = 2,
    R2 = 3,
    R3 = 4,
    R4 = 5,
    R5 = 6,
    R6 = 7,
    R7 = 8,
    R8 = 9,
}

impl fmt::Display for RegisterName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegisterName::Ip => write!(f, "IP"),
            RegisterName::Acc => write!(f, "ACC"),
            RegisterName::R1 => write!(f, "R1"),
            RegisterName::R2 => write!(f, "R2"),
            RegisterName::R3 => write!(f, "R3"),
            RegisterName::R4 => write!(f, "R4"),
            RegisterName::R5 => write!(f, "R5"),
            RegisterName::R6 => write!(f, "R6"),
            RegisterName::R7 => write!(f, "R7"),
            RegisterName::R8 => write!(f, "R8"),
        }
    }
}

pub struct CPU<'a> {
    register_map: HashMap<RegisterName, u16>,
    memory: &'a mut Memory,
}

impl<'a> CPU<'a> {
    pub fn new(memory: &'a mut Memory) -> CPU<'a> {
        let mut register_map = HashMap::new();
        register_map.insert(RegisterName::Ip, 0);
        register_map.insert(RegisterName::Acc, 0);
        register_map.insert(RegisterName::R1, 0);
        register_map.insert(RegisterName::R2, 0);
        register_map.insert(RegisterName::R3, 0);
        register_map.insert(RegisterName::R4, 0);
        register_map.insert(RegisterName::R5, 0);
        register_map.insert(RegisterName::R6, 0);
        register_map.insert(RegisterName::R7, 0);
        register_map.insert(RegisterName::R8, 0);
        CPU {
            register_map,
            memory,
        }
    }

    pub fn get_register_value(&mut self, name: RegisterName) -> u16 {
        *self.register_map.get(&name).unwrap_or(&0)
    }

    pub fn set_register_value(&mut self, name: RegisterName, value: u16) {
        self.register_map.insert(name, value);
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
            MOV_LIT_REG => {
                let literal = self.fetch_16();
                let r1 = self.fetch();
                let register = self.number_to_register(r1);
                self.set_register_value(register, literal);
            },
            MOV_REG_REG => {
                let from_register_number = self.fetch();
                let to_register_number = self.fetch();
                let from_register = self.number_to_register(from_register_number);
                let to_register = self.number_to_register(to_register_number);
                let value_from_register = self.get_register_value(from_register);
                self.set_register_value(to_register, value_from_register);
            },
            MOV_REG_MEM => {
                let from_register_number = self.fetch();
                let from_register = self.number_to_register(from_register_number);
                let memory_address = self.fetch_16();
                let value_from_register = self.get_register_value(from_register);
                self.memory.set_memory_u16(memory_address as usize, value_from_register);
            },
            MOV_MEM_REG => {
                let memory_address = self.fetch_16();
                let register_to_value = self.fetch();
                let register_to = self.number_to_register(register_to_value);
                let memory_value = self.memory.get_memory_u16(memory_address as usize);
                self.set_register_value(register_to, memory_value);
            },
            ADD_REG_REG => {
                let r1 = self.fetch();
                let r2 = self.fetch();
                let register_value_1 = self.u8_to_register_value(r1);
                let register_value_2 = self.u8_to_register_value(r2);
                let sum = register_value_1 + register_value_2;

                self.set_register_value(RegisterName::Acc, sum);
            },
            JMP_NOT_EQ => {
                let value = self.fetch_16();
                let address = self.fetch_16();

                if value != self.get_register_value(RegisterName::Acc) {
                    self.set_register_value(RegisterName::Ip, address);
                }
            }
            _ => panic!("Unknown instruction"),
        }
    }

    fn number_to_register(&mut self, num: u8) -> RegisterName {
        match num {
            0x00 => RegisterName::Ip,
            0x01 => RegisterName::Acc,
            0x02 => RegisterName::R1,
            0x03 => RegisterName::R2,
            0x04 => RegisterName::R3,
            0x05 => RegisterName::R4,
            0x06 => RegisterName::R5,
            0x07 => RegisterName::R6,
            0x08 => RegisterName::R7,
            0x09 => RegisterName::R8,
            _ => panic!("Unknown register value")
        }
    }

    fn u8_to_register_value(&mut self, num: u8) -> u16 {
        match num {
            0x01 => self.get_register_value(RegisterName::R1),
            0x02 => self.get_register_value(RegisterName::R2),
            _ => panic!("Unknown register value"),
        }
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }

    pub fn debug(&mut self) {
        for (key, value) in &self.register_map {
            println!("{} : {}", key, value);
        }
    }
}