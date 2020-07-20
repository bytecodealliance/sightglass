use super::errors::*;
use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

pub const DEFAULT_CONFIG_FILE: &str = "sightglass.toml";
pub const DEFAULT_SINGLE_CORE: bool = true;
pub const DEFAULT_INITIAL_ROUND_SIZE: usize = 1;
pub const DEFAULT_MIN_SAMPLE_SIZE: usize = 10;
pub const DEFAULT_MIN_RUN_TIME_MS: u64 = 100;
pub const DEFAULT_MAX_RUN_TIME_MS: u64 = 2_000;
pub const ENV_QUICK_MODE: &str = "QUICK_MODE";

/// A test suite
#[derive(Deserialize, Debug, Default)]
pub struct TestSuite {
    pub name: String,
    pub library_path: String,
    pub guard: Option<Vec<String>>,
}

/// An output configuration
#[derive(Deserialize, Debug, Default)]
pub struct OutputConfig {
    pub format: String,
    pub file: Option<String>,
    pub breakdown: Option<bool>,
    pub perf: Option<bool>,
}

/// Global configuration
#[derive(Deserialize, Debug)]
pub struct Config {
    pub test_suites: Vec<TestSuite>,
    pub output: Vec<OutputConfig>,
    pub single_core: Option<bool>,
    pub initial_round_size: Option<usize>,
    pub min_sample_size: Option<usize>,
    pub min_run_time_ms: Option<u64>,
    pub max_run_time_ms: Option<u64>,
}

#[derive(Default)]
struct CmdlineConfig {
    config_file: String,
    quick: bool,
}

impl Config {
    /// Load a configuration file in TOML format
    fn parse_file(path: &Path) -> Result<Config, BenchError> {
        let mut fd = File::open(path)?;
        let mut toml_str = String::new();
        fd.read_to_string(&mut toml_str)?;
        let config: Config =
            toml::from_str(&toml_str).map_err(|e| BenchError::ParseError(e.to_string()))?;
        Ok(config)
    }

    /// Parse the command-line arguments, return the path to the configuration file
    fn parse_cmdline() -> Result<CmdlineConfig, BenchError> {
        let matches = App::new("sightglass")
            .about("A test tube rack")
            .arg(
                Arg::with_name("config-file")
                    .short("c")
                    .long("conf")
                    .takes_value(true)
                    .default_value(DEFAULT_CONFIG_FILE)
                    .help("Path to the configuration file"),
            )
            .arg(
                Arg::with_name("quick")
                    .short("q")
                    .long("quick")
                    .takes_value(false)
                    .help("Quick mode - Run tests only once"),
            )
            .get_matches();
        let config_file = matches
            .value_of("config-file")
            .ok_or(BenchError::InternalError("Empty config file"))?
            .to_string();
        let cmdline_config = CmdlineConfig {
            config_file,
            quick: matches.is_present("quick"),
        };
        Ok(cmdline_config)
    }

    /// Parse the command-line and the configuration file
    pub fn parse() -> Result<Config, BenchError> {
        let cmdline_config = Self::parse_cmdline()?;
        let mut config = Self::parse_file(Path::new(&cmdline_config.config_file))?;
        if cmdline_config.quick {
            config.initial_round_size = Some(1);
            config.min_run_time_ms = Some(1);
            config.min_sample_size = Some(1);
            env::set_var(ENV_QUICK_MODE, "1");
        }
        Ok(config)
    }
}
