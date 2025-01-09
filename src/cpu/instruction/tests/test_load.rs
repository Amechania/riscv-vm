#[cfg(test)]
mod test_load {
    use crate::cpu::CPU;
    use crate::cpu::instruction::builder::InstructionBuilder;
    use crate::cpu::opcodes::*;
    use crate::cpu::register::*;

    #[test]
    fn test_load_word() {
        let mut cpu = CPU::new();
        let address = 0x50;
        cpu.memory.set_u32(0x50, 0xCC33CC33);
        cpu.pc = 0x10;
        cpu.opcode = OP_LOAD;
        cpu.instruction = InstructionBuilder.load(address, F3_LW, REG_S0);

        // Execute load
        cpu.inst_load();

        // Verify results
        // word at address is 0b11001100_11001100_00110011_00110011
        assert_eq!(cpu.registers.get_register(REG_S0), 0xCC33CC33
            , "Loaded value was not correct!\
            \nExpected: 0xCC33CC33,\
            \nGot:      0x{:0>8x}",
            cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_load_half_word() {
        let mut cpu = CPU::new();
        let address = 0x50;
        cpu.memory.set_u32(0x50, 0xCC33CC33);
        cpu.pc = 0x10;
        cpu.opcode = OP_LOAD;
        cpu.instruction = InstructionBuilder.load(address, F3_LH, REG_S0);

        // Execute load
        cpu.inst_load();

        // Verify results (half word at address is 0b11001100_11001100)
        assert_eq!(cpu.registers.get_register(REG_S0), 0xCC33
            , "Loaded value was not correct!\
            \nExpected: 0xCC33,\
            \nGot:      0x{:0>4x}",
            cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_load_byte() {
        let mut cpu = CPU::new();
        let address = 0x50;
        cpu.memory.set_u32(0x50, 0xCC33CC33);
        cpu.pc = 0x10;
        cpu.opcode = OP_LOAD;
        cpu.instruction = InstructionBuilder.load(address, F3_LB, REG_S0);

        // Execute load
        cpu.inst_load();

        // Verify results (byte at address is 0b11001100)
        assert_eq!(cpu.registers.get_register(REG_S0), 0xCC
            , "Loaded value was not correct!\
            \nExpected: 0x33,\
            \nGot:      0x{:0>2x}",
            cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }
}