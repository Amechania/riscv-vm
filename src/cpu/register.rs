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
}

impl Register {
    pub(crate) fn new() -> Register {
        Register {
            registers: [0; 32],
        }
    }

    pub fn set_register(&mut self, register: u8, value: u32) {
        if register == 0 {
            return;
        }
        self.registers[register as usize] = value;
    }

    pub fn get_register(&self, register: u8) -> u32 {
        if register == 0 {
            return 0;
        }
        self.registers[register as usize]
    }
}