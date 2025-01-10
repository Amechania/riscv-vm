pub struct Page {
    page: Box<[u8]>,
}

impl Page {
    pub fn new(page_size: usize) -> Self {
        let page = vec![0u8; page_size].into_boxed_slice();
        Self { page }
    }

    pub fn get_page(&self) -> &Box<[u8]> {
        &self.page
    }

    // Sets a byte in page
    pub fn set_u8(&mut self, offset: u32, value: u8) {
        self.page[offset as usize] = value;
    }

    // Gets a byte from page
    pub fn get_u8(&self, offset: u32) -> u8 {
        self.page[offset as usize]
    }

    // Splits a half word into 2 bytes and stores them in page
    pub fn set_u16(&mut self, offset: u32, value: u16) {
        let value_high:u8 = (value >> 8) as u8;
        let value_low:u8 = (value & 0xFF) as u8;
        // Keep in mind that the endianness of the CPU is little endian
        self.page[offset as usize] = value_low;
        self.page[offset as usize + 1] = value_high;
    }

    // Gets a half word from page, as two bytes, combines and returns it as u16
    pub fn get_u16(&self, offset: u32) -> u16 {
        let value: u16 = (self.page[offset as usize + 1] as u16) << 8
            | (self.page[offset as usize] as u16);
        value
    }

    // Splits a word into 4 bytes and stores them in page
    pub fn set_u32(&mut self, offset: u32, value: u32) {
        let value_high_high: u8 = (value >> 24) as u8;
        let value_high_low: u8 = (value >> 16 & 0xFF) as u8;
        let value_low_high: u8 = (value >> 8 & 0xFF) as u8;
        let value_low_low: u8 = (value & 0xFF) as u8;
        // Keep in mind that the endianness of the CPU is little endian
        self.page[offset as usize] = value_low_low;
        self.page[offset as usize + 1] = value_low_high;
        self.page[offset as usize + 2] = value_high_low;
        self.page[offset as usize + 3] = value_high_high;
    }

    // Gets a word from page, as four bytes, combines and returns it as u32
    pub fn get_u32(&self, offset: u32) -> u32 {
        let value: u32 = (self.page[offset as usize + 3] as u32) << 24
            | (self.page[offset as usize + 2] as u32) << 16
            | (self.page[offset as usize + 1] as u32) << 8
            | (self.page[offset as usize] as u32);
        value
    }
}

///// TESTS /////
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::cpu::memory::mmu::page::{*};

    #[test]
    fn test_set_get_u8() {
        let mut page = Page::new(128);
        page.set_u8(10, 0xFE);
        assert_eq!(page.get_u8(10), 0xFE);
    }

    #[test]
    fn test_set_get_u16() {
        let mut page = Page::new(128);
        page.set_u16(10, 0xFEDC);
        assert_eq!(page.get_u16(10), 0xFEDC);
    }

    #[test]
    fn test_set_get_u32() {
        let mut page = Page::new(128);
        page.set_u32(10, 0xFEDCBA98);
        assert_eq!(page.get_u32(10), 0xFEDCBA98);
    }
}
