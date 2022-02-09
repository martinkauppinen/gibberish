#[derive(Debug, Clone)]
pub struct Ram {
    start: u16,

    #[allow(dead_code)]
    end: u16,

    memory: Vec<u8>,
}

impl Ram {
    pub fn new(start: u16, end: u16) -> Self {
        Self {
            start,
            end,
            memory: vec![0; ((end - start) + 1) as usize],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.memory[(addr - self.start) as usize]
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read_byte(addr);
        let hi = self.read_byte(addr + 1);
        u16::from_le_bytes([lo, hi])
    }

    pub fn write_byte(&mut self, byte: u8, addr: u16) {
        self.memory[(addr - self.start) as usize] = byte;
    }

    pub fn write_word(&mut self, word: u16, addr: u16) {
        let bytes = word.to_le_bytes();
        self.write_byte(bytes[0], addr);
        self.write_byte(bytes[1], addr + 1);
    }
}
