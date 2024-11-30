// RISC-V Tiny VM - Ivi Ballou / Amechania

// 32-bit RISC-V instructions

// Opcodes
#![allow(dead_code)]
pub(crate) const OP_LUI: u8 =  0x37; // LUI
pub(crate) const OP_AUIPC: u8 = 0x17; // AUIPC
pub(crate) const OP_JAL: u8 = 0x6F; // JAL
pub(crate) const OP_JALR: u8 = 0x67; // JALR
pub(crate) const OP_BRANCH: u8 = 0x63; // BEQ, BNE, BLT, BGE, BLTU, BGEU
pub(crate) const OP_LOAD: u8 = 0x03; // LB, LH, LW, LBU, LHU
pub(crate) const OP_STORE: u8 = 0x02; // SB, SH, SW
pub(crate) const OP_ALUI: u8 = 0x13; // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SRLI, SRAI
pub(crate) const OP_ALU: u8 = 0x33; // ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND
pub(crate) const OP_FENCE: u8 = 0x0F; // FENCE, FENCE.I
pub(crate) const OP_E_C: u8 = 0x73; // ECALL, EBREAK, CSRRW, CSRRS, CSRRC, CSRRWI, CSRRSI, CSRRCI

// Function 3 Codes
pub(crate) const F3_BEQ: u8 = 0x00;
pub(crate) const F3_BNE: u8 = 0x01;
pub(crate) const F3_BLT: u8 = 0x04;
pub(crate) const F3_BGE: u8 = 0x05;
pub(crate) const F3_BLTU: u8 = 0x06;
pub(crate) const F3_BGEU: u8 = 0x07;

pub(crate) const F3_LB: u8 = 0x00;
pub(crate) const F3_LH: u8 = 0x01;
pub(crate) const F3_LW: u8 = 0x02;
pub(crate) const F3_LBU: u8 = 0x04;
pub(crate) const F3_LHU: u8 = 0x05;

pub(crate) const F3_SB: u8 = 0x00;
pub(crate) const F3_SH: u8 = 0x01;
pub(crate) const F3_SW: u8 = 0x02;

pub(crate) const F3_ADDI: u8 = 0x00;
pub(crate) const F3_SLTI: u8 = 0x02;
pub(crate) const F3_SLTIU: u8 = 0x03;
pub(crate) const F3_XORI: u8 = 0x04;
pub(crate) const F3_ORI: u8 = 0x06;
pub(crate) const F3_ANDI: u8 = 0x07;

pub(crate) const F3_SLLI: u8 = 0x01;
pub(crate) const F3_SRLI_SRAI: u8 = 0x05; // check bit 30

pub(crate) const F3_ADD_SUB: u8 = 0x00; // check F7C
pub(crate) const F3_SLL: u8 = 0x01;
pub(crate) const F3_SLT: u8 = 0x02;
pub(crate) const F3_SLTU: u8 = 0x03;
pub(crate) const F3_XOR: u8 = 0x04;
pub(crate) const F3_SRL_SLA: u8 = 0x05; // check F7C
pub(crate) const F3_OR: u8 = 0x06;
pub(crate) const F3_AND: u8 = 0x07;

pub(crate) const F3_FENCE: u8 = 0x00;
pub(crate) const F3_FENCE_I: u8 = 0x01;

pub(crate) const F3_ECALL_EBREAK: u8 = 0x00; // check imm[11:0]
pub(crate) const F3_CSRRW: u8 = 0x01;
pub(crate) const F3_CSRRS: u8 = 0x02;
pub(crate) const F3_CSRRC: u8 = 0x03;
pub(crate) const F3_CSRRWI: u8 = 0x05;
pub(crate) const F3_CSRRSI: u8 = 0x06;
pub(crate) const F3_CSRRCI: u8 = 0x07;

// Function 7 codes
pub(crate) const F7_SRLI: u8 = 0x00;
pub(crate) const F7_SRAI: u8 = 0x08;

pub(crate) const F7_ADD: u8 = 0x00;
pub(crate) const F7_SUB: u8 = 0x20;

pub(crate) const F7_SRL: u8 = 0x00;
pub(crate) const F7_SRA: u8 = 0x20;
