//! Group benchmarks into suites.
//!
//! [Suite]s are files that contain a newline-separated list of benchmark files
//! to run; see [Suite::parse] for more details on the syntax of these files. To
//! distinguish between a [Suite] file and a regular benchmark file, we use
//! [BenchmarkOrSuite].

use anyhow::Result;
use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    str::FromStr,
};

/// Decide between a suite of benchmarks or an individual benchmark file.
#[derive(Debug)]
pub enum BenchmarkOrSuite {
    Suite(Suite),
    Benchmark(PathBuf),
}

impl BenchmarkOrSuite {
    /// List all of paths to the benchmarks to run.
    pub fn paths(&self) -> Vec<String> {
        match self {
            BenchmarkOrSuite::Suite(suite) => suite
                .benchmarks
                .iter()
                .map(|p| p.display().to_string())
                .collect(),
            BenchmarkOrSuite::Benchmark(path) => vec![path.display().to_string()],
        }
    }
}

/// It is helpful to reference the original path.
impl AsRef<OsStr> for BenchmarkOrSuite {
    fn as_ref(&self) -> &OsStr {
        match self {
            BenchmarkOrSuite::Suite(suite) => suite.path.as_os_str(),
            BenchmarkOrSuite::Benchmark(path) => path.as_os_str(),
        }
    }
}

/// Parse a [BenchmarkOrSuite] from a string path; files ending in `.suite` are
/// assumed to be suite files.
impl FromStr for BenchmarkOrSuite {
    type Err = anyhow::Error;
    fn from_str(path: &str) -> Result<Self, Self::Err> {
        Ok(if Suite::has_extension(path) {
            Self::Suite(Suite::parse(path)?)
        } else {
            Self::Benchmark(PathBuf::from(path))
        })
    }
}

#[derive(Debug)]
pub struct Suite {
    path: PathBuf,
    benchmarks: Vec<PathBuf>,
}

impl Suite {
    /// Parse the contents of a suite file:
    /// - empty lines are ignored
    /// - `#`-prefixed lines are ignored
    /// - relative paths are resolved using the parent directory of the
    ///   `suite_path`.
    pub fn parse<P: AsRef<Path>>(suite_path: P) -> Result<Self> {
        Self::parse_contents(suite_path.as_ref(), &fs::read(suite_path.as_ref())?)
    }

    /// Utility function for easier testing.
    fn parse_contents<P: AsRef<Path>>(suite_path: P, file_contents: &[u8]) -> Result<Self> {
        use io::BufRead;
        let suite_path = suite_path.as_ref().to_path_buf();
        let parent_dir = suite_path
            .parent()
            .expect("the suite path must have a parent directory");
        let mut benchmarks = vec![];
        for line in io::BufReader::new(file_contents)
            .lines()
            .filter_map(Result::ok)
        {
            let line = line.trim();
            if !line.starts_with("#") && !line.is_empty() {
                benchmarks.push(parent_dir.join(line))
            }
        }
        Ok(Self {
            path: suite_path,
            benchmarks,
        })
    }

    /// Check if a path has the `suite` extension.
    fn has_extension<P: AsRef<Path>>(path: P) -> bool {
        match path.as_ref().extension() {
            Some(ext) => ext == "suite",
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUITE_PATH: &str = "/home/bench.suite";
    const CONTENTS: &str = "
        # These benchmarks are...
        a.wasm
        /b.wasm
        ../c.wasm
    ";

    #[test]
    fn parse() {
        let suite = Suite::parse_contents(SUITE_PATH, CONTENTS.as_bytes()).unwrap();

        // The suite path should be preserved.
        assert_eq!(suite.path, PathBuf::from(SUITE_PATH));

        assert_eq!(
            suite.benchmarks,
            vec![
                // Relative paths are appended.
                PathBuf::from("/home/a.wasm"),
                // Absolute paths are preserved.
                PathBuf::from("/b.wasm"),
                // Canonicalization happens later.
                PathBuf::from("/home/../c.wasm")
            ]
        )
    }
}
