//! Measure the number of CPU cycles elapsed.

use super::{Measure, Measurements};
use sightglass_data::Phase;

/// Read a free-running counter from user space (e.g. `RDTSC`).
///
/// These generally tick at a fixed rate rather than at the core's clock speed,
/// so they are cheap and precise but measure elapsed time, not work done.
#[cfg(not(target_os = "macos"))]
mod imp {
    use core::arch::asm;

    pub type State = Option<u64>;

    #[inline]
    fn cpucounter() -> u64 {
        cfg_select! {
            target_arch = "x86_64" => {
                let (low, high): (u64, u64);
                unsafe { asm!("rdtscp", out("eax") low, out("edx") high, out("ecx") _); }
                (high << 32) | low
            }
            target_arch = "x86" => {
                let (low, high): (u32, u32);
                unsafe { asm!("rdtscp", out("eax") low, out("edx") high, out("ecx") _); }
                ((high as u64) << 32) | (low as u64)
            }
            target_arch = "aarch64" => {
                let vtm: u64;
                unsafe { asm!("mrs {}, cntvct_el0", out(reg) vtm); }
                vtm
            }
            target_arch = "riscv64" => {
                let time: u64;
                unsafe { asm!("rdtime {}", out(reg) time); }
                time
            }
            target_arch = "s390x" => {
                let time: u64;
                unsafe { asm!("stck ({})", out(reg) &mut time); }
                time
            }
        }
    }

    pub fn new() -> State {
        None
    }

    pub fn start(state: &mut State) {
        *state = Some(cpucounter());
    }

    pub fn end(state: &mut State) -> u64 {
        let end = cpucounter();
        let start = state.take().expect("must call start before end");
        end.wrapping_sub(start)
    }
}

/// Ask the kernel for this process's cycle count from the hardware performance
/// monitor.
///
/// On aarch64, `cntvct_el0`, is a 24 MHz system timer on Apple silicon, so it
/// reports wall time rather than cycles.
///
/// The kernel's count is process-wide, so it includes parallel compilation
/// threads, and each read costs a syscall (~2 microseconds), which coarsens
/// very short phases.
#[cfg(target_os = "macos")]
mod imp {
    use super::super::rusage;

    pub type State = u64;

    pub fn new() -> State {
        0
    }

    pub fn start(state: &mut State) {
        *state = rusage::read().ri_cycles;
    }

    pub fn end(state: &mut State) -> u64 {
        rusage::read().ri_cycles.wrapping_sub(*state)
    }
}

pub struct CycleMeasure(imp::State);

impl Default for CycleMeasure {
    fn default() -> Self {
        Self::new()
    }
}

impl CycleMeasure {
    pub fn new() -> Self {
        Self(imp::new())
    }
}

impl Measure for CycleMeasure {
    fn start(&mut self, _phase: Phase) {
        imp::start(&mut self.0);
    }

    fn end(&mut self, phase: Phase, measurements: &mut Measurements) {
        let elapsed = imp::end(&mut self.0);
        measurements.add(phase, "cycles".into(), elapsed);
    }
}
