use super::sign_extend;

/// Add sign extended immediate byte to PC unconditionally
/// - - - -
pub fn r8(cpu: &mut crate::cpu::Cpu) {
    let addr = sign_extend(cpu.get_byte_argument());
    cpu.registers.pc = cpu.registers.pc.wrapping_add(addr);
}

/// Add sign extended immediate byte to PC if Z is set
/// - - - -
pub fn z(cpu: &mut crate::cpu::Cpu) {
    let addr = sign_extend(cpu.get_byte_argument());
    if cpu.registers.f.z {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(addr);
        cpu.machine_cycles = 1; // Extra cycle
        cpu.branch_taken = true;
    }
}

/// Add sign extended immediate byte to PC if Z is not set
/// - - - -
pub fn nz(cpu: &mut crate::cpu::Cpu) {
    let addr = sign_extend(cpu.get_byte_argument());
    if !cpu.registers.f.z {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(addr);
        cpu.machine_cycles = 1; // Extra cycle
        cpu.branch_taken = true;
    }
}

/// Add sign extended immediate byte to PC if C is set
/// - - - -
pub fn c(cpu: &mut crate::cpu::Cpu) {
    let addr = sign_extend(cpu.get_byte_argument());
    if cpu.registers.f.c {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(addr);
        cpu.machine_cycles = 1; // Extra cycle
        cpu.branch_taken = true;
    }
}

/// Add sign extended immediate byte to PC if C is not set
/// - - - -
pub fn nc(cpu: &mut crate::cpu::Cpu) {
    let addr = sign_extend(cpu.get_byte_argument());
    if !cpu.registers.f.c {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(addr);
        cpu.machine_cycles = 1; // Extra cycle
        cpu.branch_taken = true;
    }
}

#[cfg(test)]
mod tests {

    fn setup(cpu: &mut crate::cpu::Cpu) {
        let byte: u8 = 0xFE; // Signed -2
        cpu.current_argument = Some(byte.into());
        cpu.registers.pc = 3;
    }

    #[test]
    fn test_r8() {
        let mut cpu = crate::cpu::Cpu::reset();
        setup(&mut cpu);
        super::r8(&mut cpu);
        assert_eq!(cpu.registers.pc, 0x1);
    }

    #[test]
    fn test_z_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = true;
        setup(&mut cpu);
        super::z(&mut cpu);
        assert_eq!(cpu.registers.pc, 0x1);
        assert_eq!(cpu.machine_cycles, 1);
    }

    #[test]
    fn test_nz_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = false;
        setup(&mut cpu);
        super::nz(&mut cpu);
        assert_eq!(cpu.registers.pc, 0x1);
        assert_eq!(cpu.machine_cycles, 1);
    }

    #[test]
    fn test_c_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = true;
        setup(&mut cpu);
        super::c(&mut cpu);
        assert_eq!(cpu.registers.pc, 0x1);
        assert_eq!(cpu.machine_cycles, 1);
    }

    #[test]
    fn test_nc_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = false;
        setup(&mut cpu);
        super::nc(&mut cpu);
        assert_eq!(cpu.registers.pc, 0x1);
        assert_eq!(cpu.machine_cycles, 1);
    }

    #[test]
    fn test_z_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = false;
        setup(&mut cpu);
        super::z(&mut cpu);
        assert_ne!(cpu.registers.pc, 0x1);
        assert_eq!(cpu.machine_cycles, 0);
    }

    #[test]
    fn test_nz_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = true;
        setup(&mut cpu);
        super::nz(&mut cpu);
        assert_ne!(cpu.registers.pc, 0x1);
        assert_eq!(cpu.machine_cycles, 0);
    }

    #[test]
    fn test_c_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = false;
        setup(&mut cpu);
        super::c(&mut cpu);
        assert_ne!(cpu.registers.pc, 0x1);
        assert_eq!(cpu.machine_cycles, 0);
    }

    #[test]
    fn test_nc_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = true;
        setup(&mut cpu);
        super::nc(&mut cpu);
        assert_ne!(cpu.registers.pc, 0x1);
        assert_eq!(cpu.machine_cycles, 0);
    }
}
