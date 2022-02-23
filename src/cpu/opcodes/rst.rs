fn rst(cpu: &mut crate::cpu::Cpu, offset: u16) {
    cpu.push(cpu.registers.pc);
    cpu.registers.pc = offset;
    cpu.inhibit_pc = true;
}

macro_rules! gen_rst {
    ($name:ident, $offset:literal) => {
        /// Push PC onto stack and jump to address $offset in zero page
        /// - - - -
        pub fn $name(cpu: &mut crate::cpu::Cpu) {
            rst(cpu, $offset);
        }
    };
}

gen_rst!(rst_0, 0x0);
gen_rst!(rst_1, 0x8);
gen_rst!(rst_2, 0x10);
gen_rst!(rst_3, 0x18);
gen_rst!(rst_4, 0x20);
gen_rst!(rst_5, 0x28);
gen_rst!(rst_6, 0x30);
gen_rst!(rst_7, 0x38);

#[cfg(test)]
mod tests {
    macro_rules! gen_test {
        ($name:ident, $offset:literal) => {
            #[test]
            fn $name() {
                let mut cpu = crate::cpu::Cpu::reset();
                super::$name(&mut cpu);
                assert_eq!(cpu.registers.pc, $offset);
                assert_eq!(cpu.read_word(cpu.registers.sp), 0x100);
            }
        };
    }

    gen_test!(rst_0, 0x0);
    gen_test!(rst_1, 0x8);
    gen_test!(rst_2, 0x10);
    gen_test!(rst_3, 0x18);
    gen_test!(rst_4, 0x20);
    gen_test!(rst_5, 0x28);
    gen_test!(rst_6, 0x30);
    gen_test!(rst_7, 0x38);
}
