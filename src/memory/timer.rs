use crate::cpu::interrupts::Interrupt;

use super::region::MemoryRegion;

/// Memory mapped location of the [`div`](#structfield.div) register.
pub const DIV: u16 = 0xFF04;
/// Memory mapped location of the [`tima`](#structfield.tima) register.
pub const TIMA: u16 = 0xFF05;
/// Memory mapped location of the [`tma`](#structfield.tma) register.
pub const TMA: u16 = 0xFF06;
/// Memory mapped location of the [`tac`](#structfield.tac) register.
pub const TAC: u16 = 0xFF07;

#[derive(Debug, Clone)]
pub struct TimerRegisters {
    /// Divider register (R/W), incremented at 16384 Hz. Writing sets to 0.
    div: u8,
    /// Timer counter (R/W). Incremented at frequency specified by
    /// [`tac`](#structfield.tac). Generates an interrupt on overflow.
    tima: u8,
    /// Timer modulo (R/W). Value to load into [`tima`](#structfield.tima) at
    /// overflow.
    tma: u8,
    /// Timer control (R/W).
    /// ```text
    /// xxxxx000
    ///      |``- Input clock select
    ///      |      00:   4096 Hz (CPU clock / 1024)
    ///      |      01: 262144 Hz (CPU clock /   16)
    ///      |      10:  65536 Hz (CPU clock /   64)
    ///      |      11:  16384 Hz (CPU clock /  256)
    ///      |
    ///      `--- Timer stop
    ///             0: Stop timer
    ///             1: Start timer
    /// ```
    tac: u8,
    /// Internal use. A clock divider to help decide when to tick
    /// [`div`](#structfield.div).
    internal_div: usize,
    /// Internal use. A clock divider to help decide when to tick
    /// [`tima`](#structfield.tima).
    internal_tima: usize,
}

impl TimerRegisters {
    pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            internal_div: 0,
            internal_tima: 0,
        }
    }

    /// Tick the timers. [`div`](#structfield.div) gets incremented at a fixed
    /// rate of 16384 Hz, whereas [`tima`](#structfield.tima) gets incremented
    /// at the rate specified by [`tac`](#structfield.tac) if enabled. On
    /// overflow, [`tima`](#structfield.tima) gets reloaded with the value in
    /// [`tma`](#structfield.tma).
    ///
    /// # Arguments
    /// * `machine_cycles` - The amount of machine cycles that have ticked since
    /// last invocation. The machine clock rate is ~1.05MHz, and the
    /// [`div`](#structfield.div) and [`tima`](#structfield.tima) registers
    /// are incremented at a divided rate.
    ///
    /// # Returns
    /// * An [`Option<Interrupt>`] with the value [`Interrupt::Timer`] if
    /// [`tima`](#structfield.tima) overflowed, otherwise [`None`].
    pub fn tick(&mut self, machine_cycles: usize) -> Option<Interrupt> {
        self.internal_div += machine_cycles * 4;

        // 1 tick @ 16384 Hz = 256 cpu ticks @ ~4.19 MHz
        while self.internal_div >= 256 {
            self.div = self.div.wrapping_add(1);
            self.internal_div -= 256;
        }

        let timer_enabled = self.tac & 0x04 != 0;
        let input_clock_select = self.tac & 0x03;
        let input_clock = match input_clock_select {
            0 => 1024, // 1 tick @ 4096Hz = 1024 cpu ticks
            1 => 16,   // 1 tick @ 262144Hz = 16 cpu ticks
            2 => 64,   // 1 tick @ 65536Hz = 64 cpu ticks
            3 => 256,  // 1 tick @ 16384Hz = 256 cpu ticks
            _ => unreachable!(),
        };

        if timer_enabled {
            self.internal_tima += machine_cycles * 4;

            while self.internal_tima >= input_clock {
                self.tima = self.tima.wrapping_add(1);
                self.internal_tima -= input_clock;
                if self.tima == 0 {
                    self.tima = self.tma;
                    return Some(Interrupt::Timer);
                }
            }
        }

        None
    }
}

