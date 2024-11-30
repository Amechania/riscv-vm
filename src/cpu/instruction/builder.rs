// RISC-V Tiny VM - Ivi Ballou / Amechania

use crate::cpu::opcodes::*;

#[allow(dead_code)]
pub struct InstructionBuilder;

#[allow(dead_code)]
impl InstructionBuilder {

    pub fn lui(&self, imm: u32, rd: u8) -> u32 {
        imm << 12
        | ((rd as u32) << 7)
        | OP_LUI as u32
    }
    
    pub fn jal(&self, address: u32, rd: u8) -> u32 {
        let imm_encoded = ((address & 0x80000) >> 20)        // Bit 20
            | ((address & 0xFF000) >> 12)                   // Bits 19:12
            | ((address & 0x800) << 9)                      // Bit 11
            | ((address & 0x7FE) << 20);                    // Bits 10:1
        imm_encoded
        | ((rd as u32) << 7)
        | OP_JAL as u32
    }

    pub fn jalr(&self, offset: u32, rs1: u8, rd: u8) -> u32 {
        let imm_encoded = ((offset & 0x80000) >> 20)     // Bit 20
            | ((offset & 0xFF000) >> 12)   // Bits 19:12
            | ((offset & 0x800) << 9)      // Bit 11
            | ((offset & 0x7FE) << 20);    // Bits 10:1
        imm_encoded // We don't offset imm here, as it already is offset
        | (rs1 as u32) << 15
        | (rd as u32) << 7
        | OP_JALR as u32
    }

    pub fn load(&self, address: u32, funct3: u8, rd: u8) -> u32 {
        (address << 20)
        | (funct3 as u32) << 12
        | (rd as u32) << 7
        | OP_LOAD as u32
    }

    pub fn store(&self, address: u32, funct3: u8, rs2: u8, rs1: u8) -> u32 {
        let imm_11_5 = (address >> 5) & 0x7F;
        let imm_4_0 = address & 0x1F;

        imm_11_5 << 25
        | (rs2 as u32) << 20
        | (rs1 as u32) << 15
        | (funct3 as u32) << 12
        | imm_4_0 << 7
        | OP_STORE as u32
    }

    pub fn branch(&self, offset: u32, funct3: u8, rs2: u8, rs1: u8) -> u32 {
        let imm_12 = (offset >> 12) & 0x1;
        let imm_11 = (offset >> 11) & 0x1;
        let imm_10_5 = (offset >> 5) & 0x3F;
        let imm_4_1 = (offset >> 1) & 0xF;

        imm_12 << 31
        | imm_10_5 << 25
        | (rs2 as u32) << 20
        | (rs1 as u32) << 15
        | (funct3 as u32) << 12
        | imm_4_1 << 8
        | imm_11 << 7
        | OP_BRANCH as u32
    }

    pub fn alui(&self, imm: u32, funct3: u8, rs1: u8, rd: u8) -> u32 {
        (imm & 0xFFF) << 20
        | (rs1 as u32) << 15
        | (funct3 as u32) << 12
        | (rd as u32) << 7
        | OP_ALUI as u32
    }

    pub fn alu(&self, funct7: u8, funct3: u8, rs2: u8, rs1: u8, rd: u8) -> u32 {
        (funct7 as u32) << 25
        | (rs2 as u32) << 20
        | (rs1 as u32) << 15
        | (funct3 as u32) << 12
        | (rd as u32) << 7
        | OP_ALU as u32
    }
}