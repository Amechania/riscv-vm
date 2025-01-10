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
    pub(crate) registers: Register,
    pub(crate) memory: Memory,
    instruction: u32,
    opcode: u8,
}

#[allow(dead_code)]
impl CPU {
    pub fn new() -> Self {
        Self {
        pc: 4,
            registers: Register::new(),
            memory: Memory::new(MEMSIZE, 8),
            instruction: 0,
            opcode: 0,
        }
    }
    
    fn get_pc(&self) -> u32 {
        self.pc
    }

    fn fetch_inst(&mut self) {
        self.instruction = self.memory.get_u32(self.pc);
        self.opcode = (self.instruction & 0x7F) as u8;
    }

    pub(crate) fn load_image(&mut self, offset: u32, program: &Vec<u8>) {
        self.memory.load_image(offset, program);
    }

    pub(crate) fn run(&mut self, start: u32) {
        self.pc = start;
        loop {
            self.fetch_inst();
            if self.exec_inst() {
                return;
            }
        }
    }
}

///// TESTS /////
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::cpu::*;
    use crate::cpu::opcodes::*;

    #[test]
    fn test_fetch() {
        let mut cpu = CPU::new();
        cpu.pc = 0x10;
        let instruction = 0xA51E9F80 | OP_JAL as u32;
        cpu.memory.set_u32(cpu.pc, instruction);

        // Fetch instruction
        cpu.fetch_inst();

        // Verify results
        assert_eq!(cpu.instruction, instruction);
        assert_eq!(cpu.opcode, OP_JAL);
    }
}

