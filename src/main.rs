pub mod cpu;
pub mod instruction;
pub mod memory;

use crate::cpu::CPU;
use crate::instruction::*;
use crate::memory::Memory;

fn main() {
    let mut memory = Memory::new(10);
    memory.set_memory(0, MOV_LIT_R1);
    memory.set_memory(1, 0x12);
    memory.set_memory(2, 0x34);

    memory.set_memory(3, MOV_LIT_R2);
    memory.set_memory(4, 0xAB);
    memory.set_memory(5, 0xCD);

    memory.set_memory(6, ADD_REG_REG);
    memory.set_memory(7, 0x01);
    memory.set_memory(8, 0x02);

    let mut cpu = CPU::new(memory);

    cpu.debug();

    cpu.step();
    cpu.debug();
    cpu.step();
    cpu.debug();
    cpu.step();
    cpu.debug();
}
