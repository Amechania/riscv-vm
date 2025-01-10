pub use crate::cpu::memory::mmu::page::Page;

mod page;

pub(crate) struct MMU {
    page_table: Vec<Page>,
    page_offset_bits: usize, // Number of lower bits in the global address used for page offset
    page_mask: usize, // We calculate the mask once :3
    num_pages: usize,
}

impl MMU {
    pub(crate) fn get_memory(&self) -> &Vec<Page> {
        &self.page_table
    }
}

impl MMU {
    pub(crate) fn new(memsize: usize, page_offset_bits: usize) -> Self {
        let page_size = 1 << page_offset_bits;
        let num_pages = memsize / page_size;
        let mut page_table: Vec<Page> = Vec::with_capacity(num_pages);
        for _ in 0..num_pages {
            page_table.push(Page::new(page_size));
        }

        Self {
            page_table,
            page_offset_bits,
            page_mask: page_size - 1,
            num_pages,
        }
    }
    
    pub(crate) fn set_u8(&mut self, address: u32, value: u8) {
        let page_index = (address >> self.page_offset_bits) as usize;
        let page_offset = address as usize & self.page_mask;
        self.page_table[page_index].set_u8(page_offset as u32, value);
    }
    
    pub(crate) fn get_u8(&self, address: u32) -> u8 {
        let page_index = (address >> self.page_offset_bits) as usize;
        let page_offset = address as usize & self.page_mask;
        self.page_table[page_index].get_u8(page_offset as u32)
    }
    
    pub(crate) fn set_u16(&mut self, address: u32, value: u16) {
        let page_index = (address >> self.page_offset_bits) as usize;
        let page_offset = address as usize & self.page_mask;
        self.page_table[page_index].set_u16(page_offset as u32, value);
    }
    
    pub(crate) fn get_u16(&self, address: u32) -> u16 {
        let page_index = (address >> self.page_offset_bits) as usize;
        let page_offset = address as usize & self.page_mask;
        self.page_table[page_index].get_u16(page_offset as u32)
    }
    
    pub(crate) fn set_u32(&mut self, address: u32, value: u32) {
        let page_index = (address >> self.page_offset_bits) as usize;
        let page_offset = address as usize & self.page_mask;
        self.page_table[page_index].set_u32(page_offset as u32, value);
    }
    
    pub(crate) fn get_u32(&self, address: u32) -> u32 {
        let page_index = (address >> self.page_offset_bits) as usize;
        let page_offset = address as usize & self.page_mask;
        self.page_table[page_index].get_u32(page_offset as u32)
    }
}