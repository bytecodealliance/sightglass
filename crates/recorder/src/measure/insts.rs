//! Measure instructions retired.

use super::Measure;
use crate::measure::Measurements;
use sightglass_data::Phase;

/// On Linux we can use `perf_event` to access the hardware performance
/// counters.
#[cfg(target_os = "linux")]
mod linux {
    use perf_event::{Builder, events::Hardware};

    pub use perf_event::Counter as State;

    pub fn new() -> State {
        Builder::new().kind(Hardware::INSTRUCTIONS).build().expect(
            "Unable to create INSTRUCTIONS hardware counter. Does this system actually \
             have such a counter? If it does, your kernel may not fully support this \
             processor.",
        )
    }

    pub fn start(counter: &mut State) {
        counter.reset().unwrap();
        counter.enable().unwrap();
    }

    pub fn end(counter: &mut State) -> u64 {
        counter.disable().unwrap();
        counter.read().unwrap()
    }
}
#[cfg(target_os = "linux")]
use linux as imp;

/// On macOS, user space cannot read the PMU instruction counter itself, so we
/// instead ask the kernel for this process's retired-instruction count via
/// `proc_pid_rusage`. The kernel maintains that count from the hardware
/// performance monitor and exposes it without requiring sudo or enabling
/// Apple's developer tools (which some corporate laptops disallow... on that
/// corporation's developers' machines... yeah.)
#[cfg(target_os = "macos")]
mod macos {
    pub use u64 as State;

    pub fn new() -> State {
        0
    }

    pub fn start(count: &mut State) {
        *count = read_instructions_retired();
    }

    pub fn end(initial_count: &mut State) -> u64 {
        read_instructions_retired().wrapping_sub(*initial_count)
    }

    /// Read the number of instructions this process has retired so far.
    fn read_instructions_retired() -> u64 {
        // SAFETY: `rusage_info_v4` is a plain-old-data struct for which an
        // all-zero bit pattern is valid, and we hand `proc_pid_rusage` a
        // pointer to storage large enough for the `RUSAGE_INFO_V4` flavor we
        // request.
        let mut info: libc::rusage_info_v4 = unsafe { std::mem::zeroed() };
        let ret = unsafe {
            libc::proc_pid_rusage(
                libc::getpid(),
                libc::RUSAGE_INFO_V4,
                &mut info as *mut libc::rusage_info_v4 as *mut libc::rusage_info_t,
            )
        };
        assert_eq!(
            ret, 0,
            "proc_pid_rusage failed to read the retired-instruction count"
        );
        info.ri_instructions
    }
}
#[cfg(target_os = "macos")]
use macos as imp;

/// Measure the number of instructions retired.
pub struct InstsRetiredMeasure(imp::State);

impl Default for InstsRetiredMeasure {
    fn default() -> Self {
        Self::new()
    }
}

impl InstsRetiredMeasure {
    pub fn new() -> Self {
        InstsRetiredMeasure(imp::new())
    }
}

impl Measure for InstsRetiredMeasure {
    fn start(&mut self, _phase: Phase) {
        imp::start(&mut self.0);
    }

    fn end(&mut self, phase: Phase, measurements: &mut Measurements) {
        let count = imp::end(&mut self.0);
        measurements.add(phase, "instructions-retired".into(), count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn sanity() {
        if env::var("CI").is_ok() {
            // If we find ourselves in a CI environment, skip this test: not all CI environments
            // have perf events available (e.g. https://github.community/t/149164). Since this
            // short-circuiting could be problematic in the future, we attempt to warn the user
            // (hoping they have `--nocapture` set).
            println!(
                "Skipping the perf counters sanity test; it seems that this test is running \
                 in a CI environment"
            );
            return;
        }

        let mut measurements = Measurements::new("arch".into(), "engine".into(), "wasm".into());
        let mut measure = InstsRetiredMeasure::new();
        measure.start(Phase::Compilation);
        eprintln!("test test test...");
        measure.end(Phase::Compilation, &mut measurements);
        let measurements = measurements.finish();
        println!("Measurements: {:?}", measurements);
    }
}
