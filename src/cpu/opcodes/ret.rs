/// Return from subroutine unconditionally
/// - - - -
pub fn ret(cpu: &mut crate::cpu::Cpu) {
    cpu.registers.pc = cpu.pop();
    cpu.inhibit_pc = true;
}

/// Return from subroutine on condition
/// - - -_-
fn ret_cond(cpu: &mut crate::cpu::Cpu, condition: bool) {
    if !condition {
        return;
    }

    cpu.machine_cycles = 3; // Extra cycles
    ret(cpu);
}

/// Return from subroutine if Z is set
/// - - - -
pub fn z(cpu: &mut crate::cpu::Cpu) {
    ret_cond(cpu, cpu.registers.f.z);
}

/// Return from subroutine if Z is not set
/// - - - -
pub fn nz(cpu: &mut crate::cpu::Cpu) {
    ret_cond(cpu, !cpu.registers.f.z);
}

/// Return from subroutine if C is set
/// - - - -
pub fn c(cpu: &mut crate::cpu::Cpu) {
    ret_cond(cpu, cpu.registers.f.c);
}

/// Return from subroutine if C is not set
/// - - - -
pub fn nc(cpu: &mut crate::cpu::Cpu) {
    ret_cond(cpu, !cpu.registers.f.c);
}

#[cfg(test)]
mod tests {

    fn setup(cpu: &mut crate::cpu::Cpu) {
        cpu.push(0xBEEF);
    }

    #[test]
    fn test_a16() {
        let mut cpu = crate::cpu::Cpu::reset();
        setup(&mut cpu);
        super::ret(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xBEEF);
    }

    #[test]
    fn test_z_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = true;
        setup(&mut cpu);
        super::z(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xBEEF);
        assert_eq!(cpu.machine_cycles, 3);
    }

    #[test]
    fn test_nz_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = false;
        setup(&mut cpu);
        super::nz(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xBEEF);
        assert_eq!(cpu.machine_cycles, 3);
    }

    #[test]
    fn test_c_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = true;
        setup(&mut cpu);
        super::c(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xBEEF);
        assert_eq!(cpu.machine_cycles, 3);
    }

    #[test]
    fn test_nc_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = false;
        setup(&mut cpu);
        super::nc(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xBEEF);
        assert_eq!(cpu.machine_cycles, 3);
    }

    #[test]
    fn test_z_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = false;
        setup(&mut cpu);
        super::z(&mut cpu);
        assert_ne!(cpu.registers.pc, 0xBEEF);
        assert_eq!(cpu.machine_cycles, 0);
    }

    #[test]
    fn test_nz_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = true;
        setup(&mut cpu);
        super::nz(&mut cpu);
        assert_ne!(cpu.registers.pc, 0xBEEF);
        assert_eq!(cpu.machine_cycles, 0);
    }

    #[test]
    fn test_c_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = false;
        setup(&mut cpu);
        super::c(&mut cpu);
        assert_ne!(cpu.registers.pc, 0xBEEF);
        assert_eq!(cpu.machine_cycles, 0);
    }

    #[test]
    fn test_nc_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = true;
        setup(&mut cpu);
        super::nc(&mut cpu);
        assert_ne!(cpu.registers.pc, 0xBEEF);
        assert_eq!(cpu.machine_cycles, 0);
    }
}
