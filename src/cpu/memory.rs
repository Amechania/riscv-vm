use crate::cpu::memory::mmu::{MMU, Page};

pub mod mmu;

pub(crate) struct Memory {
    mmu: MMU,
}

impl Memory {
    pub fn new(memsize: usize, page_offset_bits: usize) -> Self {
        let mmu = MMU::new(memsize, page_offset_bits);
        Self {
            mmu,
        }
    }

    pub(crate) fn get_memory(&self) -> &Vec<Page> {
        self.mmu.get_memory()
    }

    // Sets a byte in memory using MMU
    pub fn set_u8(&mut self, address: u32, value: u8) {
        self.mmu.set_u8(address, value);
    }

    // Gets a byte from memory using MMU
    pub fn get_u8(&self, address: u32) -> u8 {
        self.mmu.get_u8(address)
    }

    // Splits a half word into 2 bytes and stores them in memory using MMU
    pub fn set_u16(&mut self, address: u32, value: u16) {
        self.mmu.set_u16(address, value);
    }

    // Gets a half word from memory using MMU, as two bytes, combines and returns it as u16
    pub fn get_u16(&self, address: u32) -> u16 {
        self.mmu.get_u16(address)
    }

    // Splits a word into 4 bytes and stores them in memory using MMU
    pub fn set_u32(&mut self, address: u32, value: u32) {
        self.mmu.set_u32(address, value);
    }

    // Gets a word from memory using MMU, as four bytes, combines and returns it as u32
    pub fn get_u32(&self, address: u32) -> u32 {
        self.mmu.get_u32(address)
    }

    pub fn load_image(&mut self, offset: u32, image: &Vec<u8>) {
        for (i, byte) in image.iter().enumerate() {
            self.set_u8(offset + i as u32, *byte);
        }
    }
}

///// TESTS /////

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::cpu::memory::{*};

    #[test]
    fn test_set_get_u8() {
        let mut memory = Memory::new(1024, 8);
        memory.set_u8(10, 0xFF);
        assert_eq!(memory.get_u8(10), 0xFF);
    }

    #[test]
    fn test_set_get_u16() {
        let mut memory = Memory::new(1024, 8);
        memory.set_u16(10, 0xFFFF);
        assert_eq!(memory.get_u16(10), 0xFFFF);
    }

    #[test]
    fn test_set_get_u32() {
        let mut memory = Memory::new(1024, 8);
        memory.set_u32(10, 0xFFFFFFFF);
        assert_eq!(memory.get_u32(10), 0xFFFFFFFF);
    }
}