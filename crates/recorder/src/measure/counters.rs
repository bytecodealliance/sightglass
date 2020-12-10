//! A mechanism to measure performance using CPU counters through `perf_event_open`.
//!
//! This code is lightly adapted from iximeow's work in
//! https://github.com/bytecodealliance/sightglass/pull/31.
use super::{Measure, Measurement};
use perfcnt::{AbstractPerfCounter, PerfCounter};
use precision::{Precision, Timestamp};
use serde::{Deserialize, Serialize};

/// Measure CPU counters.
pub struct CounterMeasure {
    precision: Precision,
    start_time: Option<Timestamp>,
    cpu_cycles: Option<PerfCounter>,
    instructions_retired: Option<PerfCounter>,
    cache_accesses: Option<PerfCounter>,
    cache_misses: Option<PerfCounter>,
}

impl CounterMeasure {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "linux")] {
            pub fn new() -> Self {
                use perfcnt::linux::HardwareEventType;
                let precision = Precision::new(precision::Config::default()).expect("to create a precision object");
                Self {
                    precision,
                    start_time: None,
                    cpu_cycles: linux::create_hardware_counter(HardwareEventType::CPUCycles),
                    instructions_retired: linux::create_hardware_counter(HardwareEventType::Instructions),
                    cache_accesses: linux::create_hardware_counter(HardwareEventType::CacheReferences),
                    cache_misses: linux::create_hardware_counter(HardwareEventType::CacheMisses)
                }
            }
        } else {
            pub fn new() -> Self {
                let precision = Precision::new(precision::Config::default()).expect("to create a precision object");
                // This doesn't currently support performance counter use on non-linux targets.
                // `perfcnt` doesn't feature-gate `linux` so explicitly provide None's here rather
                // than calling possibly wildly incorrect syscalls on non-linux targets.
                Self {
                    precision,
                    start_time: None,
                    cpu_cycles: None,
                    instructions_retired: None,
                    cache_accesses: None,
                    cache_misses: None,
                }
            }
        }
    }
}

/// TODO the counter measurements should be grouped to avoid noise from calling ioctl once for each
/// reset/start/stop; see https://github.com/gz/rust-perfcnt/issues/19 and discussion at
/// https://github.com/bytecodealliance/sightglass/pull/31#discussion_r459794132.
impl Measure for CounterMeasure {
    fn start(&mut self) {
        if let Some(cpu_cycle_counter) = self.cpu_cycles.as_mut() {
            cpu_cycle_counter
                .reset()
                .expect("if a counter could be created, it can be reset");
        }
        if let Some(instructions_retired) = self.instructions_retired.as_mut() {
            instructions_retired
                .reset()
                .expect("if a counter could be created, it can be reset");
        }
        if let Some(cache_accesses) = self.cache_accesses.as_mut() {
            cache_accesses
                .reset()
                .expect("if a counter could be created, it can be reset");
        }
        if let Some(cache_misses) = self.cache_misses.as_mut() {
            cache_misses
                .reset()
                .expect("if a counter could be created, it can be reset");
        }

        // Start the wall clock last to avoid measuring effects of `perf_event_open` (?).
        self.start_time = Some(self.precision.now());
    }

    fn end(&mut self) -> Measurement {
        let time_end = self.precision.now();
        Measurement::PerfCounters(PerfCounters {
            clock_time: (time_end - self.start_time.unwrap()).ticks(),
            cpu_cycles: self
                .cpu_cycles
                .as_mut()
                .and_then(|c| c.read().ok())
                .unwrap_or(0),
            instructions_retired: self
                .instructions_retired
                .as_mut()
                .and_then(|c| c.read().ok())
                .unwrap_or(0),
            cache_accesses: self
                .cache_accesses
                .as_mut()
                .and_then(|c| c.read().ok())
                .unwrap_or(0),
            cache_misses: self
                .cache_misses
                .as_mut()
                .and_then(|c| c.read().ok())
                .unwrap_or(0),
        })
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use perfcnt::linux::{HardwareEventType, PerfCounterBuilderLinux};
    use perfcnt::PerfCounter;

    fn hardware_event_name(counter_type: &HardwareEventType) -> &'static str {
        match counter_type {
            HardwareEventType::CPUCycles => "CPUCycles",
            HardwareEventType::Instructions => "Instructions",
            HardwareEventType::CacheReferences => "CacheReferences",
            HardwareEventType::CacheMisses => "CacheMisses",
            HardwareEventType::BranchInstructions => "BranchInstructions",
            HardwareEventType::BranchMisses => "BranchMisses",
            HardwareEventType::BusCycles => "BusCycles",
            HardwareEventType::StalledCyclesFrontend => "StalledCyclesFrontend",
            HardwareEventType::StalledCyclesBackend => "StalledCyclesBackend",
            HardwareEventType::RefCPUCycles => "RefCPUCycles",
        }
    }

