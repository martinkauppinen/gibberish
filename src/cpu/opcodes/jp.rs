/// Jump to address unconditionally
/// - - - -
pub fn a16(cpu: &mut crate::cpu::Cpu) {
    let addr = cpu.get_word_argument();
    cpu.registers.pc = addr;
    cpu.branch_taken = true;
}

/// Jump to address on condition
/// - - -_-
fn jump(cpu: &mut crate::cpu::Cpu, condition: bool) {
    if !condition {
        return;
    }

    let addr = cpu.get_word_argument();
    cpu.registers.pc = addr;
    cpu.machine_cycles = 1; // Extra cycle
    cpu.branch_taken = true;
}

/// Jump to address if Z is set
/// - - - -
pub fn z(cpu: &mut crate::cpu::Cpu) {
    jump(cpu, cpu.registers.f.z);
}

/// Jump to address if Z is not set
/// - - - -
pub fn nz(cpu: &mut crate::cpu::Cpu) {
    jump(cpu, !cpu.registers.f.z);
}

/// Jump to address if C is set
/// - - - -
pub fn c(cpu: &mut crate::cpu::Cpu) {
    jump(cpu, cpu.registers.f.c);
}

/// Jump to address if C is not set
/// - - - -
pub fn nc(cpu: &mut crate::cpu::Cpu) {
    jump(cpu, !cpu.registers.f.c);
}

/// Jump to address in HL unconditionally
/// - - - -
pub fn hl_ind(cpu: &mut crate::cpu::Cpu) {
    let addr = cpu.registers.hl();
    cpu.registers.pc = addr;
    cpu.branch_taken = true;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_a16() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.current_argument = Some(0xbeefu16.into());
        super::a16(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xbeef);
    }

    #[test]
    fn test_hl_ind() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.put_hl(0xbeef);
        super::hl_ind(&mut cpu);
        assert_eq!(cpu.registers.pc, 0xbeef);
    }

    #[test]
    fn test_z_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = true;
        cpu.write_word(0xbeef, cpu.registers.pc + 1);
        cpu.write_byte(0xCA, cpu.registers.pc); // opcode
        cpu.step();
        assert_eq!(cpu.registers.pc, 0xbeef);
        assert_eq!(cpu.machine_cycles, 4);
    }

    #[test]
    fn test_nz_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = false;
        cpu.write_word(0xbeef, cpu.registers.pc + 1);
        cpu.write_byte(0xC2, cpu.registers.pc); // opcode
        cpu.step();
        assert_eq!(cpu.registers.pc, 0xbeef);
        assert_eq!(cpu.machine_cycles, 4);
    }

    #[test]
    fn test_c_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = true;
        cpu.write_word(0xbeef, cpu.registers.pc + 1);
        cpu.write_byte(0xDA, cpu.registers.pc); // opcode
        cpu.step();
        assert_eq!(cpu.registers.pc, 0xbeef);
        assert_eq!(cpu.machine_cycles, 4);
    }

    #[test]
    fn test_nc_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = false;
        cpu.write_word(0xbeef, cpu.registers.pc + 1);
        cpu.write_byte(0xD2, cpu.registers.pc); // opcode
        cpu.step();
        assert_eq!(cpu.registers.pc, 0xbeef);
        assert_eq!(cpu.machine_cycles, 4);
    }

    #[test]
    fn test_z_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = false;
        cpu.write_word(0xbeef, cpu.registers.pc + 1);
        cpu.write_byte(0xCA, cpu.registers.pc); // opcode
        cpu.step();
        assert_ne!(cpu.registers.pc, 0xbeef);
        assert_eq!(cpu.machine_cycles, 3);
    }

    #[test]
    fn test_nz_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.z = true;
        cpu.write_byte(0xC2, cpu.registers.pc); // opcode
        cpu.write_word(0xbeef, cpu.registers.pc + 1);
        cpu.step();
        assert_ne!(cpu.registers.pc, 0xbeef);
        assert_eq!(cpu.machine_cycles, 3);
    }

    #[test]
    fn test_c_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = false;
        cpu.write_word(0xbeef, cpu.registers.pc + 1);
        cpu.write_byte(0xDA, cpu.registers.pc); // opcode
        cpu.step();
        assert_ne!(cpu.registers.pc, 0xbeef);
        assert_eq!(cpu.machine_cycles, 3);
    }

    #[test]
    fn test_nc_not_taken() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = true;
        cpu.write_word(0xbeef, cpu.registers.pc + 1);
        cpu.write_byte(0xD2, cpu.registers.pc); // opcode
        cpu.step();
        assert_ne!(cpu.registers.pc, 0xbeef);
        assert_eq!(cpu.machine_cycles, 3);
    }
}
