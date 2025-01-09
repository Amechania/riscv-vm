use crate::cpu::CPU;
use crate::cpu::instruction::builder::InstructionBuilder;
use crate::cpu::opcodes::{F3_REM, F3_REMU, F7_M_EXTENSION, OP_ALU};
use crate::cpu::register::{REG_S0, REG_S1};

#[test]
fn test_rem() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_REM, REG_S0, REG_S1, REG_S0);

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
fn test_rem_zero_dividend() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x0);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_REM, REG_S0, REG_S1, REG_S0);

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
fn test_rem_zero_divisor() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0x0);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_REM, REG_S0, REG_S1, REG_S0);

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

#[test]
fn test_rem_negative_dividend() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xFFFF0000);
    cpu.registers.set_register(REG_S0, 0xA);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_REM, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xFFFFFFFA;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_rem_negative_divisor() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0xFFFFFFFD);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_REM, REG_S0, REG_S1, REG_S0);

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
fn test_rem_negative_double() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xFFFF0000);
    cpu.registers.set_register(REG_S0, 0xFFFFFFF5);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_REM, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0xFFFFFFF7;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
               \nExpected: 0x{:0>8x},\
               \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}

#[test]
fn test_rem_overflow() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xF0000000);
    cpu.registers.set_register(REG_S0, 0xFFFFFFFF);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_REM, REG_S0, REG_S1, REG_S0);

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
fn test_remu() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_REMU, REG_S0, REG_S1, REG_S0);

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
fn test_remu_zero_dividend() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x0);
    cpu.registers.set_register(REG_S0, 0x10);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_REMU, REG_S0, REG_S1, REG_S0);

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
fn test_remu_zero_divisor() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0x10);
    cpu.registers.set_register(REG_S0, 0x0);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_REMU, REG_S0, REG_S1, REG_S0);

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

#[test]
fn test_remu_large_dividend() {
    let mut cpu = CPU::new();
    cpu.registers.set_register(REG_S1, 0xF0000000);
    cpu.registers.set_register(REG_S0, 0x15);
    cpu.pc = 0x10;
    cpu.opcode = OP_ALU;
    cpu.instruction = InstructionBuilder.alu(F7_M_EXTENSION, F3_REMU, REG_S0, REG_S1, REG_S0);

    // Execute load
    cpu.inst_alu();

    // Verify results
    let expected:u32 = 0x9;
    assert_eq!(cpu.registers.get_register(REG_S0), expected,
               "Stored value was not correct!\
                   \nExpected: 0x{:0>8x},\
                   \nGot:      0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
    assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
}