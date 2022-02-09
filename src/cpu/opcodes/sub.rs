macro_rules! sub {
    ($( $src:ident ),+) => {
        $(
            /// Subtract value of register from A
            /// Z 1 H C
            pub fn $src(cpu: &mut crate::cpu::Cpu) {
                cpu.registers.f.h = super::half_carry_sub(cpu.registers.a, cpu.registers.$src);
                cpu.registers.f.c = cpu.registers.a < cpu.registers.$src;
                cpu.registers.f.n = true;
                cpu.registers.a = cpu.registers.a.wrapping_sub(cpu.registers.$src);
                cpu.registers.f.z = cpu.registers.a == 0;
            }
         )+
    };
}

sub!(b, c, d, e, h, l, a);

/// Subtract immediate value from A
/// Z 1 H C
pub fn imm(cpu: &mut crate::cpu::Cpu) {
    let byte = cpu.get_byte_argument();
    cpu.registers.f.h = super::half_carry_sub(cpu.registers.a, byte);
    cpu.registers.f.c = cpu.registers.a < byte;
    cpu.registers.f.n = true;
    cpu.registers.a = cpu.registers.a.wrapping_sub(byte);
    cpu.registers.f.z = cpu.registers.a == 0;
}

/// Subtract value in memory pointed to by HL from A
/// Z 1 H C
pub fn hl_ind(cpu: &mut crate::cpu::Cpu) {
    let byte = cpu.read_byte(cpu.registers.hl());
    cpu.registers.f.h = super::half_carry_sub(cpu.registers.a, byte);
    cpu.registers.f.c = cpu.registers.a < byte;
    cpu.registers.f.n = true;
    cpu.registers.a = cpu.registers.a.wrapping_sub(byte);
    cpu.registers.f.z = cpu.registers.a == 0;
}

#[cfg(test)]
mod test {
    macro_rules! gen_tests {
        ($( $reg:ident ),+) => {
            $(
                mod $reg {

                    fn setup(minuend: u8, subtrahend: u8, cpu: &mut crate::cpu::Cpu) {
                        cpu.registers.a = minuend;
                        cpu.registers.$reg = subtrahend;
                        super::super::$reg(cpu);
                    }

                    #[test]
                    // 1 - 1 = 0
                    fn simple_subtraction() {
                        let mut cpu = crate::cpu::Cpu::reset();
                        setup(0x1, 0x1, &mut cpu);
                        assert_eq!(cpu.registers.a, 0);
                        assert!(cpu.registers.f.z);
                        assert!(cpu.registers.f.n);
                        assert!(!cpu.registers.f.h);
                        assert!(!cpu.registers.f.c);
                    }

                    #[test]
                    // 0x13 - 0x04 = 0x0F
                    fn half_carry() {
                        let mut cpu = crate::cpu::Cpu::reset();
                        setup(0x13, 0x04, &mut cpu);
                        assert_eq!(cpu.registers.a, 0x0F);
                        assert!(!cpu.registers.f.z);
                        assert!(cpu.registers.f.n);
                        assert!(cpu.registers.f.h);
                        assert!(!cpu.registers.f.c);
                    }

                    #[test]
                    // 0x00 - 0x01 = 0xFF
                    fn carry() {
                        let mut cpu = crate::cpu::Cpu::reset();
                        setup(0x00, 0x01, &mut cpu);
                        assert_eq!(cpu.registers.a, 0xFF);
                        assert!(!cpu.registers.f.z);
                        assert!(cpu.registers.f.n);
                        assert!(cpu.registers.f.h);
                        assert!(cpu.registers.f.c);
                    }
                }
             )+
        };
    }

    gen_tests!(b, c, d, e, h, l);

    mod imm {

        fn setup(minuend: u8, subtrahend: u8, cpu: &mut crate::cpu::Cpu) {
            cpu.registers.a = minuend;
            cpu.write_byte(subtrahend, cpu.registers.pc + 1);
            super::super::imm(cpu);
        }

        #[test]
        // 1 - 1 = 0
        fn simple_subtraction() {
            let mut cpu = crate::cpu::Cpu::reset();
            setup(0x1, 0x1, &mut cpu);
            assert_eq!(cpu.registers.a, 0);
            assert!(cpu.registers.f.z);
            assert!(cpu.registers.f.n);
            assert!(!cpu.registers.f.h);
            assert!(!cpu.registers.f.c);
        }

        #[test]
        // 0x13 - 0x04 = 0x0F
        fn half_carry() {
            let mut cpu = crate::cpu::Cpu::reset();
            setup(0x13, 0x04, &mut cpu);
            assert_eq!(cpu.registers.a, 0x0F);
            assert!(!cpu.registers.f.z);
            assert!(cpu.registers.f.n);
            assert!(cpu.registers.f.h);
            assert!(!cpu.registers.f.c);
        }

        #[test]
        // 0x00 - 0x01 = 0xFF
        fn carry() {
            let mut cpu = crate::cpu::Cpu::reset();
            setup(0x00, 0x01, &mut cpu);
            assert_eq!(cpu.registers.a, 0xFF);
            assert!(!cpu.registers.f.z);
            assert!(cpu.registers.f.n);
            assert!(cpu.registers.f.h);
            assert!(cpu.registers.f.c);
        }
    }

    mod a {
        #[test]
        fn trivial() {
            let mut cpu = crate::cpu::Cpu::reset();
            for i in 0..=0xFF {
                cpu.registers.a = i;
                super::super::a(&mut cpu);
                assert_eq!(cpu.registers.a, 0);
                assert!(cpu.registers.f.z);
                assert!(cpu.registers.f.n);
                assert!(!cpu.registers.f.h);
                assert!(!cpu.registers.f.c);
            }
        }
    }
}
