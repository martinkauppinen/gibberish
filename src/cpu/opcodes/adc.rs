/// This file is essentially a copy of add.rs, with added lines for adding the
/// carry bit where appropriate. Should probably be merged with add.rs somehow

macro_rules! adc {
    (a; $src:ident) => {
        /// Add value of register to A
        /// Z 0 H C
        pub fn $src(cpu: &mut crate::cpu::Cpu) {
            cpu.registers.f.n = false;
            let a_old = cpu.registers.a;
            cpu.registers.a = cpu.registers.a.wrapping_add(cpu.registers.$src);

            if cpu.registers.f.c {
                cpu.registers.a = cpu.registers.a.wrapping_add(1);
            }

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

            if cpu.registers.f.c {
                cpu.registers.put_hl(cpu.registers.hl().wrapping_add(1));
            }

            cpu.registers.f.h = super::super::half_carry_add(h_old, cpu.registers.h);
            cpu.registers.f.c = cpu.registers.h < h_old;
        }
    };
}

pub mod a {
    adc!(a; b);
    adc!(a; c);
    adc!(a; d);
    adc!(a; e);
    adc!(a; h);
    adc!(a; l);
    adc!(a; a);

    pub fn imm(cpu: &mut crate::cpu::Cpu) {
        cpu.registers.f.n = false;
        let a_old = cpu.registers.a;
        cpu.registers.a = cpu.registers.a.wrapping_add(cpu.get_byte_argument());

        if cpu.registers.f.c {
            cpu.registers.a = cpu.registers.a.wrapping_add(1);
        }

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

        if cpu.registers.f.c {
            cpu.registers.a = cpu.registers.a.wrapping_add(1);
        }

        cpu.registers.f.h = super::super::half_carry_add(a_old, cpu.registers.a);
        cpu.registers.f.c = cpu.registers.a < a_old;
        cpu.registers.f.z = cpu.registers.a == 0;
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
                    cpu.registers.f.c = true;
                    cpu.registers.$src = 0x01;
                    super::super::a::$src(&mut cpu);
                    assert_eq!(cpu.registers.a, 0x0F + 0x01 + 1);
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
            cpu.registers.f.c = true;
            super::super::a::a(&mut cpu);
            assert_eq!(cpu.registers.a, 0x0F + 0x0F + 1);
            assert!(!cpu.registers.f.z);
            assert!(!cpu.registers.f.n);
            assert!(!cpu.registers.f.c);
            assert!(cpu.registers.f.h);
        }

        #[test]
        fn imm() {
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.a = 0x0F;
            cpu.registers.f.c = true;
            cpu.current_argument = Some(1u8.into());
            super::super::a::imm(&mut cpu);
            assert_eq!(cpu.registers.a, 0x0F + 0x01 + 1);
            assert!(!cpu.registers.f.z);
            assert!(!cpu.registers.f.n);
            assert!(!cpu.registers.f.c);
            assert!(cpu.registers.f.h);
        }

        #[test]
        fn hl_ind() {
            let mut cpu = crate::cpu::Cpu::reset();
            cpu.registers.a = 0x0F;
            cpu.registers.f.c = true;
            cpu.registers.put_hl(0xBEEF);
            cpu.write_byte(0x01, cpu.registers.hl());
            super::super::a::hl_ind(&mut cpu);
            assert_eq!(cpu.registers.a, 0x0F + 0x01 + 1);
            assert!(!cpu.registers.f.z);
            assert!(!cpu.registers.f.n);
            assert!(!cpu.registers.f.c);
            assert!(cpu.registers.f.h);
        }
    }
}
