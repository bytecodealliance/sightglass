use crate::path::get_cache_dir;
use anyhow::{Context, Result};
use std::fs;

/// Clean up and remove any cached data.
pub fn clean() -> Result<()> {
    let sightglass_data_dir = get_cache_dir()?;
    if !sightglass_data_dir.is_dir() {
        return Ok(());
    }

    log::info!(
        "Recursively removing sightglass cached data in {}",
        sightglass_data_dir.display()
    );
    fs::remove_dir_all(&sightglass_data_dir).with_context(|| {
        format!(
            "failed to recursively remove {}",
            sightglass_data_dir.display(),
        )
    })?;

    Ok(())
}
