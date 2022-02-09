macro_rules! op {
    ($operation:ident, $op_symbol:tt, $set_h:literal, $( $reg:ident ),+) => {
        pub mod $operation {
            $(
                /// Bitwise $operation A with $reg
                /// Z 0 $set_h 0
                pub fn $reg(cpu: &mut crate::cpu::Cpu) {
                    cpu.registers.a = cpu.registers.a $op_symbol cpu.registers.$reg;

                    cpu.registers.f.z = cpu.registers.a == 0;
                    cpu.registers.f.n = false;
                    cpu.registers.f.h = $set_h;
                    cpu.registers.f.c = false;
                }
             )+

                /// Bitwise $operation A with value in memory pointed to by HL
                /// Z 0 $set_h 0
                pub fn hl_ind(cpu: &mut crate::cpu::Cpu) {
                    cpu.registers.a = cpu.registers.a $op_symbol cpu.read_byte(cpu.registers.hl());

                    cpu.registers.f.z = cpu.registers.a == 0;
                    cpu.registers.f.n = false;
                    cpu.registers.f.h = $set_h;
                    cpu.registers.f.c = false;
                }

                /// Bitwise $operation A with immediate value
                /// Z 0 $set_h 0
                pub fn imm(cpu: &mut crate::cpu::Cpu) {
                    cpu.registers.a = cpu.registers.a $op_symbol cpu.get_byte_argument();

                    cpu.registers.f.z = cpu.registers.a == 0;
                    cpu.registers.f.n = false;
                    cpu.registers.f.h = $set_h;
                    cpu.registers.f.c = false;
                }
        }
    };
}

op!(and, &, false, b, c, d, e, h, l, a);
op!(or, |, false, b, c, d, e, h, l, a);
op!(xor, ^, true, b, c, d, e, h, l, a);

macro_rules! cp {
    ($( $reg:ident ),+) => {
        pub mod cp {
            use super::super::half_carry_sub;

            $(
                pub fn $reg(cpu: &mut crate::cpu::Cpu) {
                    cpu.registers.f.z = cpu.registers.a == cpu.registers.$reg;
                    cpu.registers.f.n = true;
                    cpu.registers.f.h = half_carry_sub(cpu.registers.a, cpu.registers.$reg);
                    cpu.registers.f.c = cpu.registers.a < cpu.registers.$reg;
                }
             )+

                pub fn hl_ind(cpu: &mut crate::cpu::Cpu) {
                    let byte = cpu.read_byte(cpu.registers.hl());
                    cpu.registers.f.z = cpu.registers.a == byte;
                    cpu.registers.f.n = true;
                    cpu.registers.f.h = half_carry_sub(cpu.registers.a, byte);
                    cpu.registers.f.c = cpu.registers.a < byte;
                }

                pub fn imm(cpu: &mut crate::cpu::Cpu) {
                    let byte = cpu.get_byte_argument();
                    cpu.registers.f.z = cpu.registers.a == byte;
                    cpu.registers.f.n = true;
                    cpu.registers.f.h = half_carry_sub(cpu.registers.a, byte);
                    cpu.registers.f.c = cpu.registers.a < byte;
                }
        }
    };
}

cp!(b, c, d, e, h, l, a);

/// Complement A register
/// - 1 1 -
pub fn cpl(cpu: &mut crate::cpu::Cpu) {
    cpu.registers.f.n = true;
    cpu.registers.f.h = true;
    cpu.registers.a = !cpu.registers.a;
}

/// Complement carry flag
/// - 0 0 C
pub fn ccf(cpu: &mut crate::cpu::Cpu) {
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = !cpu.registers.f.c;
}

/// Set carry flag
/// - 0 0 C
pub fn scf(cpu: &mut crate::cpu::Cpu) {
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = true;
}

/// Decimal adjust accumulator
/// Z - 0 C
///
/// Makes sure upper and lower nybble contain valid BCD digits
/// (i.e. are not greater than 9) by adding 6 to the nybbles
/// if invalid, or their respective carry flags are set
pub fn daa(cpu: &mut crate::cpu::Cpu) {
    if cpu.registers.a & 0xf > 0x09 || cpu.registers.f.h {
        cpu.registers.a = cpu.registers.a.wrapping_add(0x06);
    }

    if cpu.registers.a & 0xf0 > 0x90 || cpu.registers.f.c {
        cpu.registers.a = cpu.registers.a.wrapping_add(0x60);
        cpu.registers.f.c = true;
    } else {
        cpu.registers.f.c = false;
    }

    cpu.registers.f.z = cpu.registers.a == 0;
    cpu.registers.f.h = false;
}

