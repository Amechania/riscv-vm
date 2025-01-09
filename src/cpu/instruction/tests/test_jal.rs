#[cfg(test)]
mod test_jal {
    use crate::cpu::CPU;
    use crate::cpu::instruction::builder::InstructionBuilder;
    use crate::cpu::opcodes::OP_JAL;
    use crate::cpu::register::REG_S0;

    #[test]
    fn test_jal() {
        let mut cpu = CPU::new();

        // Set PC and prepare instruction (rd = REG_S0, imm = 8)
        cpu.pc = 0x10;
        cpu.instruction = InstructionBuilder.jal(8, REG_S0);
        cpu.opcode = OP_JAL;

        // Execute JAL
        cpu.inst_jal();

        // Verify results
        assert_eq!(cpu.registers.get_register(REG_S0), 0x14); // Return address
        assert_eq!(cpu.get_pc(), 0x8);             // New PC (0x10 + 8)
    }
}