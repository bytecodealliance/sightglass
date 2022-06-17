use anyhow::{anyhow, Result};
use hwloc::{ObjectType, Topology, TopologyObject};

/// Bind the current thread to a single CPU core.
pub fn bind_to_single_core() -> Result<()> {
    let mut topo = Topology::new();
    let mut cpuset = last_core(&mut topo)?
        .cpuset()
        .ok_or(anyhow!("empty CPU set"))?;
    cpuset.singlify();
    Ok(())
}

/// Helper method to find the last core.
fn last_core(topo: &mut Topology) -> Result<&TopologyObject> {
    let core_depth = topo.depth_or_below_for_type(&ObjectType::Core).unwrap();
    let all_cores = topo.objects_at_depth(core_depth);
    all_cores.last().cloned().ok_or(anyhow!("no cores found"))
}
