use crate::memory::ram::Ram;

#[derive(Debug, Clone)]
pub struct MemoryMap {
    // Temporarily model everything with the Ram struct
    cartridge: Ram,
    vram: Ram,
    eram: Ram,
    iram: Ram,
    iram_echo: Ram,
    sprite_attrs: Ram,
    io_regs: Ram,
    hram: Ram,
    int_enable_reg: u8,
}

impl MemoryMap {
    pub fn new() -> Self {
        Self {
            cartridge: Ram::new(CART_START, CART_END),
            vram: Ram::new(VRAM_START, VRAM_END),
            eram: Ram::new(ERAM_START, ERAM_END),
            iram: Ram::new(IRAM_START, IRAM_END),
            iram_echo: Ram::new(IRAM_ECHO_START, IRAM_ECHO_END),
            sprite_attrs: Ram::new(SPRITE_ATTRS_START, SPRITE_ATTRS_END),
            io_regs: Ram::new(IO_REGS_START, IO_REGS_END),
            hram: Ram::new(HRAM_START, HRAM_END),
            int_enable_reg: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            CART_START..=CART_END => self.cartridge.read_byte(addr),
            VRAM_START..=VRAM_END => self.vram.read_byte(addr),
            ERAM_START..=ERAM_END => self.eram.read_byte(addr),
            IRAM_START..=IRAM_END => self.iram.read_byte(addr),
            IRAM_ECHO_START..=IRAM_ECHO_END => self.iram_echo.read_byte(addr),
            SPRITE_ATTRS_START..=SPRITE_ATTRS_END => self.sprite_attrs.read_byte(addr),
            IO_REGS_START..=IO_REGS_END => self.io_regs.read_byte(addr),
            HRAM_START..=HRAM_END => self.hram.read_byte(addr),
            INT_ENABLE_ADDR => self.int_enable_reg,
            _ => {
                panic!("Unusable address: {:x}", addr);
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8, addr: u16) {
        match addr {
            CART_START..=CART_END => self.cartridge.write_byte(byte, addr),
            VRAM_START..=VRAM_END => self.vram.write_byte(byte, addr),
            ERAM_START..=ERAM_END => self.eram.write_byte(byte, addr),
            IRAM_START..=IRAM_END => self.iram.write_byte(byte, addr),
            IRAM_ECHO_START..=IRAM_ECHO_END => self.iram_echo.write_byte(byte, addr),
            SPRITE_ATTRS_START..=SPRITE_ATTRS_END => self.sprite_attrs.write_byte(byte, addr),
            IO_REGS_START..=IO_REGS_END => self.io_regs.write_byte(byte, addr),
            HRAM_START..=HRAM_END => self.hram.write_byte(byte, addr),
            INT_ENABLE_ADDR => self.int_enable_reg = byte,
            _ => {
                panic!("Unusable address: {:x}", addr);
            }
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        match addr {
            CART_START..=CART_END => self.cartridge.read_word(addr),
            VRAM_START..=VRAM_END => self.vram.read_word(addr),
            ERAM_START..=ERAM_END => self.eram.read_word(addr),
            IRAM_START..=IRAM_END => self.iram.read_word(addr),
            IRAM_ECHO_START..=IRAM_ECHO_END => self.iram_echo.read_word(addr),
            SPRITE_ATTRS_START..=SPRITE_ATTRS_END => self.sprite_attrs.read_word(addr),
            IO_REGS_START..=IO_REGS_END => self.io_regs.read_word(addr),
            HRAM_START..=HRAM_END => self.hram.read_word(addr),
            _ => {
                panic!("Unusable address: {:x}", addr);
            }
        }
    }

    pub fn write_word(&mut self, word: u16, addr: u16) {
        match addr {
            CART_START..=CART_END => self.cartridge.write_word(word, addr),
            VRAM_START..=VRAM_END => self.vram.write_word(word, addr),
            ERAM_START..=ERAM_END => self.eram.write_word(word, addr),
            IRAM_START..=IRAM_END => self.iram.write_word(word, addr),
            IRAM_ECHO_START..=IRAM_ECHO_END => self.iram_echo.write_word(word, addr),
            SPRITE_ATTRS_START..=SPRITE_ATTRS_END => self.sprite_attrs.write_word(word, addr),
            IO_REGS_START..=IO_REGS_END => self.io_regs.write_word(word, addr),
            HRAM_START..=HRAM_END => self.hram.write_word(word, addr),
            _ => {
                panic!("Unusable address: {:x}", addr);
            }
        }
    }
}

const CART_START: u16 = 0x0000;
const CART_END: u16 = 0x7FFF;
const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const ERAM_START: u16 = 0xA000;
const ERAM_END: u16 = 0xBFFF;
const IRAM_START: u16 = 0xC000;
const IRAM_END: u16 = 0xDFFF;
const IRAM_ECHO_START: u16 = 0xE000;
const IRAM_ECHO_END: u16 = 0xFDFF;
const SPRITE_ATTRS_START: u16 = 0xFE00;
const SPRITE_ATTRS_END: u16 = 0xFE9F;
const UNUSABLE_START: u16 = 0xFEA0;
const UNUSABLE_END: u16 = 0xFEFF;
const IO_REGS_START: u16 = 0xFF00;
const IO_REGS_END: u16 = 0xFF7F;
const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFEF;
const INT_ENABLE_ADDR: u16 = 0xFFFF;
