#![allow(dead_code)]
pub const REG_ZERO:u8 = 0;

pub const REG_RA:u8 = 1;

pub const REG_SP:u8 = 2;
pub const REG_GP:u8 = 3;
pub const REG_TP:u8 = 4;

pub const REG_T0:u8 = 5;
pub const REG_T1:u8 = 6;
pub const REG_T2:u8 = 7;
pub const REG_S0:u8 = 8;
pub const REG_S1:u8 = 9;
pub const REG_S2:u8 = 18;

pub(crate) struct Register {
    pub(crate) registers: [u32; 32],
    
    pub zero: *const u32,

    pub ra: *const u32,

    pub sp: *const u32,
    pub gp: *const u32,
    pub tp: *const u32,

    pub t0: *const u32,
    pub t1: *const u32,
    pub t2: *const u32,

    pub s0: *const u32,
    pub s1: *const u32,

    pub a0: *const u32,
    pub a1: *const u32,
    pub a2: *const u32,
    pub a3: *const u32,
    pub a4: *const u32,
    pub a5: *const u32,
    pub a6: *const u32,
    pub a7: *const u32,

    pub s2: *const u32,
    pub s3: *const u32,
    pub s4: *const u32,
    pub s5: *const u32,
    pub s6: *const u32,
    pub s7: *const u32,
    pub s8: *const u32,
    pub s9: *const u32,
    pub s10: *const u32,
    pub s11: *const u32,

    pub t3: *const u32,
    pub t4: *const u32,
    pub t5: *const u32,
    pub t6: *const u32,

    pub x0: *const u32,
    pub x1: *const u32,
    pub x2: *const u32,
    pub x3: *const u32,
    pub x4: *const u32,
    pub x5: *const u32,
    pub x6: *const u32,
    pub x7: *const u32,

    pub x8: *const u32,
    pub x9: *const u32,
    pub x10: *const u32,
    pub x11: *const u32,
    pub x12: *const u32,
    pub x13: *const u32,
    pub x14: *const u32,
    pub x15: *const u32,

    pub x16: *const u32,
    pub x17: *const u32,
    pub x18: *const u32,
    pub x19: *const u32,
    pub x20: *const u32,
    pub x21: *const u32,
    pub x22: *const u32,
    pub x23: *const u32,

    pub x24: *const u32,
    pub x25: *const u32,
    pub x26: *const u32,
    pub x27: *const u32,
    pub x28: *const u32,
    pub x29: *const u32,
    pub x30: *const u32,
    pub x31: *const u32,
}

impl Register {
    pub(crate) fn new() -> Register {
        let registers: [u32; 32] = [0; 32];
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

    pub fn set_register(&mut self, register: u8, value: u32) {
        self.registers[register as usize] = value;
    }

    pub fn get_register(&self, register: u8) -> u32 {
        self.registers[register as usize]
    }
}