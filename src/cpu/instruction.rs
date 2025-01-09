mod builder;
mod tests;

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