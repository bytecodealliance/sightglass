use anyhow::{Context, Result};
use regex::Regex;
use structopt::StructOpt;

/// Remove cached artifacts and log files.
#[derive(StructOpt, Debug)]
#[structopt(name = "validate")]
pub struct CleanCommand {}

impl CleanCommand {
    pub fn execute(&self) -> Result<()> {
        // Remove cached data, e.g. engines.
        sightglass_artifact::clean()?;

        // Remove log files.
        let log_file_regex = Regex::new(r"^(stdout|stderr)\-\w+\-\d+-\d+.log$").unwrap();
        for entry in std::env::current_dir()?.read_dir()? {
            let entry = entry?;

            // Only consider files, not dirs or symlinks.
            if !entry.file_type()?.is_file() {
                continue;
            }

            let path = entry.path();

            // If it doesn't have a file name, it definitely isn't a log file.
            let name = match path.file_name().and_then(|n| n.to_str()) {
                None => continue,
                Some(n) => n,
            };

            // If it doesn't match our log file regex, it isn't a log file.
            if !log_file_regex.is_match(name) {
                continue;
            }

            // Okay! It's one of our log files!
            log::info!("Removing log file: {}", path.display());
            std::fs::remove_file(&path)
                .with_context(|| format!("failed to remove {}", path.display()))?;
        }

        Ok(())
    }
}
