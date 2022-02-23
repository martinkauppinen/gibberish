mod cpu;
mod memory;

fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("No ROM file provided");
        return;
    }

    let _ = args.next();

    let mut cpu = cpu::Cpu::reset();
    let rom = std::fs::read(args.next().unwrap()).unwrap();

    for (i, b) in rom.iter().enumerate() {
        cpu.write_byte(*b, i.try_into().unwrap());
    }

    loop {
        cpu.print_status();
        cpu.step();
    }
}
