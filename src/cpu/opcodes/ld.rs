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
