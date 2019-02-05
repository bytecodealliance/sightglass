use super::config::*;
use super::errors::*;
use hwloc::{ObjectType, Topology, TopologyObject};

/// Tune the system for more reliable measurements
pub fn tune(config: &Config) -> Result<(), BenchError> {
    if config.single_core.unwrap_or(DEFAULT_SINGLE_CORE) {
        bind_to_single_core()?
    }
    Ok(())
}

/// Bind the current thread to a single CPU core
fn bind_to_single_core() -> Result<(), BenchError> {
    let mut topo = Topology::new();
    let mut cpuset = last_core(&mut topo)?
        .cpuset()
        .ok_or(BenchError::InternalError("Empty CPU set"))?;
    cpuset.singlify();
    Ok(())
}

/// Helper method to find the last core
fn last_core(topo: &mut Topology) -> Result<&TopologyObject, BenchError> {
    let core_depth = topo.depth_or_below_for_type(&ObjectType::Core).unwrap();
    let all_cores = topo.objects_at_depth(core_depth);
    all_cores
        .last()
        .map(|&x| x)
        .ok_or(BenchError::InternalError("No cores found"))
}
