mod opcodes;
use crate::memory::map::MemoryMap;
use opcodes::OpCode;

#[derive(Debug, Clone)]
pub struct Cpu {
    registers: Registers,
    machine_cycles: u8,
    memory: MemoryMap,
}

impl Cpu {
    pub fn reset() -> Self {
        Self {
            registers: Registers {
                pc: 0x100,
                sp: 0xfffe,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: FlagRegister {
                    z: false,
                    n: false,
                    h: false,
                    c: false,
                },
                h: 0,
                l: 0,
            },
            machine_cycles: 0,
            memory: MemoryMap::new(),
        }
    }

    #[allow(dead_code)]
    /// Step through one instrucion
    pub fn step(&mut self) {
        let OpCode(_mnemonic, func, size, cycles) =
            opcodes::OPCODES[self.read_byte(self.registers.pc) as usize];
        func(self);
        self.registers.pc = self.registers.pc.wrapping_add(size as u16);
        self.machine_cycles = cycles;
    }

    /// Step through specific opcode
    pub fn step_op(&mut self, op: usize) {
        let OpCode(mnemonic, func, size, cycles) = opcodes::OPCODES[op];
        println!("{:#04x}: {}", op, mnemonic);
        func(self);
        self.registers.pc = self.registers.pc.wrapping_add(size as u16);
        self.machine_cycles = cycles;
    }

    /// Store a byte at a memory address
    pub fn write_byte(&mut self, byte: u8, addr: u16) {
        self.memory.write_byte(byte, addr);
    }

    /// Retrieve a byte from a memory address
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.memory.read_byte(addr)
    }

    /// Store a word at a memory address
    pub fn write_word(&mut self, word: u16, addr: u16) {
        self.memory.write_word(word, addr);
    }

    /// Retrieve a word from a memory address
    pub fn read_word(&self, addr: u16) -> u16 {
        self.memory.read_word(addr)
    }

    /// Get byte argument of instruction
    pub fn get_byte_argument(&mut self) -> u8 {
        self.read_byte(self.registers.pc + 1)
    }

    /// Get word argument of instruction
    pub fn get_word_argument(&mut self) -> u16 {
        self.read_word(self.registers.pc + 1)
    }
}

#[derive(Debug, Clone)]
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagRegister,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

impl Registers {
    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f.value() as u16
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn put_af(&mut self, word: u16) {
        self.a = (word >> 8) as u8;
        self.f.set((word & 0xff) as u8);
    }

    pub fn put_bc(&mut self, word: u16) {
        self.b = (word >> 8) as u8;
        self.c = (word & 0xff) as u8;
    }

    pub fn put_de(&mut self, word: u16) {
        self.d = (word >> 8) as u8;
        self.e = (word & 0xff) as u8;
    }

    pub fn put_hl(&mut self, word: u16) {
        self.h = (word >> 8) as u8;
        self.l = (word & 0xff) as u8;
    }
}

#[derive(Debug, Clone)]
struct FlagRegister {
    /// Zero flag
    ///
    /// Set if the result of a math operation is zero
    z: bool,

    /// Subtract flag
    ///
    /// Set if a subtraction was performed in the last math operation
    n: bool,

    /// Half Carry flag
    ///
    /// Set if a carry occurred from the lower nybble in the last math operation
    h: bool,

    /// Carry
    ///
    /// Set if a carry occurred from the last math operation
    c: bool,
}

impl FlagRegister {
    #[allow(dead_code)]
    pub fn value(&self) -> u8 {
        let mut value = 0u8;

        if self.z {
            value |= 1 << 7;
        }

        if self.n {
            value |= 1 << 6;
        }

        if self.h {
            value |= 1 << 5;
        }

        if self.c {
            value |= 1 << 4;
        }

        value
    }

    #[allow(dead_code)]
    pub fn set(&mut self, value: u8) {
        self.z = (value & 1 << 7) != 0;
        self.n = (value & 1 << 6) != 0;
        self.h = (value & 1 << 5) != 0;
        self.c = (value & 1 << 4) != 0;
    }
}
