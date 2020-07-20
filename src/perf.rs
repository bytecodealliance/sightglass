use crate::bench::Sample;
use perfcnt::{AbstractPerfCounter, PerfCounter};
use precision::{Precision, Timestamp};

pub struct PerfCounterCollection {
    pub precision: Precision,
    cpu_cycles: Option<PerfCounter>,
    instructions_retired: Option<PerfCounter>,
    cache_accesses: Option<PerfCounter>,
    cache_misses: Option<PerfCounter>,
}

impl PerfCounterCollection {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "linux")] {
            pub fn new() -> Result<Self, &'static str> {
                use perfcnt::linux::HardwareEventType;

                let precision = Precision::new(precision::Config::default())?;

                let cpu_cycles = linux::create_hardware_counter(HardwareEventType::CPUCycles);
                let instructions_retired = linux::create_hardware_counter(HardwareEventType::Instructions);
                let cache_accesses = linux::create_hardware_counter(HardwareEventType::CacheReferences);
                let cache_misses = linux::create_hardware_counter(HardwareEventType::CacheMisses);
                Ok(PerfCounterCollection {
                    precision,
                    cpu_cycles,
                    instructions_retired,
                    cache_accesses,
                    cache_misses,
                })
            }
        } else {
            pub fn new() -> Result<Self, &'static str> {
                let precision = Precision::new(precision::Config::default())?;

                // Don't currently support performance counter use on non-linux targets.
                // `perfcnt` doesn't feature-gate `linux` so explicitly provide None's here rather
                // than calling possibly wildly incorrect syscalls on non-linux targets.
                Ok(PerfCounterCollection {
                    precision,
                    cpu_cycles: None,
                    instructions_retired: None,
                    cache_accesses: None,
                    cache_misses: None,
                })
            }
        }
    }

    fn start(&mut self) -> PerfCounterMeasure {
        let time_start = self.precision.now();
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
        PerfCounterMeasure {
            time_start,
            counters: self,
        }
    }

    pub fn sample<F: FnOnce()>(&mut self, f: F) -> Sample {
        let measurement = self.start();
        f();
        measurement.finish()
    }

    pub fn now(&self) -> Timestamp {
        self.precision.now()
    }
}

struct PerfCounterMeasure<'a> {
    time_start: Timestamp,
    counters: &'a mut PerfCounterCollection,
}

impl<'a> PerfCounterMeasure<'a> {
    fn finish(self) -> Sample {
        let time_end = self.counters.precision.now();
        Sample {
            clock_time: time_end - self.time_start,
            cpu_cycles: self
                .counters
                .cpu_cycles
                .as_mut()
                .and_then(|c| c.read().ok())
                .unwrap_or(0),
            instructions_retired: self
                .counters
                .instructions_retired
                .as_mut()
                .and_then(|c| c.read().ok())
                .unwrap_or(0),
            cache_accesses: self
                .counters
                .cache_accesses
                .as_mut()
                .and_then(|c| c.read().ok())
                .unwrap_or(0),
            cache_misses: self
                .counters
                .cache_misses
                .as_mut()
                .and_then(|c| c.read().ok())
                .unwrap_or(0),
        }
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
