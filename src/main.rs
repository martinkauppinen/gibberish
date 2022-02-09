mod cpu;
mod memory;

fn main() {
    let mut cpu = cpu::Cpu::reset();

    let unused_codes = [
        0xd3, 0xdb, 0xdd, 0xe3, 0xe4, 0xeb, 0xec, 0xed, 0xf4, 0xfc, 0xfd,
    ];

    for i in 0x00..=0xFF {
        if unused_codes.contains(&i) {
            continue;
        }

        cpu.step_op(i);
    }
}
