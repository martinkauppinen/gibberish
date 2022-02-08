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
