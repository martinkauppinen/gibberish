macro_rules! ld {
    ($dst:ident; $( $src:ident ),+) => {
        $(
            /// Load a register with value from another register
            /// - - - -
            #[allow(clippy::self_assignment)]
            pub fn $src(cpu: &mut crate::cpu::Cpu) {
                cpu.registers.$dst = cpu.registers.$src;
            }
        )+

        /// Load a register with immeditate value
        /// - - - -
        pub fn imm(cpu: &mut crate::cpu::Cpu)  {
            cpu.registers.$dst = cpu.get_byte_argument();
        }

        /// Load register with value in memory pointed to by HL
        /// - - - -
        pub fn hl_ind(cpu: &mut crate::cpu::Cpu) {
            cpu.registers.$dst = cpu.read_byte(cpu.registers.hl());
        }
    };
}

/// Generate load instructions for all registers
macro_rules! gen_ld {
    ($( $dst:ident ),+) => {
        $(
            pub mod $dst {
                ld!($dst; b, c, d, e, h, l, a);
            }
         )+
    }
}

gen_ld!(b, c, d, e, h, l, a);

#[cfg(test)]
mod test {
    macro_rules! test_reg {
        ($dst:ident; $( $src:ident ),+) => {
            mod $dst {
                use crate::cpu::Cpu;

                $(
                    #[test]
                    fn $src() {
                        let mut cpu = Cpu::reset();
                        cpu.registers.$src = 0xAB;
                        super::super::$dst::$src(&mut cpu);
                        assert_eq!(cpu.registers.$dst, cpu.registers.$src);
                    }
                 )+

                #[test]
                fn imm() {
                    let value = 0xAB;
                    let mut cpu = Cpu::reset();
                    cpu.write_byte(value, cpu.registers.pc + 1);
                    super::super::$dst::imm(&mut cpu);
                    assert_eq!(cpu.registers.$dst, value);
                }

                #[test]
                fn hl_ind() {
                    let value = 0xAB;
                    let mut cpu = Cpu::reset();
                    cpu.write_byte(value, 0x100);
                    cpu.registers.put_hl(0x100);
                    super::super::$dst::hl_ind(&mut cpu);
                    assert_eq!(cpu.registers.$dst, value);
                }
            }
        };
    }

    test_reg!(b; b, c, d, e, h, l, a);
    test_reg!(c; b, c, d, e, h, l, a);
    test_reg!(d; b, c, d, e, h, l, a);
    test_reg!(e; b, c, d, e, h, l, a);
    test_reg!(h; b, c, d, e, h, l, a);
    test_reg!(l; b, c, d, e, h, l, a);
    test_reg!(a; b, c, d, e, h, l, a);
}
