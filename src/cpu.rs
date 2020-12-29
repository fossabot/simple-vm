use std::collections::HashMap;

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
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
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
        CPU { registers }
    }

    pub fn get_register_value(&mut self, name: RegisterName) -> u16 {
        return *self.registers.get(&name).unwrap_or(&0);
    }

    pub fn set_register_value(&mut self, name: RegisterName, value: u16) {
        self.registers.insert(name, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_get_register_value() {
        let mut cpu = CPU::new();

        let acc = cpu.get_register_value(RegisterName::Acc);

        assert_eq!(acc, 0);
    }

    #[test]
    fn should_be_able_to_set_register_value() {
        let mut cpu = CPU::new();

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
}
