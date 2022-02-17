/// Call subroutine unconditionally
/// - - - -
pub fn a16(cpu: &mut crate::cpu::Cpu) {
    let addr = cpu.get_word_argument();
    let pc = cpu.registers.pc;
    cpu.push(pc + 3);
    cpu.registers.pc = addr;
    cpu.branch_taken = true;
}

/// Call subroutine on condition
/// - - -_-
fn call(cpu: &mut crate::cpu::Cpu, condition: bool) {
    if !condition {
        return;
    }

    let addr = cpu.get_word_argument();
    let pc = cpu.registers.pc;
    cpu.push(pc + 3);
    cpu.registers.pc = addr;
    cpu.machine_cycles = 3; // Extra cycles
    cpu.branch_taken = true;
}

/// Call subroutine if Z is set
/// - - - -
pub fn z(cpu: &mut crate::cpu::Cpu) {
    call(cpu, cpu.registers.f.z);
}

/// Call subroutine if Z is not set
/// - - - -
pub fn nz(cpu: &mut crate::cpu::Cpu) {
    call(cpu, !cpu.registers.f.z);
}

/// Call subroutine if C is set
/// - - - -
pub fn c(cpu: &mut crate::cpu::Cpu) {
    call(cpu, cpu.registers.f.c);
}

/// Call subroutine if C is not set
/// - - - -
pub fn nc(cpu: &mut crate::cpu::Cpu) {
    call(cpu, !cpu.registers.f.c);
}

#[cfg(test)]
mod tests {

    fn setup(cpu: &mut crate::cpu::Cpu) {
        let addr: u16 = 0xDEAD;
        cpu.current_argument = Some(addr.into());
        cpu.registers.pc = 0xBEEF;
    }

    #[test]
    fn test_a16() {
        let mut cpu = crate::cpu::Cpu::reset();
        setup(&mut cpu);
        super::a16(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xDEAD);
    }

    #[test]
    fn test_z_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = true;
        setup(&mut cpu);
        super::z(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xDEAD);
        assert_eq!(cpu.machine_cycles, 3);
    }

    #[test]
    fn test_nz_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = false;
        setup(&mut cpu);
        super::nz(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xDEAD);
        assert_eq!(cpu.machine_cycles, 3);
    }

    #[test]
    fn test_c_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = true;
        setup(&mut cpu);
        super::c(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xDEAD);
        assert_eq!(cpu.machine_cycles, 3);
    }

    #[test]
    fn test_nc_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = false;
        setup(&mut cpu);
        super::nc(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xDEAD);
        assert_eq!(cpu.machine_cycles, 3);
    }

    #[test]
    fn test_z_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = false;
        setup(&mut cpu);
        super::z(&mut cpu);
        assert_ne!(cpu.registers.pc, 0xDEAD);
        assert_eq!(cpu.machine_cycles, 0);
    }

    #[test]
    fn test_nz_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = true;
        setup(&mut cpu);
        super::nz(&mut cpu);
        assert_ne!(cpu.registers.pc, 0xDEAD);
        assert_eq!(cpu.machine_cycles, 0);
    }

    #[test]
    fn test_c_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = false;
        setup(&mut cpu);
        super::c(&mut cpu);
        assert_ne!(cpu.registers.pc, 0xDEAD);
        assert_eq!(cpu.machine_cycles, 0);
    }

    #[test]
    fn test_nc_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = true;
        setup(&mut cpu);
        super::nc(&mut cpu);
        assert_ne!(cpu.registers.pc, 0xDEAD);
        assert_eq!(cpu.machine_cycles, 0);
    }
}
