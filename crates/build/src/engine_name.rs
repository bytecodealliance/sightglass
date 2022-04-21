use crate::{hash, BuildInfo};
use anyhow::{anyhow, Result};
use std::str;
use std::{borrow::Cow, fmt, path::Path};

/// Describes the shortened name for a built engine (i.e., an alias).
///
/// ```
/// # use sightglass_build::EngineName;
/// let en: EngineName = "<engine>-<slug>".parse().unwrap();
/// assert_eq!(en.to_string(), "<engine>-<slug>");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct EngineName<'a>(Cow<'a, str>, Cow<'a, str>);
impl<'a> EngineName<'a> {
    fn new<C: Into<Cow<'a, str>>>(name: C, slug: C) -> Self {
        Self(name.into(), slug.into())
    }

    /// Calculate an [EngineName] using the path of a Dockerfile--this identifies engines built
    /// from a Dockerfile directly. E.g., `dockerfile-892390f`.
    pub fn from_dockerfile<P: AsRef<Path>>(path: P) -> Self {
        let hash = hash::file(path); // TODO return Result<Self>
        Self::new("dockerfile".to_string(), hash::slug(&hash).to_string())
    }

    /// Calculate an [EngineName] from [BuildInfo]--this identifies a well-known engine by name and
    /// commit, if present, and by a hash of the [BuildInfo] otherwise. E.g., `wasmtime-ab0324d`.
    pub fn from_buildinfo<'b>(buildinfo: &'b BuildInfo) -> Result<Self> {
        let name = buildinfo
            .get("NAME")
            .ok_or(anyhow!("BUILDINFO must contain a NAME variable"))?;
        let hash = hash::string(&buildinfo.as_uri());
        Ok(Self::new(name.to_string(), hash::slug(&hash).to_string()))
    }

    /// Extract the final component of a path to be used as an [EngineName]--this is useful for
    /// parsing names from the file system. The name is validated for well-formedness. E.g.
    /// `/home/user/cache/engine-ab32ddf` -> `engine-ab32ddf`.
    ///
    /// ```
    /// # use sightglass_build::EngineName;
    /// # use std::path::PathBuf;
    /// # use std::convert::TryFrom;
    /// let en = EngineName::from_cache_directory(PathBuf::from("/home/user/cache/<engine>-<slug>").as_path()).unwrap();
    /// assert_eq!(en.to_string(), "<engine>-<slug>");
    /// ```
    pub fn from_cache_directory<'b>(path: &'b Path) -> Result<Self> {
        path.file_name()
            .ok_or(anyhow!("directory path must have a final component"))?
            .to_string_lossy()
            .to_string()
            .parse()
    }
}

impl<'a> str::FromStr for EngineName<'a> {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, slug) = s.split_once("-").ok_or(anyhow!(
            "expected engine name to be hyphenated, e.g.: <engine>-<slug>"
        ))?;
        Ok(Self::new(name.to_string(), slug.to_string()))
    }
}

impl fmt::Display for EngineName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}
