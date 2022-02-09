pub mod addr {
    /// Load offset from 0xFF00 by immediate argument with value of A
    /// - - - -
    pub fn a(cpu: &mut crate::cpu::Cpu) {
        let addr: u16 = 0xFF00 + cpu.get_byte_argument() as u16;
        cpu.write_byte(cpu.registers.a, addr);
    }
}

pub mod a {
    /// Load A with value in memory offset from 0xFF00 by immediate argument
    /// - - - -
    pub fn addr(cpu: &mut crate::cpu::Cpu) {
        let addr: u16 = 0xFF00 + cpu.get_byte_argument() as u16;
        cpu.registers.a = cpu.read_byte(addr);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn a_addr() {
        let value = 0xAB;
        let offset = 0xBA;
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.write_byte(offset, cpu.registers.pc + 1);
        cpu.write_byte(value, 0xFF00 + offset as u16);
        super::a::addr(&mut cpu);
        assert_eq!(cpu.registers.a, value);
    }

    #[test]
    fn addr_a() {
        let value = 0xAB;
        let offset = 0xBA;
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.write_byte(offset, cpu.registers.pc + 1);
        cpu.registers.a = value;
        super::addr::a(&mut cpu);
        assert_eq!(cpu.read_byte(0xFF00 + offset as u16), value);
    }
}
