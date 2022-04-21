use anyhow::Result;
use sightglass_build::engine::list_engines;
use sightglass_build::EngineName;
use sightglass_fingerprint::Engine;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

/// List the built Wasm engines known to the Sightglass cache.
#[derive(Debug, StructOpt)]
#[structopt(name = "list-engines")]
pub struct ListEnginesCommand {
    /// Force each entry on to a single line.
    #[structopt(long, short)]
    oneline: bool,
}

impl ListEnginesCommand {
    pub fn execute(&self) -> Result<()> {
        let mut stdout = io::stdout();
        for (name, path) in list_engines()? {
            let fingerprint = (!self.oneline).then(|| Engine::fingerprint(&path));
            write_engine_info(&mut stdout, &name, &path, fingerprint)?;
        }
        Ok(())
    }
}

/// Write a single entry of the `list-engines` command.
pub fn write_engine_info(
    dest: &mut dyn io::Write,
    name: &EngineName,
    path: &PathBuf,
    fingerprint: Option<Result<Engine>>,
) -> Result<()> {
    writeln!(dest, "{}", name)?;
    match fingerprint {
        Some(Err(err)) => writeln!(dest, "  Unable to fingerprint: {}", err)?,
        Some(Ok(fingerprint)) => {
            check_name_consistency(&name.to_string(), &fingerprint.name);
            writeln!(dest, "  Path: {}", path.display())?;
            if let Some(rebuild) = fingerprint.rebuild {
                writeln!(dest, "  Rebuild command: {}", rebuild)?;
            }
            if let Some(buildinfo) = fingerprint.buildinfo {
                writeln!(dest, "  .build-info:")?;
                for line in buildinfo.lines() {
                    writeln!(dest, "    {}", line)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

/// The build command should always generate an engine shared library inside a directory with a name
/// that matches the `.build-info` name. If for some reason this changed, it would not be a large
/// issue, but we can warn users that something might have changed.
fn check_name_consistency(directory_name: &str, fingerprint_name: &str) {
    if directory_name != fingerprint_name {
        log::warn!(
            "The cache directory name and the fingerprint name do not match: {} != {}",
            directory_name,
            fingerprint_name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::str::FromStr;

    #[test]
    fn list_engine_single_line() {
        assert_eq!(
            write("wasmtime-abcdef", "../ignored", None).unwrap(),
            "wasmtime-abcdef\n"
        );
    }

    #[test]
    fn list_engine_failed_fingerprint() {
        assert_eq!(
            write("wasmtime-abcdef", "../ignored", Some(Err(anyhow!("...")))).unwrap(),
            "wasmtime-abcdef\n  Unable to fingerprint: ...\n"
        );
    }

    #[test]
    fn list_engine_with_fingerprint() {
        let name = "wasmtime-abcdef";
        let path = "/some/path/to/cached/wasmtime-abcdef";
        let fingerprint = Engine {
            name: name.to_string(),
            path: path.to_string(),
            rebuild: Some("sightglass-cli build-engine wasmtime?REVISION=1234567".to_string()),
            buildinfo: Some("NAME=wasmtime\n_COMMIT=1234567\n_RUSTC=1.60.0\n".to_string()),
        };
        assert_eq!(
            write(name, path, Some(Ok(fingerprint))).unwrap(),
            r#"wasmtime-abcdef
  Path: /some/path/to/cached/wasmtime-abcdef
  Rebuild command: sightglass-cli build-engine wasmtime?REVISION=1234567
  .build-info:
    NAME=wasmtime
    _COMMIT=1234567
    _RUSTC=1.60.0
"#
        );
    }

    fn write(name: &str, path: &str, fingerprint: Option<Result<Engine>>) -> Result<String> {
        let mut buffer = vec![];
        write_engine_info(
            &mut buffer,
            &EngineName::from_str(name)?,
            &PathBuf::from(path),
            fingerprint,
        )?;
        let buffer = std::str::from_utf8(&buffer[..])?.to_string();
        Ok(buffer)
    }
}