impl MemoryRegion for TimerRegisters {
    /// Write byte into registers. When [`div`](#structfield.div) is written
    /// to, the value is always set to 0, no matter what the `byte` argument
    /// was.
    ///
    /// # Arguments
    /// * `byte` - The desired byte to write into a register.
    /// * `addr` - The address to write `byte` to.
    ///
    /// # Panics
    /// If `addr` does not correspond to one of the registers, the function
    /// panics.
    fn write_byte(&mut self, byte: u8, addr: u16) {
        match addr {
            DIV => self.div = 0, // DIV is set to 0 no matter what was written
            TIMA => self.tima = byte,
            TMA => self.tma = byte,
            TAC => self.tac = byte,
            _ => panic!("Invalid timer address: {:x}", addr),
        }
    }

    /// Write word into registers. When [`div`](#structfield.div) is written
    /// to, the value is always set to 0, no matter what the `byte` argument
    /// was. Internally just calls [`write_byte`](TimerRegisters::write_byte)
    /// with `addr` and `addr + 1`.
    ///
    /// # Arguments
    /// * `word` - The desired word to write into memory.
    /// * `addr` - The address to write `word` to.
    ///
    /// # Panics
    /// Since this function just calls
    /// [`write_byte`](TimerRegisters::write_byte) twice, the panic conditions
    /// are the same.
    fn write_word(&mut self, word: u16, addr: u16) {
        let [hi, lo] = word.to_be_bytes();
        self.write_byte(hi, addr);
        self.write_byte(lo, addr + 1);
    }

