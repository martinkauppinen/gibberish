mod interrupts;
mod opcodes;

use crate::memory::map::MemoryMap;
use interrupts::{Interrupt, InterruptController};
use opcodes::{Argument, OpCode};

#[derive(Debug, Clone)]
enum RunningMode {
    PowerUp,
    Stop,
    Halt,
    Running,
}

impl Default for RunningMode {
    fn default() -> Self {
        RunningMode::PowerUp
    }
}

#[derive(Debug, Clone, Default)]
pub struct Cpu {
    registers: Registers,
    machine_cycles: u8,
    memory: MemoryMap,
    current_instruction: u8,
    current_argument: Option<Argument>,
    branch_taken: bool,
    interrupt_master_enable: bool,
    mode: RunningMode,
    interrupts: InterruptController,
}

impl Cpu {
    pub fn reset() -> Self {
        let mut cpu = Self::default();

        // TODO: (maybe)
        // - Run startup ROM at 0x00 - 0xFF
        // - Validate checksum

        cpu.registers.pc = 0x100;
        cpu.registers.a = 0x01;
        cpu.registers.f.set(0xB0);
        cpu.registers.put_bc(0x0013);
        cpu.registers.put_de(0x00D8);
        cpu.registers.put_hl(0x014D);
        cpu.registers.sp = 0xFFFE;

        // Timer registers
        cpu.write_byte(0x00, 0xFF05); // TIMA
        cpu.write_byte(0x00, 0xFF06); // TMA
        cpu.write_byte(0x00, 0xFF07); // TAC

        // Sound registers
        cpu.write_byte(0x80, 0xFF10); // NR10
        cpu.write_byte(0xBF, 0xFF11); // NR11
        cpu.write_byte(0xF3, 0xFF12); // NR12
        cpu.write_byte(0xBF, 0xFF14); // NR14
        cpu.write_byte(0x3F, 0xFF16); // NR21
        cpu.write_byte(0x00, 0xFF17); // NR22
        cpu.write_byte(0xBF, 0xFF19); // NR24
        cpu.write_byte(0x7F, 0xFF1A); // NR30
        cpu.write_byte(0xFF, 0xFF1B); // NR31
        cpu.write_byte(0x9F, 0xFF1C); // NR32
        cpu.write_byte(0xBF, 0xFF1E); // NR33
        cpu.write_byte(0xFF, 0xFF20); // NR41
        cpu.write_byte(0x00, 0xFF21); // NR42
        cpu.write_byte(0x00, 0xFF22); // NR43
        cpu.write_byte(0xBF, 0xFF23); // NR44
        cpu.write_byte(0x77, 0xFF24); // NR50
        cpu.write_byte(0xF3, 0xFF25); // NR51
        cpu.write_byte(0xF1, 0xFF26); // NR52

        // Display registers
        cpu.write_byte(0x91, 0xFF40); // LCDC
        cpu.write_byte(0x00, 0xFF42); // SCY
        cpu.write_byte(0x00, 0xFF43); // SCX
        cpu.write_byte(0x00, 0xFF45); // LYC
        cpu.write_byte(0xFC, 0xFF47); // BGP
        cpu.write_byte(0xFF, 0xFF48); // OBP0
        cpu.write_byte(0xFF, 0xFF49); // OBP1
        cpu.write_byte(0x00, 0xFF4A); // WY
        cpu.write_byte(0x00, 0xFF4B); // WX

        // Interrupt enable flag
        cpu.write_byte(0x00, 0xFFFF); // IE

        cpu.mode = RunningMode::Running;
        cpu
    }

    #[allow(dead_code)]
    /// Step through one instrucion
    pub fn step(&mut self) {
        self.handle_interrupts();
        self.current_instruction = self.read_byte(self.registers.pc);
        let OpCode(_mnemonic, func, size, cycles) =
            opcodes::OPCODES[self.current_instruction as usize];
        match size {
            2 => {
                self.current_argument = Some(Argument::Byte(self.read_byte(self.registers.pc + 1)))
            }
            3 => {
                self.current_argument = Some(Argument::Word(self.read_word(self.registers.pc + 1)))
            }
            _ => self.current_argument = None,
        }
        self.machine_cycles = 0;
        self.branch_taken = false;

        match self.mode {
            RunningMode::Stop => todo!(),
            RunningMode::Halt => todo!(),
            _ => func(self),
        }

        self.machine_cycles += cycles;

        if !self.branch_taken {
            self.registers.pc = self.registers.pc.wrapping_add(size as u16);
        }
    }

