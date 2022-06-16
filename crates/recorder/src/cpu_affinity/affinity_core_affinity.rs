use anyhow::{anyhow, Result};

/// Bind the current thread to a single CPU core.
pub fn bind_to_single_core() -> Result<()> {
    let core_ids = core_affinity::get_core_ids().ok_or(anyhow!("empty CPU set"))?;
    let last_core = core_ids.last().ok_or(anyhow!("zero CPU cores detected"))?;
    core_affinity::set_for_current(*last_core);
    Ok(())
}
