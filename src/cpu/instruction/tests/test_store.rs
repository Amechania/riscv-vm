#[cfg(test)]
mod test_store {
    use crate::cpu::CPU;
    use crate::cpu::instruction::builder::InstructionBuilder;
    use crate::cpu::opcodes::*;
    use crate::cpu::register::*;

    #[test]
    fn test_store_word() {
        let mut cpu = CPU::new();
        cpu.registers.set_register(REG_S1, 0xCC33CC33);
        cpu.registers.set_register(REG_S0, 10);
        cpu.pc = 0x10;
        cpu.opcode = OP_STORE;

        // WORD

        cpu.instruction = InstructionBuilder.store(0x550, F3_SW, REG_S1, REG_S0);

        // Execute load
        cpu.inst_store();

        // Verify results
        // word at 0x55A is 0b11001100_11001100_00110011_00110011
        assert_eq!(cpu.memory.get_u32(0x55A), 0xCC33CC33
            , "Stored value was not correct!\
            \nExpected: 0xCC33CC33,\
            \nGot:      0b{:0>8x}",
            cpu.memory.get_u32(0x55A));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");

        cpu.opcode = OP_STORE;
        cpu.instruction = InstructionBuilder.store(0x554, F3_SH, REG_S1, REG_S0);

    }

    #[test]
    fn test_store_half_word() {
        let mut cpu = CPU::new();
        cpu.registers.set_register(REG_S1, 0xCC33CC33);
        cpu.registers.set_register(REG_S0, 10);
        cpu.pc = 0x10;
        cpu.opcode = OP_STORE;

        // WORD

        cpu.instruction = InstructionBuilder.store(0x554, F3_SH, REG_S1, REG_S0);

        // Execute load
        cpu.inst_store();

        // Verify results (half word at 0x55E is 0b11001100_11001100)
        assert_eq!(cpu.memory.get_u16(0x55E), 0xCC33
                   , "Stored value was not correct!\
            \nExpected: 0x3333,\
            \nGot:      0x{:0>4x}",
                   cpu.memory.get_u16(0x55E));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_store_byte() {
        let mut cpu = CPU::new();
        cpu.registers.set_register(REG_S1, 0xCC33CC33);
        cpu.registers.set_register(REG_S0, 10);
        cpu.pc = 0x10;
        cpu.opcode = OP_STORE;

        // WORD

        cpu.instruction = InstructionBuilder.store(0x558, F3_SB, REG_S1, REG_S0);

        // Execute load
        cpu.inst_store();

        // Verify results (byte at 0x562 is 0b11001100)
        assert_eq!(cpu.memory.get_u8(0x562), 0x33
                   , "Stored value was not correct!\
            \nExpected: 0x33,\
            \nGot:      0b{:0>2x}",
                   cpu.memory.get_u8(0x562));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }
}