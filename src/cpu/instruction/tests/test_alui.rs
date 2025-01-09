#[cfg(test)]
mod test_alui {
    use crate::cpu::CPU;
    use crate::cpu::instruction::builder::InstructionBuilder;
    use crate::cpu::opcodes::*;
    use crate::cpu::register::*;

    fn prep_alui_inst(cpu: &mut CPU, funct3: u8, rs1: u32, rd: u8, imm: u32) {
        cpu.registers.set_register(REG_S1, rs1);
        cpu.pc = 0x10;
        cpu.opcode = OP_ALUI;
        cpu.instruction = InstructionBuilder.alui(imm, funct3, REG_S1, rd);
    }

    #[test]
    fn test_addi() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_ADDI, 0x420, REG_S0, 0x420);
        cpu.inst_alui();
        let expected = 0x840;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_slti_yes() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SLTI, 0x419, REG_S0, 0x420);
        cpu.inst_alui();
        let expected = 1;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_slti_no_eq() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SLTI, 0x420, REG_S0, 0x420);
        cpu.inst_alui();
        let expected = 0;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_slti_no_gt() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SLTI, 0x421, REG_S0, 0x420);
        cpu.inst_alui();
        let expected = 0;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_sltiu_yes() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SLTIU, 0x419, REG_S0, 0x420);
        cpu.inst_alui();
        let expected = 1;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_sltiu_no_eq() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SLTIU, 0x420, REG_S0, 0x420);
        cpu.inst_alui();
        let expected = 0;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_sltiu_no_gt() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SLTIU, 0x421, REG_S0, 0x420);
        cpu.inst_alui();
        let expected = 0;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_xori() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_XORI, 0x400, REG_S0, 0x420);
        cpu.inst_alui();
        let expected = 0x20;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_ori() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_ORI, 0x400, REG_S0, 0x420);
        cpu.inst_alui();
        let expected = 0x420;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_andi() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_ANDI, 0x400, REG_S0, 0x420);
        cpu.inst_alui();
        let expected = 0x400;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_slli() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SLLI, 0x400, REG_S0, 0x1);
        cpu.inst_alui();
        let expected = 0x800;
        assert_eq!(cpu.registers.get_register(REG_S0), 0x800,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_slli_overflow() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SLLI, 0x80_00_00_00, REG_S0, 0x1);
        cpu.inst_alui();
        let expected = 0x0;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
           "\nexpected: 0x{:0>8x},\n\
           but got:  0x{:0>8x}",
           expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_srli() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SRLI_SRAI, 0x401, REG_S0, 0x1);
        cpu.inst_alui();
        let expected = 0x200;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nexpected: 0x{:0>8x},\n\
            but got:  0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_srli_underflow() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SRLI_SRAI, 0x1, REG_S0, 0x1);
        cpu.inst_alui();
        let expected = 0x0;
        assert_eq!(cpu.registers.get_register(REG_S0), 0x0,
               "\nSRLI should NOT underflow to: 0x{:0>8x},\n\
               but instead returned:         0x{:0>8x}",
               expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_srai() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SRLI_SRAI, 0x400, REG_S0, 0x1);
        // Set the SRAI bit (bit 30)
        cpu.instruction = cpu.instruction | (0x1 << 30);
        cpu.inst_alui();
        assert_eq!(cpu.registers.get_register(REG_S0), 0x200);
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }

    #[test]
    fn test_srai_overflow() {
        let mut cpu = CPU::new();
        prep_alui_inst(&mut cpu, F3_SRLI_SRAI, 0x0000_03b1, REG_S0, 0x4);
        // Set the SRAI bit (bit 30)
        cpu.instruction = cpu.instruction | (0x1 << 30);
        cpu.inst_alui();

        let expected = 0x1000_003b;
        assert_eq!(cpu.registers.get_register(REG_S0), expected,
            "\nSRAI should overflow to: 0x{:0>8x},\n\
            but instead returned:    0x{:0>8x}",
            expected, cpu.registers.get_register(REG_S0));
        assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
    }
}