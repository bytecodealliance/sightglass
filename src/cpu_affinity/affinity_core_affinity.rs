use crate::config::*;
use crate::errors::*;

/// Tune the system for more reliable measurements
pub fn tune(config: &Config) -> Result<(), BenchError> {
    if config.single_core.unwrap_or(DEFAULT_SINGLE_CORE) {
        bind_to_single_core()?
    }
    Ok(())
}

/// Bind the current thread to a single CPU core
fn bind_to_single_core() -> Result<(), BenchError> {
    let core_ids =
        core_affinity::get_core_ids().ok_or(BenchError::InternalError("Empty CPU set"))?;
    let last_core = core_ids
        .last()
        .ok_or(BenchError::InternalError("Zero CPU cores detected"))?;
    core_affinity::set_for_current(*last_core);
    Ok(())
}
