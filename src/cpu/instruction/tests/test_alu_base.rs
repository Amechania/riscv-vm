use crate::cpu::CPU;
use crate::cpu::instruction::builder::InstructionBuilder;
use crate::cpu::opcodes::{F3_ADD_SUB, F3_AND, F3_OR, F3_SLL, F3_SLT, F3_SLTU, F3_SRL_SLA, F3_XOR, F7_ADD, F7_SRA, F7_SRL, F7_SUB, OP_ALU};
use crate::cpu::register::{REG_S0, REG_S1};

#[test]
fn test_add() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xCC33CC33);
    cpu.registers.set_register(REG_S0, 10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_ADD, F3_ADD_SUB, REG_S1, REG_S0, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xCC33CC3d;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_sub() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xCC33CC33);
    cpu.registers.set_register(REG_S0, 10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_SUB, F3_ADD_SUB, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xCC33CC29;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_sll() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xCC33CC33);
    cpu.registers.set_register(REG_S0, 8);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(0, F3_SLL, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x33CC3300;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_slt() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x419);
    cpu.registers.set_register(REG_S0, 0x420);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(0, F3_SLT, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 1;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_sltu() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x421);
    cpu.registers.set_register(REG_S0, 0x420);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(0, F3_SLTU, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_xor() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xCC33CC33);
    cpu.registers.set_register(REG_S0, 0xF00FF00F);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(0, F3_XOR, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x3C3C3C3C;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_srl() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xCC33CC33);
    cpu.registers.set_register(REG_S0, 0x8);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_SRL, F3_SRL_SLA, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x00CC33CC;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_sra() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xCC33CC33);
    cpu.registers.set_register(REG_S0, 0x8);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_SRA, F3_SRL_SLA, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x33CC33CC;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_or() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xCC33CC33);
    cpu.registers.set_register(REG_S0, 0x330000CC);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(0, F3_OR, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xFF33CCFF;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_and() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xCC33CC33);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(0, F3_AND, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x10;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

