mod builder;

use std::num::Wrapping;
use crate::cpu::*;
use crate::cpu::opcodes::*;
use crate::cpu::register::*;

#[allow(dead_code)]
impl CPU {

    fn inst_lui(&mut self) {
        let rd = ((self.instruction >> 7) & 0x1F) as u8;
        // LUI is a special case, it's an immediate, not an offset
        // The LUI instruction stores the 20-bit immediate
        // in the 20 most significant bits of the destination register.
        // The 12 least significant bits are set to zero.
        let imm = self.instruction & 0xFFFFF000;
        self.registers.set_register(rd, imm);
        self.pc += 4;
    }
    
    fn inst_jal(&mut self) {
        let rd = ((self.instruction >> 7) as u8) & 0x1F;

        // Immediate is split into parts, reconstruct it correctly
        let imm_20 = ((self.instruction >> 31) & 0x1) << 20; // Bit 20
        let imm_10_1 = ((self.instruction >> 21) & 0x3FF) << 1; // Bits 10:1
        let imm_11 = ((self.instruction >> 20) & 0x1) << 11; // Bit 11
        let imm_19_12 = ((self.instruction >> 12) & 0xFF) << 12; // Bits 19:12

        // Combine and sign-extend the immediate
        let imm = imm_20 | imm_19_12 | imm_11 | imm_10_1;

        self.registers.set_register(rd, self.pc + 4);

        self.pc = imm;
    }

    fn inst_jalr(&mut self) {
        let rd = ((self.instruction >> 7) as u8) & 0x1F;
        let rs1 = ((self.instruction >> 15) as u8) & 0x1F;
        let imm = (self.instruction >> 20) & 0xFFFFF;
        self.registers.set_register(rd, self.pc + 4);
        self.pc = self.pc + imm + self.registers.get_register(rs1);
    }

    fn inst_load(&mut self) {
        let rd = ((self.instruction >> 7) & 0x1F) as u8; // Bits 7:2
        let funct3 = ((self.instruction >> 12) & 0x7) as u8; // Bits 14:12
        let rs1 = ((self.instruction >> 15) & 0x1F) as u8; // Bits 15:11
        let imm = (self.instruction >> 20) & 0xFFF; // Bits 31:20

        match funct3 {
            F3_LW => {
                self.registers.set_register(rd, self.memory.get_u32(self.registers.get_register(rs1) + imm));
            }
            F3_LH => {
                self.registers.set_register(rd, self.memory.get_u16(self.registers.get_register(rs1) + imm + 2) as u32);
            }
            F3_LHU => {
                self.registers.set_register(rd, self.memory.get_u16(self.registers.get_register(rs1) + imm) as u32); // TODO: Verify
            }
            F3_LB => {
                self.registers.set_register(rd, self.memory.get_u8(self.registers.get_register(rs1) + imm + 3) as u32);
            }
            _ => {
                panic!("Invalid load instruction");
            }
        }
        self.pc += 4;
    }

    // TODO: Double check endianness
    fn inst_store(&mut self) {
        let funct3 = (self.instruction >> 12) as u8 & 0x7;
        let rs1 = (self.instruction >> 15) as u8 & 0x1F;
        let rs2 = (self.instruction >> 20) as u8 & 0x1F;
        let imm_11_5 = self.instruction >> 25 & 0x7F;
        let imm_4_0 = self.instruction >> 7 & 0x1F;
        let imm = imm_11_5 << 5 | imm_4_0;

        match funct3 {
            F3_SW => {
                self.memory.set_u32(self.registers.get_register(rs1) + imm, self.registers.get_register(rs2));
            }
            F3_SH => {
                self.memory.set_u16(self.registers.get_register(rs1) + imm, self.registers.get_register(rs2) as u16);
            }
            F3_SB => {
                self.memory.set_u8(self.registers.get_register(rs1) + imm, self.registers.get_register(rs2) as u8);
            }
            _ => {
                panic!("Invalid store instruction");
            }
        }

        self.pc += 4;
    }

