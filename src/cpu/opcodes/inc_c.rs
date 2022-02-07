use crate::cpu::Cpu;

/// 0x0C: Increment C
/// Z 0 H -
pub fn inc_c(cpu: &mut Cpu) {
    cpu.registers.f.n = false;

    let c_old = cpu.registers.c;

    cpu.registers.c = c_old.wrapping_add(1);
    cpu.registers.f.h = super::half_carry(c_old, cpu.registers.c);

    if cpu.registers.c == 0 {
        cpu.registers.f.z = true;
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::Cpu;

    #[test]
    fn functionality() {
        let mut cpu = Cpu::reset();
        super::inc_c(&mut cpu);
        assert_eq!(cpu.registers.c, 1);
        assert!(!cpu.registers.f.z);
        assert!(!cpu.registers.f.n);
        assert!(!cpu.registers.f.h);
    }

    #[test]
    fn half_carry() {
        let mut cpu = Cpu::reset();
        cpu.registers.c = 0x0F;
        super::inc_c(&mut cpu);
        assert_eq!(cpu.registers.c, 0x10);
        assert!(!cpu.registers.f.z);
        assert!(cpu.registers.f.h);
    }

    #[test]
    fn wrapping() {
        let mut cpu = Cpu::reset();
        cpu.registers.c = 0xFF;
        super::inc_c(&mut cpu);
        assert_eq!(cpu.registers.c, 0);
        assert!(cpu.registers.f.z);
        assert!(cpu.registers.f.h);
    }
}
