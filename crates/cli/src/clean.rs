use anyhow::{Context, Result};
use regex::Regex;
use structopt::StructOpt;

/// Remove the log files emitted in Sightglass runs.
#[derive(StructOpt, Debug)]
#[structopt(name = "clean")]
pub struct CleanCommand {}

impl CleanCommand {
    pub fn execute(&self) -> Result<()> {
        let log_file_regex = Regex::new(r"^(stdout|stderr)\-\w+\-\d+.log$").unwrap();
        let mut removed_count = 0;

        // Clean log files from both current directory (legacy) and temp directory (new)
        let dirs_to_clean = vec![
            std::env::current_dir()?,
            std::env::temp_dir().join("sightglass-logs"),
        ];

        for dir in dirs_to_clean {
            if !dir.exists() {
                continue;
            }

            let read_dir = match dir.read_dir() {
                Ok(rd) => rd,
                Err(_) => continue,
            };

            for entry in read_dir {
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
                removed_count += 1;
            }
        }

        println!("Removed {} log file(s)", removed_count);
        Ok(())
    }
}
