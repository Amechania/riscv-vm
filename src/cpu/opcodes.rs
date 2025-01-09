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

// We check F7C to discern between ADD and SUB
pub(crate) const F3_ADD_SUB: u8 = 0x00; // check F7C

// First set of M extension instruction of F3 codes
pub(crate) const F3_MUL: u8 = 0x00;
pub(crate) const F3_MULH: u8 = 0x01;
pub(crate) const F3_MULHSU: u8 = 0x02;
pub(crate) const F3_MULHU: u8 = 0x03;
pub(crate) const F3_DIV: u8 = 0x04;
pub(crate) const F3_DIVU: u8 = 0x05;
pub(crate) const F3_REM: u8 = 0x06;
pub(crate) const F3_REMU: u8 = 0x07;

// W set of M extension instruction of F3 codes
pub(crate) const F3_MULW: u8 = 0x00;
pub(crate) const F3_DIVW: u8 = 0x04;
pub(crate) const F3_DIVUW: u8 = 0x05;
pub(crate) const F3_REMW: u8 = 0x06;
pub(crate) const F3_REMUW: u8 = 0x07;

// D set of M extension instruction of F3 codes
pub(crate) const F3_MULD: u8 = 0x00;
pub(crate) const F3_DIVD: u8 = 0x04;
pub(crate) const F3_DIVUD: u8 = 0x05;
pub(crate) const F3_REMD: u8 = 0x06;
pub(crate) const F3_REMUD: u8 = 0x07;

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

// These codes are used for every M extension instruction
pub(crate) const F7_M_EXTENSION: u8 = 0x33;
// W instructions are valid for RV64. We're only targeting RV32
// pub(crate) const F7_M_EXTENSION_W: u8 = 0x3B;

pub(crate) const F7_SRL: u8 = 0x00;
pub(crate) const F7_SRA: u8 = 0x20;

pub(crate) const F73_ADD: u16 = ((F7_ADD as u16) << 3) | (F3_ADD_SUB as u16);
pub(crate) const F73_SUB: u16 = ((F7_SUB as u16) << 3) | (F3_ADD_SUB as u16);
pub(crate) const F73_SLL: u16 = ((0x0u16) << 3) | (F3_SLL as u16);
pub(crate) const F73_SLT: u16 = ((0x0u16) << 3) | (F3_SLT as u16);
pub(crate) const F73_SLTU: u16 = ((0x0u16) << 3) | (F3_SLTU as u16);
pub(crate) const F73_XOR: u16 = ((0x0u16) << 3) | (F3_XOR as u16);
pub(crate) const F73_SRL: u16 = ((F7_SRL as u16) << 3) | (F3_SRL_SLA as u16);
pub(crate) const F73_SRA: u16 = ((F7_SRA as u16) << 3) | (F3_SRL_SLA as u16);
pub(crate) const F73_OR: u16 = ((0x0u16) << 3) | (F3_OR as u16);
pub(crate) const F73_AND: u16 = ((0x0u16) << 3) | (F3_AND as u16);
pub(crate) const F73_MUL: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_MUL as u16);
pub(crate) const F73_MULH: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_MULH as u16);
pub(crate) const F73_MULHSU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_MULHSU as u16);
pub(crate) const F73_MULHU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_MULHU as u16);
// W instructions are valid for RV64. We're only targeting RV32
// pub(crate) const F73_MULW: u16 = ((F7_M_EXTENSION_W as u16) << 3) | (F3_MULW as u16);

pub(crate) const F73_DIV: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_DIV as u16);
pub(crate) const F73_DIVU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_DIVU as u16);
// W instructions are valid for RV64. We're only targeting RV32
//pub(crate) const F73_DIVW: u16 = ((F7_M_EXTENSION_W as u16) << 3) | (F3_DIVW as u16);

pub(crate) const F73_REM: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_REM as u16);
pub(crate) const F73_REMU: u16 = ((F7_M_EXTENSION as u16) << 3) | (F3_REMU as u16);
// W instructions are valid for RV64. We're only targeting RV32
//pub(crate) const F73_REMW: u16 = ((F7_M_EXTENSION_W as u16) << 3) | (F3_REMW as u16);
