use crate::cpu::{Cpu, RunningMode};

pub fn stop(cpu: &mut Cpu) {
    cpu.mode = RunningMode::Stop;
}

pub fn halt(cpu: &mut Cpu) {
    cpu.mode = RunningMode::Halt;
}

pub fn ei(cpu: &mut Cpu) {
    cpu.interrupt_master_enable = true;
}

pub fn di(cpu: &mut Cpu) {
    cpu.interrupt_master_enable = false;
}

pub fn reti(cpu: &mut Cpu) {
    cpu.interrupt_master_enable = true;
    cpu.registers.pc = cpu.pop();
    cpu.branch_taken = true;
}

#[cfg(test)]
mod test {
    use crate::cpu::{interrupts::Interrupt, Cpu};

    #[test]
    fn ei() {
        let mut cpu = Cpu::reset();
        cpu.registers.a = 0;
        cpu.write_byte(0x3C, cpu.registers.pc); // INC A
        cpu.write_byte(0x3C, cpu.registers.pc + 1);
        cpu.interrupts.request_interrupt(Interrupt::Vblank);
        cpu.interrupts.enable_interrupt(Interrupt::Vblank);
        cpu.step();
        assert_eq!(cpu.registers.a, 1);
        assert_ne!(cpu.registers.pc, 0x41);
        super::ei(&mut cpu);
        cpu.step();

        // A has not been incremented, and instead the first instruction
        // of the interrupt service handler has been executed (NOP)
        assert_ne!(cpu.registers.a, 2);
        assert_eq!(cpu.registers.pc, 0x41);
    }

    #[test]
    fn di() {
        let mut cpu = Cpu::reset();
        cpu.registers.a = 0;
        cpu.write_byte(0x3C, cpu.registers.pc); // INC A
        cpu.interrupts.request_interrupt(Interrupt::Vblank);
        cpu.interrupts.enable_interrupt(Interrupt::Vblank);
        super::ei(&mut cpu);
        cpu.step();
        assert_ne!(cpu.registers.a, 1);
        assert_eq!(cpu.registers.pc, 0x41);
        cpu.registers.pc = 0x100;
        cpu.write_byte(0x3C, cpu.registers.pc); // INC A
        super::di(&mut cpu);
        cpu.step();
        assert_eq!(cpu.registers.a, 1);
        assert_ne!(cpu.registers.pc, 0x41);
    }

    #[test]
    fn reti() {
        let mut cpu = Cpu::reset();
        cpu.registers.a = 0;
        cpu.write_byte(0x3C, cpu.registers.pc); // INC A
        cpu.write_byte(0x3C, cpu.registers.pc + 1);

        cpu.write_byte(0xD9, 0x41); // RETI
        cpu.interrupts.request_interrupt(Interrupt::Vblank);
        cpu.interrupts.enable_interrupt(Interrupt::Vblank);
        super::ei(&mut cpu);

        cpu.step(); // NOP in ISR
        assert_ne!(cpu.registers.a, 1);
        assert_eq!(cpu.registers.pc, 0x41);
        cpu.step(); // RETI
        assert_eq!(cpu.registers.pc, 0x100);
    }
}
