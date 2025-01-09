use crate::cpu::CPU;
use crate::cpu::instruction::builder::InstructionBuilder;
use crate::cpu::opcodes::{F3_MUL, F3_MULH, F3_MULHSU, F3_MULHU, F7_M_EXTENSION, OP_ALU};
use crate::cpu::register::{REG_S0, REG_S1};

#[test]
fn test_mul()
{
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MUL, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x100;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_mul_double_negative()
{
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xFFFFFFFF);
    cpu.registers.set_register(REG_S0, 0xFFFFFFFE);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MUL, REG_S0, REG_S1, REG_S0);

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
fn test_mul_single_negative()
{
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xFFFFFFFE);
    cpu.registers.set_register(REG_S0, 0x2);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MUL, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = -4i32 as u32;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_mul_overflow()
{
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10000000);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MUL, REG_S0, REG_S1, REG_S0);

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
fn test_mulh() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULH, REG_S0, REG_S1, REG_S0);

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
fn test_mulh_double_negative()
{
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xFFFFFFFF);
    cpu.registers.set_register(REG_S0, 0xFFFFFFFE);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULH, REG_S0, REG_S1, REG_S0);

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
fn test_mulh_single_negative()
{
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xFFFFFFFE);
    cpu.registers.set_register(REG_S0, 0x2);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULH, REG_S0, REG_S1, REG_S0);

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
fn test_mulh_overflow_to_higher()
{
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x1000000);
    cpu.registers.set_register(REG_S0, 0x100);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULH, REG_S0, REG_S1, REG_S0);

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
fn test_mulh_overflow()
{
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xFFFFFFFF);
    cpu.registers.set_register(REG_S0, 0xFFFFFFFF);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULH, REG_S0, REG_S1, REG_S0);

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
fn test_mulhsu() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULHSU, REG_S0, REG_S1, REG_S0);

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
fn test_mulhsu_negative_small_int() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xC4653600);
    cpu.registers.set_register(REG_S0, 0x3B9ACA00);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULHSU, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xF21F494C;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_mulhsu_negative_big_int() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xC4653600);
    cpu.registers.set_register(REG_S0, 0xC4653600);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULHSU, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xD245ECB3;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_mulhsu_positive_big_int() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x3B9ACA00);
    cpu.registers.set_register(REG_S0, 0xC4653600);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULHSU, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x2DBA134C;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_mulhu() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULHU, REG_S0, REG_S1, REG_S0);

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
fn test_mulhu_high_ls() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x1A2B7F0D);
    cpu.registers.set_register(REG_S0, 0x10000000);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULHU, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x01A2B7F0;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_mulhu_ls_high() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10000000);
    cpu.registers.set_register(REG_S0, 0x1A2B7F0D);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULHU, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x01A2B7F0;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_mulhu_high_high() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xEE6B2800);
    cpu.registers.set_register(REG_S0, 0xEE6B2800);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_MULHU, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xDE0B6B3A;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}