#[cfg(test)]
mod test {
    macro_rules! gen_logical_tests {
        ($operation:ident, $op_symbol:tt, $( $reg:ident ),+) => {
            mod $operation {
                $(
                    #[test]
                    fn $reg() {
                        let mut cpu = crate::cpu::Cpu::reset();
                        cpu.registers.a = 0xAA;
                        cpu.registers.$reg = 0x55;
                        super::super::$operation::$reg(&mut cpu);
                        assert_eq!(cpu.registers.a, 0xAA $op_symbol 0x55);
                    }
                 )+

                    #[test]
                    fn a() {
                        let mut cpu = crate::cpu::Cpu::reset();
                        cpu.registers.a = 0xAA;
                        super::super::$operation::a(&mut cpu);
                        assert_eq!(cpu.registers.a, 0xAA $op_symbol 0xAA);
                    }

                    #[test]
                    fn hl_ind() {
                        let mut cpu = crate::cpu::Cpu::reset();
                        cpu.registers.a = 0xAA;
                        cpu.registers.put_hl(0xBEEF);
                        cpu.write_byte(0x55, cpu.registers.hl());
                        super::super::$operation::hl_ind(&mut cpu);
                        assert_eq!(cpu.registers.a, 0xAA $op_symbol 0x55);
                    }

                    #[test]
                    fn imm() {
                        let mut cpu = crate::cpu::Cpu::reset();
                        cpu.registers.a = 0xAA;
                        cpu.write_byte(0x55, cpu.registers.pc + 1);
                        super::super::$operation::imm(&mut cpu);
                        assert_eq!(cpu.registers.a, 0xAA $op_symbol 0x55);
                    }
            }
        };
    }

    gen_logical_tests!(and, &, b, c, d, e, h, l);
    gen_logical_tests!(or, |, b, c, d, e, h, l);
    gen_logical_tests!(xor, ^, b, c, d, e, h, l);

