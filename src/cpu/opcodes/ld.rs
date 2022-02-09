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

        /// Load a register with immediate value
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

/// Generate load instructions for registers
macro_rules! gen_ld {
    ($( $dst:ident ),+) => {
        $(
            pub mod $dst {
                ld!($dst; b, c, d, e, h, l, a);
            }
         )+
    }
}

gen_ld!(b, c, d, e, h, l);

pub mod a {
    macro_rules! ld_pair_ind_src {
        ($name:ident, $pair:ident) => {
            /// Load A with value pointed to by $pair
            /// - - - -
            pub fn $name(cpu: &mut crate::cpu::Cpu) {
                cpu.registers.a = cpu.read_byte(cpu.registers.$pair());
            }
        };
    }

    ld!(a; b, c, d, e, h, l, a);
    ld_pair_ind_src!(bc_ind, bc);
    ld_pair_ind_src!(de_ind, de);

    /// Load A with value pointed to by HL and increment HL
    /// - - - -
    pub fn hl_ind_add(cpu: &mut crate::cpu::Cpu) {
        hl_ind(cpu);
        cpu.registers.put_hl(cpu.registers.hl().wrapping_add(1));
    }

    /// Load A with value pointed to by HL and decrement HL
    /// - - - -
    pub fn hl_ind_sub(cpu: &mut crate::cpu::Cpu) {
        hl_ind(cpu);
        cpu.registers.put_hl(cpu.registers.hl().wrapping_sub(1));
    }

    /// Load A with value pointed to by immediate argument
    /// - - - -
    pub fn addr(cpu: &mut crate::cpu::Cpu) {
        let addr = cpu.get_word_argument();
        cpu.registers.a = cpu.read_byte(addr);
    }

    /// Load A with value pointed to by C
    /// - - - -
    pub fn c_ind(cpu: &mut crate::cpu::Cpu) {
        cpu.registers.a = cpu.read_byte(0xFF00 + cpu.registers.c as u16);
    }
}

pub mod c_ind {

    /// Load memory pointed to by C with value of A
    /// - - - -
    pub fn a(cpu: &mut crate::cpu::Cpu) {
        let addr = 0xFF00 + cpu.registers.c as u16;
        cpu.write_byte(cpu.registers.a, addr);
    }
}

macro_rules! ld_pair_ind {
    ($pair:ident, $src:ident) => {
        /// Load value of register into memory pointed to by register pair
        /// - - - -
        pub fn $src(cpu: &mut crate::cpu::Cpu) {
            cpu.write_byte(cpu.registers.$src, cpu.registers.$pair());
        }
    };

    ($pair:ident; imm) => {
        /// Load immediate value into memory pointed to by register pair
        /// - - - -
        pub fn imm(cpu: &mut crate::cpu::Cpu) {
            let byte = cpu.get_byte_argument();
            cpu.write_byte(byte, cpu.registers.$pair());
        }
    };
}

pub mod hl_ind {
    ld_pair_ind!(hl, b);
    ld_pair_ind!(hl, c);
    ld_pair_ind!(hl, d);
    ld_pair_ind!(hl, e);
    ld_pair_ind!(hl, h);
    ld_pair_ind!(hl, l);
    ld_pair_ind!(hl, a);
    ld_pair_ind!(hl; imm);

    pub mod add {
        /// Load A into memory pointed to by HL and increment HL
        /// - - - -
        pub fn a(cpu: &mut crate::cpu::Cpu) {
            super::a(cpu);
            cpu.registers.put_hl(cpu.registers.hl().wrapping_add(1));
        }
    }

    pub mod sub {
        /// Load A into memory pointed to by HL and decrement HL
        /// - - - -
        pub fn a(cpu: &mut crate::cpu::Cpu) {
            super::a(cpu);
            cpu.registers.put_hl(cpu.registers.hl().wrapping_sub(1));
        }
    }
}

pub mod bc_ind {
    ld_pair_ind!(bc, a);
}

pub mod de_ind {
    ld_pair_ind!(de, a);
}

macro_rules! ld_pair_imm {
    ($load_func:ident) => {
        pub fn imm(cpu: &mut crate::cpu::Cpu) {
            let word = cpu.get_word_argument();
            cpu.registers.$load_func(word);
        }
    };
}

pub mod bc {
    ld_pair_imm!(put_bc);
}

pub mod de {
    ld_pair_imm!(put_de);
}

