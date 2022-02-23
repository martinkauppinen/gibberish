pub enum Interrupt {
    Vblank,
    Lcdc,
    Timer,
    Serial,
    Joypad,
}

impl From<u8> for Interrupt {
    fn from(value: u8) -> Self {
        match value {
            0b00001 => Self::Vblank,
            0b00010 => Self::Lcdc,
            0b00100 => Self::Timer,
            0b01000 => Self::Serial,
            0b10000 => Self::Joypad,
            _ => panic!("Invalid bitmask for interrupt"),
        }
    }
}

impl From<Interrupt> for u8 {
    fn from(value: Interrupt) -> Self {
        match value {
            Interrupt::Vblank => 0b00001,
            Interrupt::Lcdc => 0b00010,
            Interrupt::Timer => 0b00100,
            Interrupt::Serial => 0b01000,
            Interrupt::Joypad => 0b10000,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct InterruptMask {
    vblank: bool,
    lcdc: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl From<u8> for InterruptMask {
    fn from(value: u8) -> Self {
        Self {
            vblank: value & 0b00001 > 0,
            lcdc: value & 0b00010 > 0,
            timer: value & 0b00100 > 0,
            serial: value & 0b01000 > 0,
            joypad: value & 0b10000 > 0,
        }
    }
}

impl From<InterruptMask> for u8 {
    fn from(value: InterruptMask) -> Self {
        let mut ret = 0;

        if value.vblank {
            ret |= 1;
        }

        if value.lcdc {
            ret |= 1 << 1;
        }

        if value.timer {
            ret |= 1 << 2;
        }

        if value.serial {
            ret |= 1 << 3;
        }

        if value.joypad {
            ret |= 1 << 4;
        }

        ret
    }
}

#[derive(Debug, Clone, Default)]
pub struct InterruptController {
    request: InterruptMask,
    enabled: InterruptMask,
}

impl InterruptController {
    /// Returns the pending interrupt with the highest priority
    pub fn get_pending_interrupt(&mut self) -> Option<Interrupt> {
        let bits: u8 = self.request.into();

        // None of the 5 interrupts set
        if bits.trailing_zeros() > 4 {
            return None;
        }

        let mut highest_prio: Option<u8> = None;
        for i in bits.trailing_zeros()..=4 {
            // Lowest set interrupt bit has highest priority
            let interrupt_bit = 1 << i;
            if u8::from(self.enabled) & interrupt_bit > 0 {
                highest_prio = Some(interrupt_bit);
                break;
            }
        }

        // Check that an enabled interrupt was found
        highest_prio?;

        // Reset interrupt flag
        self.request = (u8::from(self.request) & !highest_prio.unwrap()).into();

        let interrupt = highest_prio.unwrap().into();
        Some(interrupt)
    }

    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        let interrupt_number: u8 = interrupt.into();
        self.request = (u8::from(self.request) | interrupt_number).into();
    }

    pub fn enable_interrupt(&mut self, interrupt: Interrupt) {
        let interrupt_number: u8 = interrupt.into();
        self.enabled = (u8::from(self.enabled) | interrupt_number).into();
    }

    pub fn interrupts_pending(&self) -> bool {
        u8::from(self.request) & u8::from(self.enabled) != 0
    }

    pub fn interrupts_requested(&self) -> bool {
        u8::from(self.request) != 0
    }
}
