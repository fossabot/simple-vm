use vm::instruction::*;
use vm::memory::Memory;
use vm::cpu::*;

#[test]
fn should_be_able_to_get_register_value() {
    let mut memory = Memory::new(10);
    let mut cpu = CPU::new(&mut memory);

    let acc = cpu.get_register_value(RegisterName::Acc);

    assert_eq!(acc, 0);
}

#[test]
fn should_be_able_to_set_register_value() {
    let mut memory = Memory::new(10);
    let mut cpu = CPU::new(&mut memory);

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

    let mut cpu = CPU::new(&mut memory);
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

    let mut cpu = CPU::new(&mut memory);
    cpu.set_register_value(RegisterName::Ip, 0);

    let instruction_fetched = cpu.fetch_16();

    assert_eq!(instruction_fetched, 0x1234);
    assert_eq!(cpu.get_register_value(RegisterName::Ip), 2);
}

#[test]
fn should_execute_mov_lit_reg() {
    let mut memory = Memory::new(10);
    memory.set_memory(0, MOV_LIT_REG);
    memory.set_memory(1, 0x00);
    memory.set_memory(2, 0x01);
    memory.set_memory(3, 2);

    let mut cpu = CPU::new(&mut memory);
    cpu.set_register_value(RegisterName::Ip, 0);

    cpu.step();

    assert_eq!(cpu.get_register_value(RegisterName::R1), 0x0001);
    assert_eq!(cpu.get_register_value(RegisterName::Ip), 4);
}

#[test]
fn should_execute_mov_reg_reg() {
    let mut memory = Memory::new(10);
    memory.set_memory(0, MOV_REG_REG);
    memory.set_memory(1, 0x02);
    memory.set_memory(2, 0x03);

    let mut cpu = CPU::new(&mut memory);
    cpu.set_register_value(RegisterName::R1, 1);
    cpu.set_register_value(RegisterName::R2, 2);

    cpu.step();

    assert_eq!(cpu.get_register_value(RegisterName::R1), 0x0001);
    assert_eq!(cpu.get_register_value(RegisterName::R2), 0x0001);
    assert_eq!(cpu.get_register_value(RegisterName::Ip), 3);
}

#[test]
fn should_execute_mov_reg_mem() {
    let mut memory = Memory::new(10);
    memory.set_memory(0, MOV_REG_MEM);
    memory.set_memory(1, 0x02);
    memory.set_memory(2, 0x00);
    memory.set_memory(3, 0x05);

    memory.set_memory(5, 0x00);
    memory.set_memory(6, 0x00);

    let mut cpu = CPU::new(&mut memory);
    cpu.set_register_value(RegisterName::R1, 0xFFFF);
    

    cpu.step();
    let r1 = cpu.get_register_value(RegisterName::R1);
    let ip = cpu.get_register_value(RegisterName::Ip);

    let expected = memory.get_memory_u16(5);

    assert_eq!(expected, 0xFFFF);
    assert_eq!(r1, 0xFFFF);
    assert_eq!(ip, 4);
}

#[test]
fn should_execute_add_reg_reg() {
    let mut memory = Memory::new(10);
    memory.set_memory(0, ADD_REG_REG);
    memory.set_memory(1, 0x01);
    memory.set_memory(2, 0x02);

    let mut cpu = CPU::new(&mut memory);
    cpu.set_register_value(RegisterName::R1, 5);
    cpu.set_register_value(RegisterName::R2, 6);

    cpu.step();

    assert_eq!(cpu.get_register_value(RegisterName::Acc), 11);
    assert_eq!(cpu.get_register_value(RegisterName::Ip), 3);
}

#[test]
fn should_execute_mov_mem_to_reg() {
    let mut memory = Memory::new(10);
    memory.set_memory(0, MOV_MEM_REG);
    memory.set_memory(1, 0x00);
    memory.set_memory(2, 0x04);
    memory.set_memory(3, 0x02);
    memory.set_memory(4, 0x12);
    memory.set_memory(5, 0x34);

    let mut cpu = CPU::new(&mut memory);
    cpu.set_register_value(RegisterName::R1, 1);

    cpu.step();

    assert_eq!(cpu.get_register_value(RegisterName::R1), 0x1234);
    assert_eq!(cpu.get_register_value(RegisterName::Ip), 4);
}

#[test]
fn should_jump_if_value_not_equal_to_acc() {
    let mut memory = Memory::new(10);
    memory.set_memory(0, JMP_NOT_EQ);
    memory.set_memory(1, 0x12);
    memory.set_memory(2, 0x34);
    memory.set_memory(3, 0x00);
    memory.set_memory(4, 0x08);
    memory.set_memory(5, 0x34);

    let mut cpu = CPU::new(&mut memory);
    cpu.set_register_value(RegisterName::Acc, 1);

    cpu.step();

    assert_eq!(cpu.get_register_value(RegisterName::Ip), 0x0008);
}