pub mod hl {
    use crate::cpu::opcodes::sign_extend;

    ld_pair_imm!(put_hl);

    pub fn sp_add_reg(cpu: &mut crate::cpu::Cpu) {
        let byte = cpu.get_byte_argument();

        cpu.registers
            .put_hl(cpu.registers.sp.wrapping_add(sign_extend(byte)));
        cpu.registers.f.z = false;
        cpu.registers.f.n = false;
        cpu.registers.f.h = super::super::half_carry_add(
            (cpu.registers.sp >> 8) as u8,
            (cpu.registers.hl() >> 8) as u8,
        );
        cpu.registers.f.c = cpu.registers.hl() < cpu.registers.sp;
    }
}

pub mod addr {
    pub fn a(cpu: &mut crate::cpu::Cpu) {
        let addr = cpu.get_word_argument();
        cpu.write_byte(cpu.registers.a, addr);
    }

    pub fn sp(cpu: &mut crate::cpu::Cpu) {
        let addr = cpu.get_word_argument();
        cpu.write_word(cpu.registers.sp, addr);
    }
}

pub mod sp {
    pub fn imm(cpu: &mut crate::cpu::Cpu) {
        let word = cpu.get_word_argument();
        cpu.registers.sp = word
    }

    pub fn hl(cpu: &mut crate::cpu::Cpu) {
        cpu.registers.sp = cpu.registers.hl();
    }
}

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

    macro_rules! test_pair_ind {
        ($mod_name:ident, $load_func:ident, $pair:ident, $src:ident) => {
            /// Test loading register value into memory pointed to by $pair
            #[test]
            fn $src() {
                let value = 0xAB;
                let mut cpu = crate::cpu::Cpu::reset();
                cpu.registers.$src = value;
                cpu.registers.$load_func(0x100);
                super::super::$mod_name::$src(&mut cpu);
                assert_eq!(cpu.read_byte(cpu.registers.$pair()), value);
            }
        };

        ($mod_name:ident, $pair:ident, $src:ident) => {
            /// Specialized version for h, l
            /// Only difference is that $load_func is not used, as it
            /// interferes when H or L are the source registers
            #[test]
            fn $src() {
                let value = 0xAB;
                let mut cpu = crate::cpu::Cpu::reset();
                cpu.registers.$src = value;
                super::super::$mod_name::$src(&mut cpu);
                assert_eq!(cpu.read_byte(cpu.registers.$pair()), value);
            }
        };

        ($mod_name:ident, $load_func:ident, $pair:ident; imm) => {
            /// Test loading immediate value into memory pointed to by $pair
            #[test]
            fn imm() {
                let value = 0xAB;
                let mut cpu = crate::cpu::Cpu::reset();
                cpu.write_byte(value, cpu.registers.pc + 1);
                cpu.registers.$load_func(0x100);
                super::super::$mod_name::imm(&mut cpu);
                assert_eq!(cpu.read_byte(cpu.registers.$pair()), value);
            }
        };
    }

    pub mod hl_ind {
        test_pair_ind!(hl_ind, put_hl, hl, b);
        test_pair_ind!(hl_ind, put_hl, hl, c);
        test_pair_ind!(hl_ind, put_hl, hl, d);
        test_pair_ind!(hl_ind, put_hl, hl, e);
        test_pair_ind!(hl_ind, hl, h);
        test_pair_ind!(hl_ind, hl, l);
        test_pair_ind!(hl_ind, put_hl, hl, a);
        test_pair_ind!(hl_ind, put_hl, hl; imm);

        /// Test loading A and incrementing
        #[test]
        fn add_a() {
            let value = 0xAB;
            let addr = 0x100;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.a = value;
            cpu.registers.put_hl(addr);
            super::super::hl_ind::add::a(&mut cpu);
            assert_eq!(cpu.read_byte(cpu.registers.hl() - 1), value);
            assert_eq!(cpu.registers.hl(), 0x101);
        }

        /// Test loading A and decrementing
        #[test]
        fn sub_a() {
            let value = 0xAB;
            let addr = 0x100;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.a = value;
            cpu.registers.put_hl(addr);
            super::super::hl_ind::sub::a(&mut cpu);
            assert_eq!(cpu.read_byte(cpu.registers.hl() + 1), value);
            assert_eq!(cpu.registers.hl(), 0xFF);
        }
    }

    pub mod bc_ind {
        test_pair_ind!(bc_ind, put_bc, bc, a);
    }

    pub mod de_ind {
        test_pair_ind!(de_ind, put_de, de, a);
    }

    pub mod a_special {

        #[test]
        fn c_ind() {
            let value = 0xAB;
            let addr = 0xBA;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.c = addr;
            cpu.write_byte(value, 0xFF00 + cpu.registers.c as u16);
            super::super::a::c_ind(&mut cpu);
            assert_eq!(cpu.registers.a, value);
        }

        #[test]
        fn bc_ind() {
            let value = 0xAB;
            let addr = 0x100;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.put_bc(addr);
            cpu.write_byte(value, cpu.registers.bc());
            super::super::a::bc_ind(&mut cpu);
            assert_eq!(cpu.registers.a, value);
        }

        #[test]
        fn de_ind() {
            let value = 0xAB;
            let addr = 0x100;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.put_de(addr);
            cpu.write_byte(value, cpu.registers.de());
            super::super::a::de_ind(&mut cpu);
            assert_eq!(cpu.registers.a, value);
        }

        #[test]
        fn hl_ind_add() {
            let value = 0xAB;
            let addr = 0x100;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.put_hl(addr);
            cpu.write_byte(value, cpu.registers.hl());
            super::super::a::hl_ind_add(&mut cpu);
            assert_eq!(cpu.registers.a, value);
            assert_eq!(cpu.registers.hl(), addr + 1);
        }

        #[test]
        fn hl_ind_sub() {
            let value = 0xAB;
            let addr = 0x100;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.put_hl(addr);
            cpu.write_byte(value, cpu.registers.hl());
            super::super::a::hl_ind_sub(&mut cpu);
            assert_eq!(cpu.registers.a, value);
            assert_eq!(cpu.registers.hl(), addr - 1);
        }

        #[test]
        fn addr() {
            let value = 0xAB;
            let addr = 0x123;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.write_word(addr, cpu.registers.pc + 1);
            cpu.write_byte(value, addr);
            super::super::a::addr(&mut cpu);
            assert_eq!(cpu.registers.a, value);
        }
    }

    macro_rules! test_pair_imm {
        ($pair:ident) => {
            #[test]
            fn imm() {
                let value = 0xBEEF;
                let mut cpu = crate::cpu::Cpu::reset();
                cpu.write_word(value, cpu.registers.pc + 1);
                super::super::$pair::imm(&mut cpu);
                assert_eq!(cpu.registers.$pair(), value);
            }
        };
    }

    pub mod bc {
        test_pair_imm!(bc);
    }

    pub mod de {
        test_pair_imm!(bc);
    }

    pub mod hl {
        test_pair_imm!(bc);

        #[test]
        fn sp_add_reg() {
            let value = 0xFF;
            let addr = 0xBEEF;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.sp = addr;
            cpu.write_byte(value, cpu.registers.pc + 1);
            super::super::hl::sp_add_reg(&mut cpu);
            assert_eq!(cpu.registers.hl(), 0xBEEE);
            assert!(!cpu.registers.f.z);
            assert!(!cpu.registers.f.n);
            assert!(!cpu.registers.f.h);
            assert!(cpu.registers.f.c);
        }
    }

    pub mod addr {

        #[test]
        fn a() {
            let value = 0xAB;
            let addr = 0x123;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.a = value;
            cpu.write_word(addr, cpu.registers.pc + 1);
            super::super::addr::a(&mut cpu);
            assert_eq!(cpu.read_byte(addr), value);
        }

        #[test]
        fn sp() {
            let value = 0xAB;
            let addr = 0x123;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.sp = value;
            cpu.write_word(addr, cpu.registers.pc + 1);
            super::super::addr::sp(&mut cpu);
            assert_eq!(cpu.read_word(addr), value);
        }
    }

    pub mod sp {
        #[test]
        fn hl() {
            let value = 0xAB;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.put_hl(value);
            super::super::sp::hl(&mut cpu);
            assert_eq!(cpu.registers.sp, cpu.registers.hl());
        }
    }

    pub mod c_ind {
        #[test]
        fn a() {
            let value = 0xAB;
            let addr = 0xBA;
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.c = addr;
            cpu.registers.a = value;
            super::super::c_ind::a(&mut cpu);
            assert_eq!(cpu.read_byte(0xFF00 + cpu.registers.c as u16), value);
        }
    }
}
