use crate::cpu::CPU;
use crate::cpu::instruction::builder::InstructionBuilder;
use crate::cpu::opcodes::{F3_DIV, F3_DIVU, F7_M_EXTENSION, OP_ALU};
use crate::cpu::register::{REG_S0, REG_S1};

#[test]
fn test_div() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_DIV, REG_S0, REG_S1, REG_S0);

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
fn test_div_non_perfect() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x14);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_DIV, REG_S0, REG_S1, REG_S0);

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
fn test_div_negative_dividend() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0xFFFFFFFF);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_DIV, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xFFFFFFF0;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_div_negative_divisor() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xFFFFFFFF);
    cpu.registers.set_register(REG_S0, 0x1);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_DIV, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xFFFFFFFF;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_div_negative_double() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xFFFFFFFC);
    cpu.registers.set_register(REG_S0, 0xFFFFFFFE);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_DIV, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x2;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_div_complex() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x840);
    cpu.registers.set_register(REG_S0, 0x1F4);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_DIV, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x4;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_div_zero_dividend() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x0);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_DIV, REG_S0, REG_S1, REG_S0);

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
fn test_div_zero_divisor() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0x0);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_DIV, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xFFFFFFFF;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_divu() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x14);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_DIVU, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x1;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_divu_zero_dividend() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x0);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_DIVU, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x0;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_divu_zero_divisor() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0x0);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_DIVU, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xFFFFFFFF;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}