//include!("globals.rs");

struct Register {
    registers: [ArchWS; 32],
    
    zero: * ArchWS,

    ra: * ArchWS,

    sp: * ArchWS,
    gp: * ArchWS,
    tp: * ArchWS,

    t0: * ArchWS,
    t1: * ArchWS,
    t2: * ArchWS,

    s0: * ArchWS,
    s1: * ArchWS,

    a0: * ArchWS,
    a1: * ArchWS,
    a2: * ArchWS,
    a3: * ArchWS,
    a4: * ArchWS,
    a5: * ArchWS,
    a6: * ArchWS,
    a7: * ArchWS,

    s2: * ArchWS,
    s3: * ArchWS,
    s4: * ArchWS,
    s5: * ArchWS,
    s6: * ArchWS,
    s7: * ArchWS,
    s8: * ArchWS,
    s9: * ArchWS,
    s10: * ArchWS,
    s11: * ArchWS,

    t3: * ArchWS,
    t4: * ArchWS,
    t5: * ArchWS,
    t6: * ArchWS,

    x0: * ArchWS,
    x1: * ArchWS,
    x2: * ArchWS,
    x3: * ArchWS,
    x4: * ArchWS,
    x5: * ArchWS,
    x6: * ArchWS,
    x7: * ArchWS,

    x8: * ArchWS,
    x9: * ArchWS,
    x10: * ArchWS,
    x11: * ArchWS,
    x12: * ArchWS,
    x13: * ArchWS,
    x14: * ArchWS,
    x15: * ArchWS,

    x16: * ArchWS,
    x17: * ArchWS,
    x18: * ArchWS,
    x19: * ArchWS,
    x20: * ArchWS,
    x21: * ArchWS,
    x22: * ArchWS,
    x23: * ArchWS,

    x24: * ArchWS,
    x25: * ArchWS,
    x26: * ArchWS,
    x27: * ArchWS,
    x28: * ArchWS,
    x29: * ArchWS,
    x30: * ArchWS,
    x31: * ArchWS,
}

impl Register {
    fn new() -> Register {
        let registers: [ArchWS; 32];
        let x0 = &registers[0];
        let zero = &registers[0];
        let x1 = &registers[1];
        let ra = &registers[1];
        let x2 = &registers[2];
        let sp = &registers[2];
        let x3 = &registers[3];
        let gp = &registers[3];

        let x4 = &registers[4];
        let tp = &registers[4];
        let x5 = &registers[5];
        let t0 = &registers[5];
        let x6 = &registers[6];
        let t1 = &registers[6];
        let x7 = &registers[7];
        let t2 = &registers[7];

        let x8 = &registers[8];
        let s0 = &registers[8];
        let x9 = &registers[9];
        let s1 = &registers[9];
        let x10 = &registers[10];
        let a0 = &registers[10];
        let x11 = &registers[11];
        let a1 = &registers[11];

        let x12 = &registers[12];
        let a2 = &registers[12];
        let x13 = &registers[13];
        let a3 = &registers[13];
        let x14 = &registers[14];
        let a4 = &registers[14];
        let x15 = &registers[15];
        let a5 = &registers[15];

        let x16 = &registers[16];
        let a6 = &registers[16];
        let x17 = &registers[17];
        let a7 = &registers[17];
        let x18 = &registers[18];
        let s2 = &registers[18];
        let x19 = &registers[19];
        let s3 = &registers[19];

        let x20 = &registers[20];
        let s4 = &registers[20];
        let x21 = &registers[21];
        let s5 = &registers[21];
        let x22 = &registers[22];
        let s6 = &registers[22];
        let x23 = &registers[23];
        let s7 = &registers[23];

        let x24 = &registers[24];
        let s8 = &registers[24];
        let x25 = &registers[25];
        let s9 = &registers[25];
        let x26 = &registers[26];
        let s10 = &registers[26];
        let x27 = &registers[27];
        let s11 = &registers[27];

        let x28 = &registers[28];
        let t3 = &registers[28];
        let x29 = &registers[29];
        let t4 = &registers[29];
        let x30 = &registers[30];
        let t5 = &registers[30];
        let x31 = &registers[31];
        let t6 = &registers[31];
        Register {
            registers: [0; 32],
            x0,
            zero,
            x1,
            ra,
            x2,
            x3,
            sp,
            x4,
            gp,
            tp,
            t0,
            x6,
            t1,
            x7,
            t2,
            x8,
            s0,
            x9,
            s1,
            x10,
            a0,
            x11,
            a1,
            x12,
            a2,
            x13,
            a3,
            x14,
            a4,
            x15,
            a5,
            x16,
            a6,
            x17,
            a7,
            x18,
            s2,
            x19,
            s3,
            x20,
            s4,
            x21,
            s5,
            x22,
            s6,
            x23,
            s7,
            x24,
            s8,
            x25,
            s9,
            x26,
            s10,
            x27,
            s11,
            x28,
            t3,
            x29,
            t4,
            x30,
            t5,
            x31,
            t6,
            x5,
        }
    }
}