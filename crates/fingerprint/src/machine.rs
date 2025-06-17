use crate::hash;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::env;
use sysinfo::{CpuExt, System, SystemExt};

/// Describes a fingerprinted system.
///
/// ```
/// # use sightglass_fingerprint::Machine;
/// println!("Current machine fingerprint: {:?}", Machine::fingerprint());
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Machine {
    /// A unique identifier hashed from all other properties.
    pub id: String,
    /// The host name of the machine.
    pub name: String,
    /// The long version of the system OS; e.g., Linux 35 Fedora Linux.
    pub os: String,
    /// The system's kernel version; e.g., 5.15.14
    pub kernel: String,
    /// The system's CPU architecture.
    pub arch: String,
    /// The CPU brand string; e.g., like `lscpu`'s model name.
    pub cpu: String,
    /// The amount of system memory in a human-readable string; e.g. 4 GiB.
    pub memory: String,
}

impl Machine {
    /// Detect system information for the currently machine running Sightglass.
    pub fn fingerprint() -> Result<Self> {
        // Gather the host name.
        let name = hostname::get()
            .expect("must be able to detect the system hostname")
            .to_string_lossy()
            .to_string();

        // Gather the OS information.
        let mut sys = System::new();
        let os = sys
            .long_os_version()
            .ok_or(anyhow!("must be able to detect the system OS"))?;
        let kernel = sys
            .kernel_version()
            .ok_or(anyhow!("must be able to detect the system kernel version"))?;

        // Gather some CPU information.
        let arch = std::env::consts::ARCH.to_string();
        sys.refresh_cpu();
        let cpu = sys.global_cpu_info().brand().to_string();

        // Gather the memory information. The expected result should be in GiB (the 1024-base SI
        // measurement commonly used for memory) but it is unclear whether `sysinfo` is returning KB
        // or KiB.
        sys.refresh_memory();
        let memory_total_kb = sys.total_memory() / 1024;
        let memory = bytesize::ByteSize::kib(memory_total_kb)
            .display()
            .iec()
            .to_string();

        // Hash all properties into a unique identifier.
        let hash = hash::string(&format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            name, arch, os, kernel, cpu, memory
        ));
        let id = format!(
            "{}-{}-{}",
            env::consts::ARCH,
            env::consts::OS,
            hash::slug(&hash)
        );

        Ok(Self {
            id,
            name,
            arch,
            os,
            kernel,
            cpu,
            memory,
        })
    }
}