    /// Check for interrupts and handle them if enabled
    fn handle_interrupts(&mut self) {
        if !self.interrupt_master_enable {
            return;
        }

        if let Some(interrupt) = self.interrupts.get_pending_interrupt() {
            self.push(self.registers.pc);
            self.interrupt_master_enable = false;
            match interrupt {
                Interrupt::Vblank => self.registers.pc = 0x0040,
                Interrupt::Lcdc => self.registers.pc = 0x0048,
                Interrupt::Timer => self.registers.pc = 0x0050,
                Interrupt::Serial => self.registers.pc = 0x0058,
                Interrupt::Joypad => self.registers.pc = 0x0060,
            }

            // Dispatching takes 5 machine cycles
            self.machine_cycles = 5;
        }
    }

    /// Print method for debugging
    pub fn print_status(&self) {
        let current_instruction = self.read_byte(self.registers.pc);
        let OpCode(mnemonic, _, size, _) = opcodes::OPCODES[current_instruction as usize];

        let argument: Option<Argument>;
        match size {
            2 => argument = Some(Argument::Byte(self.read_byte(self.registers.pc + 1))),
            3 => argument = Some(Argument::Word(self.read_word(self.registers.pc + 1))),
            _ => argument = None,
        }

        print!("A: {:02x} | ", self.registers.a);
        print!("B: {:02x} | ", self.registers.b);
        print!("C: {:02x} | ", self.registers.c);
        print!("D: {:02x} | ", self.registers.d);
        print!("E: {:02x} | ", self.registers.e);
        print!("F: {:08b} | ", self.registers.f.value());
        print!("H: {:02x} | ", self.registers.h);
        print!("L: {:02x} | ", self.registers.l);
        print!("SP: {:04x} | ", self.registers.sp);
        println!("PC: {:04x}", self.registers.pc);
        println!("Instruction at PC: {mnemonic}");
        println!("Argument: {:x?}", argument);
    }

    #[allow(dead_code)]
    /// Step through specific opcode
    pub fn step_op(&mut self, op: usize) {
        self.current_instruction = op as u8;
        let OpCode(mnemonic, func, size, cycles) = opcodes::OPCODES[op];
        match size {
            2 => {
                self.current_argument = Some(Argument::Byte(self.read_byte(self.registers.pc + 1)))
            }
            3 => {
                self.current_argument = Some(Argument::Word(self.read_word(self.registers.pc + 1)))
            }
            _ => self.current_argument = None,
        }
        println!("{:#04x}: {}", op, mnemonic);
        self.machine_cycles = 0;
        self.branch_taken = false;
        func(self);

        if !self.branch_taken {
            self.registers.pc = self.registers.pc.wrapping_add(size as u16);
        }

        self.machine_cycles += cycles;
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
        if let Some(Argument::Byte(arg)) = self.current_argument {
            arg
        } else {
            panic!("Expected byte argument, found {:?}", self.current_argument);
        }
    }

    /// Get word argument of instruction
    pub fn get_word_argument(&mut self) -> u16 {
        if let Some(Argument::Word(arg)) = self.current_argument {
            arg
        } else {
            panic!("Expected word argument, found {:?}", self.current_argument);
        }
    }

    /// Push a word to the stack
    pub fn push(&mut self, word: u16) {
        let [hi, lo] = word.to_be_bytes();
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_byte(hi, self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_byte(lo, self.registers.sp);
    }

    /// Pop a word off the stack
    pub fn pop(&mut self) -> u16 {
        let lo = self.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let hi = self.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);

        u16::from_be_bytes([hi, lo])
    }
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
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
