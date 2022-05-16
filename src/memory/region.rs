pub trait MemoryRegion {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, byte: u8, addr: u16);
    fn read_word(&self, addr: u16) -> u16;
    fn write_word(&mut self, word: u16, addr: u16);
}
