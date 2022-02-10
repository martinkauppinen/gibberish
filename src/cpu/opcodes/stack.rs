macro_rules! push {
    ($pair:ident) => {
        pub fn $pair(cpu: &mut crate::cpu::Cpu) {
            let [hi, lo] = cpu.registers.$pair().to_be_bytes();
            cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
            cpu.write_byte(hi, cpu.registers.sp);
            cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
            cpu.write_byte(lo, cpu.registers.sp);
        }
    };
}

macro_rules! pop {
    ($pair:ident, $load_func:ident) => {
        pub fn $pair(cpu: &mut crate::cpu::Cpu) {
            let lo = cpu.read_byte(cpu.registers.sp);
            cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
            let hi = cpu.read_byte(cpu.registers.sp);
            cpu.registers.sp = cpu.registers.sp.wrapping_add(1);

            cpu.registers.$load_func(u16::from_be_bytes([hi, lo]));
        }
    };
}

pub mod push {
    push!(bc);
    push!(de);
    push!(hl);
    push!(af);
}

pub mod pop {
    pop!(bc, put_bc);
    pop!(de, put_de);
    pop!(hl, put_hl);
    pop!(af, put_af);
}

#[cfg(test)]
mod test {
    macro_rules! test_stack {
        ($pair:ident, $load_func:ident) => {
            mod $pair {
                #[test]
                fn push() {
                    let mut cpu = crate::cpu::Cpu::reset();
                    let value: u16 = 0xDEA0; // Wanted 0xDEAD, but low nybble of F is always zero
                    let address = 0xBEEF;
                    let [value_hi, value_lo] = value.to_be_bytes();

                    cpu.registers.$load_func(value);
                    cpu.registers.sp = address;
                    super::super::push::$pair(&mut cpu);

                    assert_eq!(cpu.read_byte(cpu.registers.sp), value_lo);
                    assert_eq!(cpu.read_byte(cpu.registers.sp + 1), value_hi);
                }

                #[test]
                fn pop() {
                    let mut cpu = crate::cpu::Cpu::reset();
                    let value: u16 = 0xDEA0;
                    let address = 0xBEEF;
                    let [value_hi, value_lo] = value.to_be_bytes();

                    cpu.registers.sp = address;
                    cpu.write_word(value, cpu.registers.sp);
                    super::super::pop::$pair(&mut cpu);

                    let [pair_hi, pair_lo] = cpu.registers.$pair().to_be_bytes();
                    assert_eq!(pair_lo, value_lo);
                    assert_eq!(pair_hi, value_hi);
                }
            }
        };
    }

    test_stack!(bc, put_bc);
    test_stack!(de, put_de);
    test_stack!(hl, put_hl);
    test_stack!(af, put_af);
}
