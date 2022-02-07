use crate::cpu::Cpu;

macro_rules! dec {
    ($reg:ident) => {
        /// Decrement a register
        /// Z 1 H -
        pub fn $reg(cpu: &mut Cpu) {
            cpu.registers.f.n = true;

            let reg_old = cpu.registers.$reg;

            cpu.registers.$reg = reg_old.wrapping_sub(1);
            cpu.registers.f.h = super::half_carry(reg_old, cpu.registers.$reg);
            cpu.registers.f.z = cpu.registers.$reg == 0;
        }
    };

    ($name:ident, $reg_hi:ident, $reg_lo:ident) => {
        /// Decrement a combined register
        /// - - - -
        pub fn $name(cpu: &mut Cpu) {
            cpu.registers.$reg_lo = cpu.registers.$reg_lo.wrapping_sub(1);
            if cpu.registers.$reg_lo == 0xFF {
                cpu.registers.$reg_hi = cpu.registers.$reg_hi.wrapping_sub(1);
            }
        }
    };
}

dec!(a);
dec!(b);
dec!(c);
dec!(d);
dec!(e);
dec!(h);
dec!(l);
dec!(bc, b, c);
dec!(de, d, e);
dec!(hl, h, l);

/// Decrement SP
/// - - - -
pub fn sp(cpu: &mut Cpu) {
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
}

/// Decrement the value at the address pointed to by HL
/// Z 1 H -
pub fn hl_ind(cpu: &mut Cpu) {
    cpu.registers.f.n = false;

    let value_old = cpu.get_byte(cpu.registers.hl());
    let value_new = value_old.wrapping_sub(1);

    cpu.put_byte(cpu.registers.hl(), value_new);
    cpu.registers.f.h = super::half_carry(value_old, value_new);

    if value_new == 0 {
        cpu.registers.f.z = true;
    }
}
