#[cfg(test)]
mod test_jalr {
    use crate::cpu::CPU;
    use crate::cpu::instruction::builder::InstructionBuilder;
    use crate::cpu::register::{REG_S0, REG_S1};

    #[test]
    fn test_jalr() {
        let mut cpu = CPU::new();

        // Set PC and prepare instruction (rd = REG_S0, imm = 8)
        cpu.pc = 0x10;
        cpu.registers.set_register(REG_S1, 0x10);
        cpu.instruction = InstructionBuilder.jalr(8,REG_S1, REG_S0);

        // Execute JALR
        cpu.inst_jalr();

        // Verify results
        assert_eq!(cpu.registers.get_register(REG_S0), 0x14); // Return address
        assert_eq!(cpu.get_pc(), 0x28);             // New PC (0x10 + 8)
    }
}