    /// Read byte from registers.
    ///
    /// # Arguments
    /// * `addr` - The address to read from.
    ///
    /// # Returns
    /// The value of the register mapped at `addr`.
    ///
    /// # Panics
    /// If `addr` does not correspond to one of the registers, the function
    /// panics.
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            DIV => self.div,
            TIMA => self.tima,
            TMA => self.tma,
            TAC => self.tac,
            _ => panic!("Invalid timer address: {:x}", addr),
        }
    }

    /// Read word from registers.
    ///
    /// # Arguments
    /// * `addr` - The address to read from.
    ///
    /// # Returns
    /// The value of the register pair mapped at `addr` and `addr + 1`.
    ///
    /// # Panics
    /// If `addr` does not correspond to one of the registers, the function
    /// panics.
    fn read_word(&self, addr: u16) -> u16 {
        let hi = self.read_byte(addr);
        let lo = self.read_byte(addr + 1);
        u16::from_be_bytes([hi, lo])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    enum TimaSpeed {
        Clock4k = 256, // 256 machine cycles = 1 cycle @ 4096Hz
        Clock256k = 4,
        Clock64k = 16,
        Clock16k = 64,
    }

    enum TimerControl {
        Enabled4k = 0b100,
        Enabled256k = 0b101,
        Enabled64k = 0b110,
        Enabled16k = 0b111,
        Disabled16k = 0b011,
    }

    #[test]
    fn test_div_writing() {
        let mut timer = TimerRegisters::new();
        let ticks = TimaSpeed::Clock16k as usize;

        // Tick 10 times
        for i in 0..10 {
            timer.tick(ticks);
            let div = timer.read_byte(DIV);
            assert_eq!(div, i + 1);
        }

        // Write to div
        timer.write_byte(0xFF, DIV);
        assert_eq!(timer.read_byte(DIV), 0);
    }

    #[test]
    fn test_basic_overflow() {
        let mut timer = TimerRegisters::new();
        let ticks = TimaSpeed::Clock16k as usize;

        // Enable timer, 16384 Hz
        timer.write_byte(TimerControl::Enabled16k as u8, TAC);

        for i in 0..255 {
            let tick = timer.tick(ticks);
            let div = timer.read_byte(DIV);
            let tima = timer.read_byte(TIMA);

            assert!(tick.is_none());
            assert_eq!(div, i + 1);
            assert_eq!(tima, i + 1);
        }

        let tick = timer.tick(ticks);
        assert!(tick.is_some());
    }

    #[test]
    fn test_disabled_timer() {
        let mut timer = TimerRegisters::new();
        let ticks = TimaSpeed::Clock16k as usize;

        // Disable timer, 16384 Hz
        timer.write_byte(TimerControl::Disabled16k as u8, TAC);

        for i in 0..255 {
            let tick = timer.tick(ticks);
            let div = timer.read_byte(DIV);
            let tima = timer.read_byte(TIMA);

            assert!(tick.is_none());
            assert_eq!(div, i + 1);
            assert_eq!(tima, 0);
        }

        let tick = timer.tick(ticks);
        assert!(tick.is_none());
    }

    #[test]
    fn test_modulo_load() {
        let mut timer = TimerRegisters::new();
        let ticks = TimaSpeed::Clock16k as usize;

        // Enable timer, 16384 Hz
        timer.write_byte(TimerControl::Enabled16k as u8, TAC);
        timer.write_byte(0x0F, TMA);

        for i in 0..255 {
            let tick = timer.tick(ticks);
            let tima = timer.read_byte(TIMA);

            assert!(tick.is_none());
            assert_eq!(tima, i + 1);
        }

        let tick = timer.tick(ticks);
        assert!(tick.is_some());
        assert_eq!(timer.read_byte(TIMA), timer.read_byte(TMA));

        for i in timer.read_byte(TMA)..255 {
            let tick = timer.tick(ticks);
            let tima = timer.read_byte(TIMA);

            assert!(tick.is_none());
            assert_eq!(tima, i + 1);
        }

        let tick = timer.tick(ticks);
        assert!(tick.is_some());
        assert_eq!(timer.read_byte(TIMA), timer.read_byte(TMA));
    }

    #[test]
    fn test_timer_frequencies() {
        let mut timer = TimerRegisters::new();
        let ticks = TimaSpeed::Clock4k as usize;

        // Enable timer, 4096 Hz
        timer.write_byte(TimerControl::Enabled4k as u8, TAC);

        for i in 0..255 {
            let tick = timer.tick(ticks);
            let tima = timer.read_byte(TIMA);

            assert!(tick.is_none());
            assert_eq!(tima, i + 1);
        }
        assert!(timer.tick(ticks).is_some());

        // Enable timer, 65536 Hz
        let mut timer = TimerRegisters::new();
        let ticks = TimaSpeed::Clock64k as usize;
        timer.write_byte(TimerControl::Enabled64k as u8, TAC);

        for i in 0..255 {
            let tick = timer.tick(ticks);
            let tima = timer.read_byte(TIMA);

            assert!(tick.is_none());
            assert_eq!(tima, i + 1);
        }
        assert!(timer.tick(ticks).is_some());

        // Enable timer, 262144 Hz
        let mut timer = TimerRegisters::new();
        let ticks = TimaSpeed::Clock256k as usize;
        timer.write_byte(TimerControl::Enabled256k as u8, TAC);

        for i in 0..255 {
            let tick = timer.tick(ticks);
            let tima = timer.read_byte(TIMA);

            assert!(tick.is_none());
            assert_eq!(tima, i + 1);
        }
        assert!(timer.tick(ticks).is_some());

        // Enable timer, 16384 Hz
        let mut timer = TimerRegisters::new();
        let ticks = TimaSpeed::Clock16k as usize;
        timer.write_byte(TimerControl::Enabled16k as u8, TAC);

        for i in 0..255 {
            let tick = timer.tick(ticks);
            let tima = timer.read_byte(TIMA);

            assert!(tick.is_none());
            assert_eq!(tima, i + 1);
        }
        assert!(timer.tick(ticks).is_some());
    }
}
