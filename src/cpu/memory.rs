pub(crate) struct Memory {
    memory: Box<[u8]>,
}


impl Memory {
    pub fn new(memsize: usize) -> Self {
        let memory = {
            let mut memory = Vec::with_capacity(memsize); // Allocates directly on the heap
            memory.resize(memsize, 0); // Fill with zeroes
            memory.into_boxed_slice() // Convert Vec into Box<[u32]>
        };
        Self {
            memory, // Convert to heap-allocated slice
        }
    }

    pub(crate) fn get_memory(&self) -> &Box<[u8]> {
        &self.memory
    }

    // Sets a byte in memory
    pub fn set_u8(&mut self, address: u32, value: u8) {
        self.memory[address as usize] = value;
    }

    // Gets a byte from memory
    pub fn get_u8(&self, address: u32) -> u8 {
        self.memory[address as usize]
    }

    // Splits a half word into 2 bytes and stores them in memory
    pub fn set_u16(&mut self, address: u32, value: u16) {
        let value_high:u8 = (value >> 8) as u8;
        let value_low:u8 = (value & 0xFF) as u8;
        // Keep in mind that the endianness of the CPU is little endian
        self.memory[address as usize] = value_low;
        self.memory[address as usize + 1] = value_high;
    }

    // Gets a half word from memory, as two bytes, combines and returns it as u16
    pub fn get_u16(&self, address: u32) -> u16 {
        let value: u16 = (self.memory[address as usize + 1] as u16) << 8
            | (self.memory[address as usize] as u16);
        value
    }

    // Splits a word into 4 bytes and stores them in memory
    pub fn set_u32(&mut self, address: u32, value: u32) {
        let value_high_high: u8 = (value >> 24) as u8;
        let value_high_low: u8 = (value >> 16 & 0xFF) as u8;
        let value_low_high: u8 = (value >> 8 & 0xFF) as u8;
        let value_low_low: u8 = (value & 0xFF) as u8;
        // Keep in mind that the endianness of the CPU is little endian
        self.memory[address as usize] = value_low_low;
        self.memory[address as usize + 1] = value_low_high;
        self.memory[address as usize + 2] = value_high_low;
        self.memory[address as usize + 3] = value_high_high;
    }

    // Gets a word from memory, as four bytes, combines and returns it as u32
    pub fn get_u32(&self, address: u32) -> u32 {
        let value_high_high: u32 = (self.memory[address as usize + 3] as u32) << 24;
        let value_high_low: u32 = (self.memory[address as usize + 2] as u32) << 16;
        let value_low_high: u32 = (self.memory[address as usize + 1] as u32) << 8;
        let value_low_low: u32 = self.memory[address as usize] as u32;
        let value:u32 = value_high_high | value_high_low | value_low_high | value_low_low;
        value
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
        let mut memory = Memory::new(1024);
        memory.set_u8(10, 0xFF);
        assert_eq!(memory.get_u8(10), 0xFF);
    }

    #[test]
    fn test_set_get_u16() {
        let mut memory = Memory::new(1024);
        memory.set_u16(10, 0xFFFF);
        assert_eq!(memory.get_u16(10), 0xFFFF);
    }

    #[test]
    fn test_set_get_u32() {
        let mut memory = Memory::new(1024);
        memory.set_u32(10, 0xFFFFFFFF);
        assert_eq!(memory.get_u32(10), 0xFFFFFFFF);
    }

}