    fn inst_branch(&mut self) {
        let rs1 = ((self.instruction >> 15) as u8) & 0x1F;
        let rs2 = ((self.instruction >> 20) as u8) & 0x1F;
        let funct3 = ((self.instruction >> 12) as u8) & 0x7;
        let imm_12 = ((self.instruction >> 31) & 0x1) << 12;
        let imm_11 = ((self.instruction >> 7) & 0x1) << 11;
        let imm_10_5 = ((self.instruction >> 25) & 0x7F) << 5;
        let imm_4_1 = ((self.instruction >> 8) & 0xF) << 1;
        let imm = imm_12 | imm_11 | imm_10_5 | imm_4_1;

        let condition:bool;

        match funct3 {
            F3_BEQ => {
                condition = self.registers.get_register(rs1) == self.registers.get_register(rs2);
            }
            F3_BNE => {
                condition = self.registers.get_register(rs1) != self.registers.get_register(rs2);
            }
            F3_BLT => {
                condition = self.registers.get_register(rs1) < self.registers.get_register(rs2);
            }
            F3_BGE => {
                condition = self.registers.get_register(rs1) >= self.registers.get_register(rs2);
            }
            F3_BLTU => {
                condition = self.registers.get_register(rs1) < self.registers.get_register(rs2);
            }
            F3_BGEU => {
                condition = self.registers.get_register(rs1) >= self.registers.get_register(rs2);
            }
            _ => {
                panic!("Invalid branch instruction");
            }
        }
        if condition {
            self.registers.set_register(REG_RA, self.pc + 4);
            self.pc = self.pc + imm;
        } else {
            self.pc += 4;
        }
    }

    fn inst_alui(&mut self) {
        let rd = ((self.instruction >> 7) & 0x1F) as u8;
        let funct3 = ((self.instruction >> 12) & 0x7) as u8;
        let rs1 = ((self.instruction >> 15) & 0x1F) as u8;
        let imm = (self.instruction >> 20) & 0xFFF;

        let rs1_value = self.registers.get_register(rs1);
        let result:u32;
        match funct3 {
            F3_ADDI => result = imm + rs1_value,
            F3_SLTI => {
                let mut imm_extended = imm as i32;
                // We need to sign extend the immediate
                if (imm & 0x800) != 0 {  // MSB is not set
                    imm_extended = imm_extended | (0xFFFF_F000u32 as i32);
                }
                result = ((rs1_value as i32) < imm_extended) as u32;
            }
            F3_SLTIU => {
                let mut imm_extended = imm;
                // We need to sign extend the immediate
                if (imm & 0x800) != 0 {  // MSB is not set
                    imm_extended = imm_extended | 0xFFFF_F000;
                }
                result = (rs1_value < imm_extended) as u32;
            }
            F3_XORI => result = rs1_value ^ imm,
            F3_ORI => result = rs1_value | imm,
            F3_ANDI => result = rs1_value & imm,
            F3_SLLI => result = rs1_value << imm,
            F3_SRLI_SRAI => {
                let slai_bit = ((self.instruction >> 30) as u8) & 0x1;
                if slai_bit == 0 { // SLRI
                    result = rs1_value >> imm;
                }
                else { // SLAI
                    // Here, we need to rotate, instead of shifting and zero filling
                    result = rs1_value.rotate_right(imm);
                }
            }
            _ => {
                panic!("Invalid alui instruction - funct3");
            }
        }
        self.registers.set_register(rd, result);
        self.pc += 4;
    }

    fn inst_alu(&mut self) {
        let rd = ((self.instruction >> 7) & 0x1F) as u8;
        let funct3 = ((self.instruction >> 12) & 0x7) as u8;
        let rs1 = ((self.instruction >> 15) & 0x1F) as u8;
        let rs2 = ((self.instruction >> 20) & 0x1F) as u8;
        let funct7 = ((self.instruction >> 25) & 0x7F) as u8;
        let funct73:u16 = ((funct7 as u16) << 3) | funct3 as u16;

        let rs1_value = self.registers.get_register(rs1);
        let rs2_value = self.registers.get_register(rs2);
        let result:u32;

        match funct73 {
            F73_ADD => result = rs1_value + rs2_value,
            F73_SUB => result = (Wrapping(rs1_value) - Wrapping(rs2_value)).0,
            F73_SLL => result = rs1_value << (rs2_value & 0x1F),
            F73_SLT => result = ( (rs1_value as i32) < (rs2_value as i32) ) as u32,
            F73_SLTU => result = ( rs1_value < rs2_value ) as u32,
            F73_XOR => result = rs1_value ^ rs2_value,
            F73_SRL => result = rs1_value >> (rs2_value & 0x1F),
            F73_SRA => result = rs1_value.rotate_right(rs2_value & 0x1F), // Only lower 5 bits of rs2 are used
            F73_OR => result = rs1_value | rs2_value,
            F73_AND => result = rs1_value & rs2_value,
            F73_MUL => result = (rs1_value as i32).wrapping_mul(rs2_value as i32) as u32, // Rust does not like multiplication overflows
            F73_MULH => { // We need to cast into larger signed int to get the upper 32 bits. We don't need to do this for MUL
                let result64 = (rs1_value as i32 as i64).wrapping_mul(rs2_value as i32 as i64);
                result = (result64 as u64 >> 32) as u32;
            },
            F73_MULHSU => { // Casting rs2 to unsigned larger int before casting to signed int, guarantees that the value is not signed
                let result64 = (rs1_value as i32 as i64).wrapping_mul(rs2_value as u64 as i64);
                result = (result64 >> 32) as u32;
            },
            F73_MULHU => {
                let result64 = (rs1_value as u64).wrapping_mul(rs2_value as u64);
                result = (result64 >> 32) as u32;
            },
            F73_DIV => {
                if rs2_value == 0 {
                    result = 0xFFFFFFFF;
                } else {
                    result = (rs1_value as i32 / rs2_value as i32) as u32;
                }
            },
            F73_DIVU => {
                if rs2_value == 0 {
                    result = 0xFFFFFFFF;
                } else {
                    result = rs1_value / rs2_value;
                }
            },
            F73_REM => {
                if rs2_value == 0 {
                    result = rs1_value;
                } else if (rs1_value == (i32::MIN as u32)) && ((rs2_value as i32) == -1) {
                    result = 0;
                } else {
                    result = (rs1_value as i32 % rs2_value as i32) as u32;
                }
            },
            F73_REMU => {
                if rs2_value == 0 {
                    result = rs1_value;
                } else {
                    result = rs1_value % rs2_value;
                }
            },
            _ => {
                panic!("Invalid alu instruction - funct3");
            }
        }

        self.registers.set_register(rd, result);
        self.pc += 4;
    }

