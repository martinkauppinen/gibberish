use crate::cpu::{Cpu, RunningMode};
use crate::memory::map;

pub fn stop(cpu: &mut crate::cpu::Cpu) {
    cpu.mode = RunningMode::Stop;
}

pub fn halt(cpu: &mut crate::cpu::Cpu) {
    cpu.mode = RunningMode::Halt;
}

pub fn ei(cpu: &mut crate::cpu::Cpu) {
    cpu.interrupt_master_enable = true;
    cpu.write_byte(1, map::INT_ENABLE_ADDR);
}

pub fn di(cpu: &mut crate::cpu::Cpu) {
    cpu.interrupt_master_enable = false;
    cpu.write_byte(0, map::INT_ENABLE_ADDR);
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
}
