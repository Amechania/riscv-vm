mod test_lui {
    use crate::cpu::CPU;
    use crate::cpu::instruction::builder::InstructionBuilder;
    use crate::cpu::opcodes::OP_LUI;
    use crate::cpu::register::REG_S0;

    #[test]
    fn test_lui() {
        let mut cpu = CPU::new();
        cpu.pc = 0x10;
        cpu.instruction = InstructionBuilder.lui(0x420, REG_S0);
        cpu.opcode = OP_LUI;

        // Execute LUI
        cpu.inst_lui();

        // Verify results
        assert_eq!(cpu.registers.get_register(REG_S0), 0x420000,
                   "\nexpected: 0x{:0>8x},\n\
        but got:  0x{:0>8x}",
                   0x420000, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }
}