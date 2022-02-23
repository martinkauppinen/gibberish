use crate::cpu::{Cpu, RunningMode};

pub fn stop(cpu: &mut Cpu) {
    cpu.mode = RunningMode::Stop;
}

pub fn halt(cpu: &mut Cpu) {
    if cpu.interrupt_master_enable {
        cpu.mode = RunningMode::HaltImeSet;
    } else if !cpu.interrupts.interrupts_pending() {
        cpu.mode = RunningMode::HaltImeClear;
    } else {
        cpu.mode = RunningMode::HaltBug;
    }
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
    cpu.inhibit_pc = true;
}

#[cfg(test)]
mod test {
    use crate::cpu::{interrupts::Interrupt, Cpu, RunningMode};

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

    #[test]
    fn halt_normal() {
        let mut cpu = Cpu::reset();
        cpu.registers.a = 0;
        cpu.interrupt_master_enable = true;
        cpu.write_byte(0x76, cpu.registers.pc); // HALT
        cpu.write_byte(0x3C, cpu.registers.pc + 1); // INC A
        cpu.write_byte(0xD9, 0x41); // RETI
        cpu.interrupts.enable_interrupt(Interrupt::Vblank);

        // Go into HALT mode
        cpu.step();
        assert_eq!(cpu.mode, RunningMode::HaltImeSet);

        // Make sure nothing happens
        for _ in 0..100 {
            cpu.step();
            assert_eq!(cpu.registers.pc, 0x101);
            assert_eq!(cpu.registers.a, 0x0);
        }

        cpu.interrupts.request_interrupt(Interrupt::Vblank);
        cpu.step(); // NOP in ISR
        assert_eq!(cpu.registers.pc, 0x41);
        assert_eq!(cpu.mode, RunningMode::Running);

        cpu.step(); // RETI
        assert_eq!(cpu.registers.pc, 0x101);

        cpu.step(); // INC A
        assert_eq!(cpu.registers.a, 0x1);
    }

    #[test]
    fn halt_ime_clear() {
        let mut cpu = Cpu::reset();
        cpu.registers.a = 0;
        cpu.interrupt_master_enable = false;
        cpu.write_byte(0x76, cpu.registers.pc); // HALT
        cpu.write_byte(0x3C, cpu.registers.pc + 1); // INC A

        // Go into HALT mode
        cpu.step();
        assert_eq!(cpu.mode, RunningMode::HaltImeClear);

        // Make sure nothing happens
        for _ in 0..100 {
            cpu.step();
            assert_eq!(cpu.registers.pc, 0x101);
            assert_eq!(cpu.registers.a, 0x0);
            assert_eq!(cpu.mode, RunningMode::HaltImeClear);
        }

        cpu.interrupts.request_interrupt(Interrupt::Vblank);
        assert!(cpu.interrupts.interrupts_requested());
        cpu.step(); // Would be NOP in ISR, is now just INC A
        assert_eq!(cpu.mode, RunningMode::Running);
        assert_eq!(cpu.registers.pc, 0x102);
        assert_eq!(cpu.registers.a, 0x1);
    }

    #[test]
    fn halt_bug() {
        let mut cpu = Cpu::reset();
        cpu.registers.a = 0;

        // Halt bug combination
        cpu.interrupt_master_enable = false;
        cpu.interrupts.request_interrupt(Interrupt::Vblank);
        cpu.interrupts.enable_interrupt(Interrupt::Vblank);

        cpu.write_byte(0x76, cpu.registers.pc); // HALT
        cpu.write_byte(0x3C, cpu.registers.pc + 1); // INC A

        // Execute HALT, go into bug state
        cpu.step();
        assert_eq!(cpu.mode, RunningMode::HaltBug);
        assert_eq!(cpu.registers.pc, 0x101);

        cpu.step(); // INC A
        assert_eq!(cpu.mode, RunningMode::Running);
        assert_eq!(cpu.registers.pc, 0x101); // Did not increment because of bug!
        cpu.step(); // Should be NOP, but is INC A again because of the halt bug!
        assert_eq!(cpu.mode, RunningMode::Running);
        assert_eq!(cpu.registers.pc, 0x102);
        assert_eq!(cpu.registers.a, 0x2);
    }
}
