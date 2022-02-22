use crate::cpu::Cpu;

macro_rules! inc {
    ($reg:ident) => {
        /// Increment a register
        /// Z 0 H -
        pub fn $reg(cpu: &mut Cpu) {
            cpu.registers.f.n = false;

            let reg_old = cpu.registers.$reg;

            cpu.registers.$reg = reg_old.wrapping_add(1);
            cpu.registers.f.h = super::half_carry_add(reg_old, cpu.registers.$reg);
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
    cpu.registers.f.h = super::half_carry_add(value_old, value_new);

    cpu.registers.f.z = value_new == 0;
}

#[cfg(test)]
mod test {
    macro_rules! test_reg {
        ($reg:ident) => {
            mod $reg {
                use crate::cpu::Cpu;

                fn setup(init: u8) -> Cpu {
                    let mut cpu = Cpu::reset();
                    cpu.registers.$reg = init;
                    super::super::$reg(&mut cpu);
                    cpu
                }

                #[test]
                fn simple_inc() {
                    let cpu = setup(3);
                    assert_eq!(cpu.registers.$reg, 4);
                    assert!(!cpu.registers.f.z);
                    assert!(!cpu.registers.f.n);
                    assert!(!cpu.registers.f.h);
                }

                #[test]
                fn half_carry() {
                    let cpu = setup(0xF);
                    assert_eq!(cpu.registers.$reg, 0x10);
                    assert!(!cpu.registers.f.z);
                    assert!(!cpu.registers.f.n);
                    assert!(cpu.registers.f.h);
                }

                #[test]
                fn overflow_zero() {
                    let cpu = setup(0xFF);
                    assert_eq!(cpu.registers.$reg, 0x00);
                    assert!(cpu.registers.f.z);
                    assert!(!cpu.registers.f.n);
                    assert!(cpu.registers.f.h);
                }
            }
        };
    }

    macro_rules! test_reg_pair {
        ($func:ident, $reg_hi:ident, $reg_lo:ident) => {
            mod $func {
                use crate::cpu::Cpu;

                #[test]
                fn simple_inc() {
                    let mut cpu = Cpu::reset();
                    cpu.registers.$reg_lo = 3;
                    cpu.registers.$reg_hi = 0;
                    super::super::$func(&mut cpu);
                    assert_eq!(cpu.registers.$reg_lo, 4);
                    assert_eq!(cpu.registers.$reg_hi, 0);
                }

                #[test]
                fn carry_bytes() {
                    let mut cpu = Cpu::reset();
                    cpu.registers.$reg_lo = 0xFF;
                    cpu.registers.$reg_hi = 0;
                    super::super::$func(&mut cpu);
                    assert_eq!(cpu.registers.$func(), 0x01_00);
                }

                #[test]
                fn overflow_zero() {
                    let mut cpu = Cpu::reset();
                    cpu.registers.$reg_hi = 0xFF;
                    cpu.registers.$reg_lo = 0xFF;
                    super::super::$func(&mut cpu);
                    assert_eq!(cpu.registers.$func(), 0);
                }
            }
        };
    }

    test_reg!(a);
    test_reg!(b);
    test_reg!(c);
    test_reg!(d);
    test_reg!(e);
    test_reg!(h);
    test_reg!(l);
    test_reg_pair!(bc, b, c);
    test_reg_pair!(de, d, e);
    test_reg_pair!(hl, h, l);

    mod sp {
        use crate::cpu::Cpu;

        #[test]
        fn simple_inc() {
            let mut cpu = Cpu::reset();
            cpu.registers.sp = 3;
            super::super::sp(&mut cpu);
            assert_eq!(cpu.registers.sp, 4);
        }

        #[test]
        fn overflow_zero() {
            let mut cpu = Cpu::reset();
            cpu.registers.sp = 0xFF_FF;
            super::super::sp(&mut cpu);
            assert_eq!(cpu.registers.sp, 0);
        }
    }

    mod hl_ind {
        use crate::cpu::Cpu;

        fn setup(init: u8, addr: u16) -> Cpu {
            let mut cpu = Cpu::reset();
            cpu.registers.put_hl(addr);
            cpu.write_byte(init, addr);
            super::super::hl_ind(&mut cpu);
            cpu
        }

        #[test]
        fn simple_inc() {
            let cpu = setup(3, 0x100);
            assert_eq!(cpu.read_byte(0x100), 4);
            assert!(!cpu.registers.f.z);
            assert!(!cpu.registers.f.n);
            assert!(!cpu.registers.f.h);
        }

        #[test]
        fn half_carry() {
            let cpu = setup(0x0F, 0x100);
            assert_eq!(cpu.read_byte(0x100), 0x10);
            assert!(!cpu.registers.f.z);
            assert!(!cpu.registers.f.n);
            assert!(cpu.registers.f.h);
        }

        #[test]
        fn overflow_zero() {
            let cpu = setup(0xFF, 0x100);
            assert_eq!(cpu.read_byte(0x100), 0x00);
            assert!(cpu.registers.f.z);
            assert!(!cpu.registers.f.n);
            assert!(cpu.registers.f.h);
        }
    }
}
