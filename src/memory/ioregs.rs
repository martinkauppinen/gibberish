use crate::memory::timer::DIV;

use super::{
    map::{IO_REGS_END, IO_REGS_START},
    ram::Ram,
    region::MemoryRegion,
    timer::{TimerRegisters, TAC},
};

#[derive(Debug, Clone)]
pub struct IoRegs {
    p1: u8, // joypad
    timer: TimerRegisters,
    others: Ram, // TODO
}

impl IoRegs {
    pub fn new() -> Self {
        Self {
            p1: 0,
            timer: TimerRegisters::new(),
            others: Ram::new(TAC + 1, IO_REGS_END),
        }
    }

    pub fn get_timer_mut(&mut self) -> &mut TimerRegisters {
        &mut self.timer
    }
}

impl MemoryRegion for IoRegs {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            IO_REGS_START => self.p1,
            DIV..=TAC => self.timer.read_byte(addr),
            IO_REGS_START..=IO_REGS_END => self.others.read_byte(addr),
            _ => panic!("Invalid i/o register address: {:x}", addr),
        }
    }

    fn write_byte(&mut self, byte: u8, addr: u16) {
        match addr {
            IO_REGS_START => self.p1 = byte,
            DIV..=TAC => self.timer.write_byte(byte, addr),
            IO_REGS_START..=IO_REGS_END => self.others.write_byte(byte, addr),
            _ => panic!("Invalid i/o register address: {:x}", addr),
        }
    }

    fn read_word(&self, addr: u16) -> u16 {
        let hi = self.read_byte(addr);
        let lo = self.read_byte(addr + 1);
        u16::from_be_bytes([hi, lo])
    }

    fn write_word(&mut self, word: u16, addr: u16) {
        let [hi, lo] = word.to_be_bytes();
        self.write_byte(hi, addr);
        self.write_byte(lo, addr + 1);
    }
}
