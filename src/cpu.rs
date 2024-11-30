// RISC-V Tiny VM - Ivi Ballou / Amechania

mod register;
mod opcodes;
mod memory;
mod instruction;

use crate::cpu::register::*;
use crate::cpu::memory::Memory;
const MEMSIZE_MB: usize = 2;
const MEMSIZE: usize = MEMSIZE_MB*1024*1024; // 2MB

pub struct CPU {
    pc: u32,
    registers: Register,
    memory: Memory,
    instruction: u32,
    opcode: u8,
}

#[allow(dead_code)]
impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            registers: Register::new(),
            memory: Memory::new(MEMSIZE),
            instruction: 0,
            opcode: 0,
        }
    }
    
    fn get_pc(&self) -> u32 {
        self.pc
    }
}

