#[cfg(test)]
mod test_branch {
    use crate::cpu::CPU;
    use crate::cpu::instruction::builder::InstructionBuilder;
    use crate::cpu::opcodes::*;
    use crate::cpu::register::*;

    fn prep_branch_inst(cpu: &mut CPU, funct3: u8, rs1: u32, rs2: u32) {
        let offset: u32 = 0x108;
        cpu.registers.set_register(REG_S1, rs1);
        cpu.registers.set_register(REG_S2, rs2);
        cpu.pc = 0x10;
        cpu.opcode = OP_BRANCH;
        cpu.instruction = InstructionBuilder.branch(offset, funct3, REG_S2, REG_S1);
    }

    #[test]
    fn test_beq_yes() {
        let mut cpu = CPU::new();
        prep_branch_inst(&mut cpu, F3_BEQ, 0x420, 0x420);
        cpu.inst_branch();
        assert_eq!(cpu.pc, 0x118, "PC was not updated correctly!");
        assert_eq!(cpu.registers.get_register(REG_RA), 0x14);
    }

    #[test]
    fn test_beq_no() {
        let mut cpu = CPU::new();
        prep_branch_inst(&mut cpu, F3_BEQ, 0x420, 0x421);
        cpu.inst_branch();
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_bne_yes() {
        let mut cpu = CPU::new();
        prep_branch_inst(&mut cpu, F3_BNE, 0x420, 0x421);
        cpu.inst_branch();
        assert_eq!(cpu.pc, 0x118, "PC was not updated correctly!");
        assert_eq!(cpu.registers.get_register(REG_RA), 0x14);
    }

    #[test]
    fn test_bne_no() {
        let mut cpu = CPU::new();
        prep_branch_inst(&mut cpu, F3_BNE, 0x420, 0x420);
        cpu.inst_branch();
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_blt_yes() {
        let mut cpu = CPU::new();
        prep_branch_inst(&mut cpu, F3_BLT, 0x41F, 0x420);
        cpu.inst_branch();
        assert_eq!(cpu.pc, 0x118, "PC was not updated correctly!");
        assert_eq!(cpu.registers.get_register(REG_RA), 0x14);
    }

    #[test]
    fn test_blt_no() {
        let mut cpu = CPU::new();
        prep_branch_inst(&mut cpu, F3_BLT, 0x420, 0x420);
        cpu.inst_branch();
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_bge_yes_gt() {
        let mut cpu = CPU::new();
        prep_branch_inst(&mut cpu, F3_BGE, 0x422, 0x420);
        cpu.inst_branch();
        assert_eq!(cpu.pc, 0x118, "PC was not updated correctly!");
        assert_eq!(cpu.registers.get_register(REG_RA), 0x14);
    }

    #[test]
    fn test_bge_yes_eq() {
        let mut cpu = CPU::new();
        prep_branch_inst(&mut cpu, F3_BGE, 0x420, 0x420);
        cpu.inst_branch();
        assert_eq!(cpu.pc, 0x118, "PC was not updated correctly!");
        assert_eq!(cpu.registers.get_register(REG_RA), 0x14);
    }

    #[test]
    fn test_bge_no() {
        let mut cpu = CPU::new();
        prep_branch_inst(&mut cpu, F3_BGE, 0x419, 0x420);
        cpu.inst_branch();
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }
}