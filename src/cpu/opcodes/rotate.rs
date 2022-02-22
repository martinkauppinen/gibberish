/// Rotate A register left, storing old high bit in carry flag
/// 0 0 0 C
pub fn rlca(cpu: &mut crate::cpu::Cpu) {
    let old_high_bit = cpu.registers.a & 0x80;
    cpu.registers.f.c = cpu.registers.a & 0x80 > 0;
    cpu.registers.a <<= 1;
    cpu.registers.a |= old_high_bit >> 7;
    cpu.registers.f.z = cpu.registers.a == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
}

/// Rotate A register left, using carry flag as 9th bit
/// 0 0 0 C
pub fn rla(cpu: &mut crate::cpu::Cpu) {
    let old_carry = cpu.registers.f.c as u8;
    cpu.registers.f.c = cpu.registers.a & 0x80 > 0;
    cpu.registers.a <<= 1;
    cpu.registers.a |= old_carry;
    cpu.registers.f.z = cpu.registers.a == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
}

/// Rotate A register right, storing old low bit in carry flag
/// 0 0 0 C
pub fn rrca(cpu: &mut crate::cpu::Cpu) {
    let old_low_bit = cpu.registers.a & 0x01;
    cpu.registers.f.c = cpu.registers.a & 0x01 > 0;
    cpu.registers.a >>= 1;
    cpu.registers.a |= old_low_bit << 7;
    cpu.registers.f.z = cpu.registers.a == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
}

/// Rotate A register right, using carry flag as 9th bit
/// 0 0 0 C
pub fn rra(cpu: &mut crate::cpu::Cpu) {
    let old_carry = cpu.registers.f.c as u8;
    cpu.registers.f.c = cpu.registers.a & 0x01 > 0;
    cpu.registers.a >>= 1;
    cpu.registers.a |= old_carry << 7;
    cpu.registers.f.z = cpu.registers.a == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
}

#[cfg(test)]
mod test {

    #[test]
    fn rlca() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.a = 0b1010_1010;
        super::rlca(&mut cpu);
        assert!(cpu.registers.f.c);
        assert_eq!(cpu.registers.a, 0b0101_0101);
    }

    #[test]
    fn rrca() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = true; // set to make sure it changes
        cpu.registers.a = 0b1010_1010;
        super::rrca(&mut cpu);
        assert!(!cpu.registers.f.c);
        assert_eq!(cpu.registers.a, 0b0101_0101);
    }

    #[test]
    fn rla() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.set(0);
        cpu.registers.a = 0b1010_1010;
        super::rla(&mut cpu);
        assert!(cpu.registers.f.c);
        assert_eq!(cpu.registers.a, 0b0101_0100);
    }

    #[test]
    fn rra() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = true; // set to make sure it changes
        cpu.registers.a = 0b1010_1010;
        super::rra(&mut cpu);
        assert!(!cpu.registers.f.c);
        assert_eq!(cpu.registers.a, 0b1101_0101);
    }
}
