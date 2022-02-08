use crate::cpu::Cpu;

macro_rules! inc {
    ($reg:ident) => {
        /// Increment a register
        /// Z 0 H -
        pub fn $reg(cpu: &mut Cpu) {
            cpu.registers.f.n = false;

            let reg_old = cpu.registers.$reg;

            cpu.registers.$reg = reg_old.wrapping_add(1);
            cpu.registers.f.h = super::half_carry(reg_old, cpu.registers.$reg);
            cpu.registers.f.z = cpu.registers.$reg == 0;
        }
    };

    ($name:ident, $reg_hi:ident, $reg_lo:ident) => {
        /// Increment a combined register
        /// - - - -
        pub fn $name(cpu: &mut Cpu) {
            cpu.registers.$reg_lo = cpu.registers.$reg_lo.wrapping_add(1);
            if cpu.registers.$reg_lo == 0 {
                cpu.registers.$reg_hi = cpu.registers.$reg_hi.wrapping_add(1);
            }
        }
    };
}

inc!(a);
inc!(b);
inc!(c);
inc!(d);
inc!(e);
inc!(h);
inc!(l);
inc!(bc, b, c);
inc!(de, d, e);
inc!(hl, h, l);

/// Increment SP
/// - - - -
pub fn sp(cpu: &mut Cpu) {
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
}

/// Increment the value at the address pointed to by HL
/// Z 0 H -
pub fn hl_ind(cpu: &mut Cpu) {
    cpu.registers.f.n = false;

    let value_old = cpu.read_byte(cpu.registers.hl());
    let value_new = value_old.wrapping_add(1);

    cpu.write_byte(value_new, cpu.registers.hl());
    cpu.registers.f.h = super::half_carry(value_old, value_new);

    if value_new == 0 {
        cpu.registers.f.z = true;
    }
}