    // Errors in initializing performance counters are probably not fatal. For some kinds of
    // errors, we can give a useful hint if someone is looking at stderr. Either way, we can
    // continue on with None where the only data recorded is some form of timer.
    pub fn create_hardware_counter(counter_type: HardwareEventType) -> Option<PerfCounter> {
        // `HardwareEventType` is neither Clone nor Debug. In case we need a name for this counter
        // later, get it ready here.
        let event_name = hardware_event_name(&counter_type);
        let counter = PerfCounterBuilderLinux::from_hardware_event(counter_type)
            .on_all_cpus()
            .for_pid(0) // 0 means "the current PID"
            .enable_read_format_time_enabled()
            .enable_read_format_time_running()
            .enable_read_format_id()
            .finish();
        match counter {
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        // Linux appears to support an undocumented level "3" for
                        // `perf_event_paranoid`, denying the ability to use perf to inspect the
                        // current process.
                        eprintln!("Permission denied for hardware event {}. Try setting /proc/sys/kernel/perf_event_paranoid to 2 or below?", event_name);
                    }
                    std::io::ErrorKind::NotFound => {
                        eprintln!(
                            "Unable to create hardware counter for {}. Does this system actually have such a counter? If it does, your kernel may not fully support this processor.", event_name);
                    }
                    _ => {
                        eprintln!(
                            "Unknown error encountered when creating hardware counter for {}: {:?}",
                            event_name, e
                        );
                    }
                };
                None
            }
            Ok(counter) => Some(counter),
        }
    }
}

/// A recording of time and performance counter information. `PerfCounters::default()` provides a
/// useful zero to accumulate into.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct PerfCounters {
    /// The wall clock time (in CPU cycles) that passed while measuring this sample.
    pub clock_time: u64,
    /// Measured by performance counter. May be 0, in which case the counter is almost certainly
    /// disabled.
    pub cpu_cycles: u64,
    /// Measured by performance counter. May be 0, in which case the counter is almost certainly
    /// disabled.
    pub instructions_retired: u64,
    /// Measured by performance counter. May be 0, in which case the counter is almost certainly
    /// disabled.
    pub cache_accesses: u64,
    /// Measured by performance counter. May be 0, in which case the counter is almost certainly
    /// disabled.
    pub cache_misses: u64,
}

// impl Measurement for CounterMeasurement {}

impl std::ops::Div<u64> for PerfCounters {
    type Output = Self;
    fn div(self, rhs: u64) -> Self::Output {
        PerfCounters {
            clock_time: self.clock_time / rhs,
            cpu_cycles: self.cpu_cycles / rhs,
            instructions_retired: self.instructions_retired / rhs,
            cache_accesses: self.cache_accesses / rhs,
            cache_misses: self.cache_misses / rhs,
        }
    }
}

impl std::ops::Add<PerfCounters> for PerfCounters {
    type Output = Self;
    fn add(self, rhs: PerfCounters) -> Self::Output {
        PerfCounters {
            clock_time: self.clock_time + rhs.clock_time,
            cpu_cycles: self.cpu_cycles + rhs.cpu_cycles,
            instructions_retired: self.instructions_retired + rhs.instructions_retired,
            cache_accesses: self.cache_accesses + rhs.cache_accesses,
            cache_misses: self.cache_misses + rhs.cache_misses,
        }
    }
}

impl std::ops::AddAssign<PerfCounters> for PerfCounters {
    fn add_assign(&mut self, rhs: PerfCounters) {
        self.clock_time += rhs.clock_time;
        self.cpu_cycles += rhs.cpu_cycles;
        self.instructions_retired += rhs.instructions_retired;
        self.cache_accesses += rhs.cache_accesses;
        self.cache_misses += rhs.cache_misses;
    }
}