    #[test]
    fn cpl() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.a = 0xAA;
        super::cpl(&mut cpu);
        assert_eq!(cpu.registers.a, !0xAA);
    }

    #[test]
    fn ccf() {
        let mut cpu = crate::cpu::Cpu::reset();
        cpu.registers.f.c = false;
        super::ccf(&mut cpu);
        assert!(cpu.registers.f.c);
        super::ccf(&mut cpu);
        assert!(!cpu.registers.f.c);
    }

    #[test]
    fn scf() {
        let mut cpu = crate::cpu::Cpu::reset();
        super::scf(&mut cpu);
        assert!(cpu.registers.f.c);
        cpu.registers.f.c = false;
        super::scf(&mut cpu);
        assert!(cpu.registers.f.c);
    }

    mod daa {
        #[test]
        fn no_adjust() {
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.a = 0x99;
            super::super::daa(&mut cpu);
            assert_eq!(cpu.registers.a, 0x99);
            assert!(!cpu.registers.f.c);
        }

        #[test]
        fn adjust_lo() {
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.a = 0x0A;
            super::super::daa(&mut cpu);
            assert_eq!(cpu.registers.a, 0x10);
            assert!(!cpu.registers.f.c);
        }

        #[test]
        fn adjust_hi() {
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.a = 0xA0;
            super::super::daa(&mut cpu);
            assert_eq!(cpu.registers.a, 0x00);
            assert!(cpu.registers.f.c);
        }

        #[test]
        fn adjust_both() {
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.a = 0xAB;
            super::super::daa(&mut cpu);
            assert_eq!(cpu.registers.a, 0x11);
            assert!(cpu.registers.f.c);
        }
    }

    macro_rules! gen_cp_tests {
        ($( $reg:ident ),+) => {
            mod cp {
                $(
                    mod $reg {
                        #[test]
                        fn equality() {
                            let mut cpu = crate::cpu::Cpu::reset();
                            cpu.registers.a = 0xAB;
                            cpu.registers.$reg = cpu.registers.a;
                            super::super::super::cp::$reg(&mut cpu);
                            assert!(cpu.registers.f.z);
                            assert!(cpu.registers.f.n);
                            assert!(!cpu.registers.f.h);
                            assert!(!cpu.registers.f.c);
                        }

                        #[test]
                        fn a_smaller() {
                            let mut cpu = crate::cpu::Cpu::reset();
                            cpu.registers.a = 0xAB;
                            cpu.registers.$reg = 0xAC;
                            super::super::super::cp::$reg(&mut cpu);
                            assert!(!cpu.registers.f.z);
                            assert!(cpu.registers.f.n);
                            assert!(cpu.registers.f.h);
                            assert!(cpu.registers.f.c);
                        }

                        #[test]
                        fn a_larger() {
                            let mut cpu = crate::cpu::Cpu::reset();
                            cpu.registers.a = 0xAB;
                            cpu.registers.$reg = 0xAA;
                            super::super::super::cp::$reg(&mut cpu);
                            assert!(!cpu.registers.f.z);
                            assert!(cpu.registers.f.n);
                            assert!(!cpu.registers.f.h);
                            assert!(!cpu.registers.f.c);
                        }
                    }
                 )+
                     mod a {
                         #[test]
                         fn trivial() {
                            let mut cpu = crate::cpu::Cpu::reset();
                            for i in 0..=0xFF {
                                cpu.registers.a = i;
                                super::super::super::cp::a(&mut cpu);
                                assert!(cpu.registers.f.z);
                                assert!(cpu.registers.f.n);
                                assert!(!cpu.registers.f.h);
                                assert!(!cpu.registers.f.c);
                            }
                         }
                     }

                    mod hl_ind {
                        #[test]
                        fn equality() {
                            let mut cpu = crate::cpu::Cpu::reset();
                            cpu.registers.a = 0xAB;
                            cpu.registers.put_hl(0xBEEF);
                            cpu.write_byte(cpu.registers.a, cpu.registers.hl());
                            super::super::super::cp::hl_ind(&mut cpu);
                            assert!(cpu.registers.f.z);
                            assert!(cpu.registers.f.n);
                            assert!(!cpu.registers.f.h);
                            assert!(!cpu.registers.f.c);
                        }

                        #[test]
                        fn a_smaller() {
                            let mut cpu = crate::cpu::Cpu::reset();
                            cpu.registers.a = 0xAB;
                            cpu.registers.put_hl(0xBEEF);
                            cpu.write_byte(0xAC, cpu.registers.hl());
                            super::super::super::cp::hl_ind(&mut cpu);
                            assert!(!cpu.registers.f.z);
                            assert!(cpu.registers.f.n);
                            assert!(cpu.registers.f.h);
                            assert!(cpu.registers.f.c);
                        }

                        #[test]
                        fn a_larger() {
                            let mut cpu = crate::cpu::Cpu::reset();
                            cpu.registers.a = 0xAB;
                            cpu.registers.put_hl(0xBEEF);
                            cpu.write_byte(0xAA, cpu.registers.hl());
                            super::super::super::cp::hl_ind(&mut cpu);
                            assert!(!cpu.registers.f.z);
                            assert!(cpu.registers.f.n);
                            assert!(!cpu.registers.f.h);
                            assert!(!cpu.registers.f.c);
                        }
                    }

                    mod imm {
                        #[test]
                        fn equality() {
                            let mut cpu = crate::cpu::Cpu::reset();
                            cpu.registers.a = 0xAB;
                            cpu.write_byte(cpu.registers.a, cpu.registers.pc + 1);
                            super::super::super::cp::imm(&mut cpu);
                            assert!(cpu.registers.f.z);
                            assert!(cpu.registers.f.n);
                            assert!(!cpu.registers.f.h);
                            assert!(!cpu.registers.f.c);
                        }

                        #[test]
                        fn a_smaller() {
                            let mut cpu = crate::cpu::Cpu::reset();
                            cpu.registers.a = 0xAB;
                            cpu.write_byte(0xAC, cpu.registers.pc + 1);
                            super::super::super::cp::imm(&mut cpu);
                            assert!(!cpu.registers.f.z);
                            assert!(cpu.registers.f.n);
                            assert!(cpu.registers.f.h);
                            assert!(cpu.registers.f.c);
                        }

                        #[test]
                        fn a_larger() {
                            let mut cpu = crate::cpu::Cpu::reset();
                            cpu.registers.a = 0xAB;
                            cpu.write_byte(0xAA, cpu.registers.pc + 1);
                            super::super::super::cp::imm(&mut cpu);
                            assert!(!cpu.registers.f.z);
                            assert!(cpu.registers.f.n);
                            assert!(!cpu.registers.f.h);
                            assert!(!cpu.registers.f.c);
                        }
                    }
            }
        };
    }

    gen_cp_tests!(b, c, d, e, h, l);
}
