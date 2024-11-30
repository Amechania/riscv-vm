// RISC-V Tiny VM - Ivi Ballou / Amechania

// 32-bit RISC-V instructions
struct Opcodes {
    lui: u8, // LUI
    auipc: u8, // AUIPC
    jal: u8, // JAL
    jalr: u8, // JALR
    branch: u8, // BEQ, BNE, BLT, BGE, BLTU, BGEU
    load: u8, // LB, LH, LW, LBU, LHU
    store: u8, // SB, SH, SW
    alui: u8, // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SRLI, SRAI
    alu: u8, // ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND
    fence: u8, // FENCE, FENCE.I
    e_c: u8, // ECALL, EBREAK, CSRRW, CSRRS, CSRRC, CSRRWI, CSRRSI, CSRRCI
}

impl Opcodes {
    fn new() -> Self {
        Opcodes {
            lui: 0x37,
            auipc: 0x17,
            jal: 0x6F,
            jalr: 0x67,
            branch: 0x63,
            load: 0x03,
            store: 0x02,
            alui: 0x13,
            alu: 0x33,
            fence: 0x0F,
            e_c: 0x73,
        }
    }
}