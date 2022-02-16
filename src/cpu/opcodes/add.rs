macro_rules! add {
    (a; $src:ident) => {
        /// Add value of register to A
        /// Z 0 H C
        pub fn $src(cpu: &mut crate::cpu::Cpu) {
            cpu.registers.f.n = false;
            let a_old = cpu.registers.a;
            cpu.registers.a = cpu.registers.a.wrapping_add(cpu.registers.$src);
            cpu.registers.f.h = super::super::half_carry_add(a_old, cpu.registers.a);
            cpu.registers.f.c = cpu.registers.a < a_old;
            cpu.registers.f.z = cpu.registers.a == 0;
        }
    };

    (hl; $src:ident) => {
        /// Add value of register pair to HL
        /// - 0 H C
        pub fn $src(cpu: &mut crate::cpu::Cpu) {
            cpu.registers.f.n = false;
            let h_old = cpu.registers.h;
            cpu.registers
                .put_hl(cpu.registers.hl().wrapping_add(cpu.registers.$src()));
            cpu.registers.f.h = super::super::half_carry_add(h_old, cpu.registers.h);
            cpu.registers.f.c = cpu.registers.h < h_old;
        }
    };
}

pub mod a {

    add!(a; b);
    add!(a; c);
    add!(a; d);
    add!(a; e);
    add!(a; h);
    add!(a; l);
    add!(a; a);

    pub fn imm(cpu: &mut crate::cpu::Cpu) {
        cpu.registers.f.n = false;
        let a_old = cpu.registers.a;
        cpu.registers.a = cpu.registers.a.wrapping_add(cpu.get_byte_argument());
        cpu.registers.f.h = super::super::half_carry_add(a_old, cpu.registers.a);
        cpu.registers.f.c = cpu.registers.a < a_old;
        cpu.registers.f.z = cpu.registers.a == 0;
    }

    pub fn hl_ind(cpu: &mut crate::cpu::Cpu) {
        cpu.registers.f.n = false;
        let a_old = cpu.registers.a;
        cpu.registers.a = cpu
            .registers
            .a
            .wrapping_add(cpu.read_byte(cpu.registers.hl()));
        cpu.registers.f.h = super::super::half_carry_add(a_old, cpu.registers.a);
        cpu.registers.f.c = cpu.registers.a < a_old;
        cpu.registers.f.z = cpu.registers.a == 0;
    }
}

pub mod hl {
    add!(hl; bc);
    add!(hl; de);
    add!(hl; hl);

    /// Add value of SP to HL
    /// - 0 H C
    pub fn sp(cpu: &mut crate::cpu::Cpu) {
        cpu.registers.f.n = false;
        let h_old = cpu.registers.h;
        cpu.registers
            .put_hl(cpu.registers.hl().wrapping_add(cpu.registers.sp));
        cpu.registers.f.h = super::super::half_carry_add(h_old, cpu.registers.h);
        cpu.registers.f.c = cpu.registers.h < h_old;
    }
}

pub mod sp {
    use crate::cpu::opcodes::sign_extend;

    /// Add sign-extended immediate value to SP
    /// 0 0 H C
    pub fn r8(cpu: &mut crate::cpu::Cpu) {
        let old_sp = cpu.registers.sp;

        let byte = cpu.get_byte_argument();
        cpu.registers.sp = cpu.registers.sp.wrapping_add(sign_extend(byte));

        cpu.registers.f.z = false;
        cpu.registers.f.n = false;
        cpu.registers.f.h =
            super::super::half_carry_add((cpu.registers.sp >> 8) as u8, (old_sp >> 8) as u8);
        cpu.registers.f.c = cpu.registers.sp < old_sp;
    }
}

#[cfg(test)]
mod test {
    mod a {

        macro_rules! test_a {
            ($src:ident) => {
                #[test]
                fn $src() {
                    let mut cpu = crate::cpu::Cpu::reset();
                    cpu.registers.a = 0x0F;
                    cpu.registers.$src = 0x01;
                    super::super::a::$src(&mut cpu);
                    assert_eq!(cpu.registers.a, 0x0F + 0x01);
                    assert!(!cpu.registers.f.z);
                    assert!(!cpu.registers.f.n);
                    assert!(!cpu.registers.f.c);
                    assert!(cpu.registers.f.h);
                }
            };
        }

        test_a!(b);
        test_a!(c);
        test_a!(d);
        test_a!(e);
        test_a!(h);
        test_a!(l);

        #[test]
        fn a() {
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.a = 0x0F;
            super::super::a::a(&mut cpu);
            assert_eq!(cpu.registers.a, 0x0F + 0x0F);
            assert!(!cpu.registers.f.z);
            assert!(!cpu.registers.f.n);
            assert!(!cpu.registers.f.c);
            assert!(cpu.registers.f.h);
        }

        #[test]
        fn imm() {
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.a = 0x0F;
            cpu.current_argument = Some(1u8.into());
            super::super::a::imm(&mut cpu);
            assert_eq!(cpu.registers.a, 0x0F + 0x01);
            assert!(!cpu.registers.f.z);
            assert!(!cpu.registers.f.n);
            assert!(!cpu.registers.f.c);
            assert!(cpu.registers.f.h);
        }
    }
    mod hl {
        macro_rules! test_hl {
            ($src:ident, $load_func:ident) => {
                #[test]
                fn $src() {
                    let mut cpu = crate::cpu::Cpu::reset();
                    cpu.registers.put_hl(0xBAD);
                    cpu.registers.$load_func(0xBEEF);
                    super::super::hl::$src(&mut cpu);
                    assert_eq!(cpu.registers.hl(), 0xBAD + 0xBEEF);
                    assert!(!cpu.registers.f.n);
                    assert!(!cpu.registers.f.c);
                    assert!(cpu.registers.f.h);
                }
            };
        }

        test_hl!(bc, put_bc);
        test_hl!(de, put_de);

        #[test]
        fn hl() {
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.put_hl(0xBAD);
            super::super::hl::hl(&mut cpu);
            assert_eq!(cpu.registers.hl(), 2 * 0xBAD);
            assert!(!cpu.registers.f.n);
            assert!(!cpu.registers.f.c);
            assert!(cpu.registers.f.h);
        }

        #[test]
        fn sp() {
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.put_hl(0xBAD);
            cpu.registers.sp = 0xBEEF;
            super::super::hl::sp(&mut cpu);
            assert_eq!(cpu.registers.hl(), 0xBAD + 0xBEEF);
            assert!(!cpu.registers.f.n);
            assert!(!cpu.registers.f.c);
            assert!(cpu.registers.f.h);
        }
    }

    mod sp {
        #[test]
        fn r8() {
            let mut cpu = crate::cpu::Cpu::reset();
            let value: u8 = 0xFF;
            cpu.registers.sp = 0xBEEF;
            cpu.current_argument = Some(value.into());
            super::super::sp::r8(&mut cpu);
            assert_eq!(cpu.registers.sp, 0xBEEE);
            assert!(!cpu.registers.f.z);
            assert!(!cpu.registers.f.n);
            assert!(cpu.registers.f.c);
            assert!(!cpu.registers.f.h);
        }
    }
}