    pub(crate) fn exec_inst(&mut self) -> bool {
        match self.opcode {
            OP_LUI => self.inst_lui(),
            OP_JAL => self.inst_jal(),
            OP_JALR => self.inst_jalr(),
            OP_BRANCH => self.inst_branch(),
            OP_LOAD => self.inst_load(),
            OP_STORE => self.inst_store(),
            OP_ALUI => self.inst_alui(),
            OP_ALU => self.inst_alu(),
            //OP_FENCE => self.inst_fence(),
            OP_E_C => return true,
            0x0 => return true,
            _ => panic!("Invalid opcode: 0b{:0>8b}", self.opcode),
        }
        false
    }
}

///// TESTS /////
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::cpu::*;
    use crate::cpu::instruction::builder::InstructionBuilder;
    use crate::cpu::opcodes::*;

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

    #[cfg(test)]
    mod test_alu {
        use crate::cpu::CPU;
        use crate::cpu::instruction::builder::InstructionBuilder;
        use crate::cpu::opcodes::*;
        use crate::cpu::register::*;

        #[test]
        fn test_add() {
            let mut cpu = CPU::new();
            cpu.registers.set_register(REG_S1, 0xCC33CC33);
            cpu.registers.set_register(REG_S0, 10);
            cpu.pc = 0x10;
            cpu.opcode = OP_ALU;
            cpu.instruction = InstructionBuilder.alu(F7_ADD, F3_ADD_SUB, REG_S1, REG_S0, REG_S0);

            // Execute load
            cpu.inst_alu();

            // Verify results
            let expected:u32 = 0xCC33CC3d;
            assert_eq!(cpu.registers.get_register(REG_S0), expected,
                       "Stored value was not correct!\
                       \nExpected: 0x{:0>8x},\
                       \nGot:      0x{:0>8x}",
                       expected, cpu.registers.get_register(REG_S0));
            assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
        }

        #[test]
        fn test_sub() {
            let mut cpu = CPU::new();
            cpu.registers.set_register(REG_S1, 0xCC33CC33);
            cpu.registers.set_register(REG_S0, 10);
            cpu.pc = 0x10;
            cpu.opcode = OP_ALU;
            cpu.instruction = InstructionBuilder.alu(F7_SUB, F3_ADD_SUB, REG_S0, REG_S1, REG_S0);

            // Execute load
            cpu.inst_alu();

            // Verify results
            let expected:u32 = 0xCC33CC29;
            assert_eq!(cpu.registers.get_register(REG_S0), expected,
                       "Stored value was not correct!\
                       \nExpected: 0x{:0>8x},\
                       \nGot:      0x{:0>8x}",
                       expected, cpu.registers.get_register(REG_S0));
            assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
        }

        #[test]
        fn test_sll() {
            let mut cpu = CPU::new();
            cpu.registers.set_register(REG_S1, 0xCC33CC33);
            cpu.registers.set_register(REG_S0, 8);
            cpu.pc = 0x10;
            cpu.opcode = OP_ALU;
            cpu.instruction = InstructionBuilder.alu(0, F3_SLL, REG_S0, REG_S1, REG_S0);

            // Execute load
            cpu.inst_alu();

            // Verify results
            let expected:u32 = 0x33CC3300;
            assert_eq!(cpu.registers.get_register(REG_S0), expected,
                       "Stored value was not correct!\
                       \nExpected: 0x{:0>8x},\
                       \nGot:      0x{:0>8x}",
                       expected, cpu.registers.get_register(REG_S0));
            assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
        }

        #[test]
        fn test_slt() {
            let mut cpu = CPU::new();
            cpu.registers.set_register(REG_S1, 0x419);
            cpu.registers.set_register(REG_S0, 0x420);
            cpu.pc = 0x10;
            cpu.opcode = OP_ALU;
            cpu.instruction = InstructionBuilder.alu(0, F3_SLT, REG_S0, REG_S1, REG_S0);

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
        fn test_sltu() {
            let mut cpu = CPU::new();
            cpu.registers.set_register(REG_S1, 0x421);
            cpu.registers.set_register(REG_S0, 0x420);
            cpu.pc = 0x10;
            cpu.opcode = OP_ALU;
            cpu.instruction = InstructionBuilder.alu(0, F3_SLTU, REG_S0, REG_S1, REG_S0);

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
        fn test_xor() {
            let mut cpu = CPU::new();
            cpu.registers.set_register(REG_S1, 0xCC33CC33);
            cpu.registers.set_register(REG_S0, 0xF00FF00F);
            cpu.pc = 0x10;
            cpu.opcode = OP_ALU;
            cpu.instruction = InstructionBuilder.alu(0, F3_XOR, REG_S0, REG_S1, REG_S0);

            // Execute load
            cpu.inst_alu();

            // Verify results
            let expected:u32 = 0x3C3C3C3C;
            assert_eq!(cpu.registers.get_register(REG_S0), expected,
                       "Stored value was not correct!\
                       \nExpected: 0x{:0>8x},\
                       \nGot:      0x{:0>8x}",
                       expected, cpu.registers.get_register(REG_S0));
            assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
        }

        #[test]
        fn test_srl() {
            let mut cpu = CPU::new();
            cpu.registers.set_register(REG_S1, 0xCC33CC33);
            cpu.registers.set_register(REG_S0, 0x8);
            cpu.pc = 0x10;
            cpu.opcode = OP_ALU;
            cpu.instruction = InstructionBuilder.alu(F7_SRL, F3_SRL_SLA, REG_S0, REG_S1, REG_S0);

            // Execute load
            cpu.inst_alu();

            // Verify results
            let expected:u32 = 0x00CC33CC;
            assert_eq!(cpu.registers.get_register(REG_S0), expected,
                       "Stored value was not correct!\
                       \nExpected: 0x{:0>8x},\
                       \nGot:      0x{:0>8x}",
                       expected, cpu.registers.get_register(REG_S0));
            assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
        }

        #[test]
        fn test_sra() {
            let mut cpu = CPU::new();
            cpu.registers.set_register(REG_S1, 0xCC33CC33);
            cpu.registers.set_register(REG_S0, 0x8);
            cpu.pc = 0x10;
            cpu.opcode = OP_ALU;
            cpu.instruction = InstructionBuilder.alu(F7_SRA, F3_SRL_SLA, REG_S0, REG_S1, REG_S0);

            // Execute load
            cpu.inst_alu();

            // Verify results
            let expected:u32 = 0x33CC33CC;
            assert_eq!(cpu.registers.get_register(REG_S0), expected,
                       "Stored value was not correct!\
                       \nExpected: 0x{:0>8x},\
                       \nGot:      0x{:0>8x}",
                       expected, cpu.registers.get_register(REG_S0));
            assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
        }

        #[test]
        fn test_or() {
            let mut cpu = CPU::new();
            cpu.registers.set_register(REG_S1, 0xCC33CC33);
            cpu.registers.set_register(REG_S0, 0x330000CC);
            cpu.pc = 0x10;
            cpu.opcode = OP_ALU;
            cpu.instruction = InstructionBuilder.alu(0, F3_OR, REG_S0, REG_S1, REG_S0);

            // Execute load
            cpu.inst_alu();

            // Verify results
            let expected:u32 = 0xFF33CCFF;
            assert_eq!(cpu.registers.get_register(REG_S0), expected,
                       "Stored value was not correct!\
                       \nExpected: 0x{:0>8x},\
                       \nGot:      0x{:0>8x}",
                       expected, cpu.registers.get_register(REG_S0));
            assert_eq!(cpu.pc, 0x14, "PC was not updated correctly!");
        }

        #[test]
        fn test_and() {
            let mut cpu = CPU::new();
            cpu.registers.set_register(REG_S1, 0xCC33CC33);
            cpu.registers.set_register(REG_S0, 0x10);
            cpu.pc = 0x10;
            cpu.opcode = OP_ALU;
            cpu.instruction = InstructionBuilder.alu(0, F3_AND, REG_S0, REG_S1, REG_S0);

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
    }
}