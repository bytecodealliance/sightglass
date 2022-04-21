use anyhow::Result;
use std::fmt::Write;
use std::fs;
use std::str::FromStr;
use std::{collections::BTreeMap, fmt, io, iter::FromIterator, path::Path};

/// The default file name used for [`BuildInfo`] files.
pub const DEFAULT_FILE_NAME: &'static str = ".build-info";

/// A collection of variable-value pairs used to reproduce a build of some artifact. These pairs
/// are representable as either:
/// - a URI-like string, e.g., `wasmtime?REPOSITORY=https://...+COMMIT=...
/// - a newline-separated file, e.g.,
///   ```text
///   NAME=wasmtime
///   REPOSITORY=https://...
///   COMMIT=...
///   ```
#[derive(Debug, PartialEq)]
pub struct BuildInfo(BTreeMap<String, String>);
impl BuildInfo {
    /// Construct a [`BuildInfo`] from key-value pairs.
    pub fn new<K, V>(pairs: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        let mut map = BTreeMap::new();
        for (var, val) in pairs {
            map.insert(var.into(), val.into());
        }
        Self(map)
    }

    /// Return the [`BuildInfo`] name; every artifact with associated [`BuildInfo`] is expected to have
    /// a name.
    ///
    /// ```
    /// # use sightglass_build::BuildInfo;
    /// assert_eq!(Some("test"), BuildInfo::parse_uri("test").unwrap().name());
    /// assert_eq!(Some("test"), BuildInfo::parse_uri("test?a=b").unwrap().name());
    /// assert_eq!(None, BuildInfo::parse_uri("a=b").unwrap().name());
    /// ```
    pub fn name(&self) -> Option<&str> {
        self.get("NAME")
    }

    /// Extract one of the [`BuildInfo`] values.
    ///
    /// ```
    /// # use sightglass_build::BuildInfo;
    /// assert_eq!(Some("test"), BuildInfo::parse_uri("test").unwrap().get("NAME"));
    /// assert_eq!(Some("b"), BuildInfo::parse_uri("test?a=b").unwrap().get("a"));
    /// assert_eq!(None, BuildInfo::parse_uri("a=b").unwrap().get("c"));
    /// ```
    pub fn get(&self, variable: &str) -> Option<&str> {
        self.0.get(variable).map(String::as_str)
    }

    /// Modify one of the [`BuildInfo`] values.
    ///
    /// ```
    /// # use sightglass_build::BuildInfo;
    /// let mut buildinfo = BuildInfo::parse_uri("a").unwrap();
    /// buildinfo.set("NAME", "b");
    /// assert_eq!("b", buildinfo.as_uri());
    /// ```
    pub fn set<S: Into<String>>(&mut self, variable: S, value: S) -> Option<String> {
        self.0.insert(variable.into(), value.into())
    }

    /// Modify one of the [`BuildInfo`] values.
    ///
    /// ```
    /// # use sightglass_build::BuildInfo;
    /// let buildinfo = BuildInfo::parse_uri("a").unwrap();
    /// let mut iter = buildinfo.iter();
    /// assert_eq!(Some(("NAME", "a")), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a str, &'a str)> {
        self.0.iter().map(|(a, b)| (a.as_str(), b.as_str()))
    }

    /// Parse [`BuildInfo`] from a URI-like string; e.g., `<name>`,
    /// `<name>?<var1>=<val1>+<var2>=<val2>`
    ///
    /// ```
    /// # use sightglass_build::BuildInfo;
    /// assert_eq!("test", BuildInfo::parse_uri("test").unwrap().as_uri());
    /// assert_eq!("a=b", BuildInfo::parse_uri("a=b").unwrap().as_uri());
    /// assert_eq!("test?a=b", BuildInfo::parse_uri("test?a=b").unwrap().as_uri());
    /// assert_eq!("test?a=b+c=d", BuildInfo::parse_uri("test?a=b+c=d").unwrap().as_uri());
    /// assert_eq!("test?a=b+c='d e'", BuildInfo::parse_uri("test?a=b+c='d e'").unwrap().as_uri());
    /// ```
    pub fn parse_uri(uri: &str) -> Result<Self> {
        if let Some((name, rest)) = uri.split_once("?") {
            let mut b = BuildInfo::from_iter(rest.split("+"));
            b.0.insert("NAME".to_string(), name.to_string());
            Ok(b)
        } else if uri.contains("=") {
            Ok(BuildInfo::from_iter(uri.split("+")))
        } else {
            Ok(BuildInfo::new([("NAME", uri)]))
        }
    }

    /// Emit [`BuildInfo`] as a URI-like string; e.g., `<name>`,
    /// `<name>?<var1>=<val1>+<var2>=<val2>`. See [`Self::parse_uri()`].
    pub fn as_uri(&self) -> String {
        let mut uri = self.name().unwrap_or("").to_owned();

        // Interject "+" between each printed variable-value pair. Using "&" is problematic due to
        // its creation of background processes in shells.
        let mut iter = self.0.iter().filter(|(var, _)| var != &"NAME");
        if let Some(pair) = iter.next() {
            if uri.len() > 0 {
                write!(&mut uri, "?").unwrap();
            }
            write_pair(&mut uri, pair).unwrap();
        }
        for pair in iter {
            write!(&mut uri, "+").unwrap();
            write_pair(&mut uri, pair).unwrap();
        }

        uri
    }

    /// Parse [`BuildInfo`] from newline-separated file contents; e.g.:
    ///
    /// ```
    /// # use sightglass_build::BuildInfo;
    /// let b = BuildInfo::parse_file_string("A=1
    ///  NAME=C
    ///  D=E F G").unwrap();
    /// assert_eq!(b.as_uri(), "C?A=1+D='E F G'")
    /// ```
    pub fn parse_file_string(file_contents: &str) -> Result<Self> {
        use io::BufRead;
        Ok(io::BufReader::new(file_contents.as_bytes())
            .lines()
            .filter_map(Result::ok)
            .map(|l| l.trim().to_string())
            .filter(|l| !l.starts_with("#"))
            .collect())
    }

    /// Parse [`BuildInfo`] from a file. See [`Self::parse_file_string()`].
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::parse_file_string(&fs::read_to_string(path.as_ref())?)
    }

    /// Emit [`BuildInfo`] as newline-separated string; e.g.:
    ///
    /// ```
    /// # use sightglass_build::BuildInfo;
    /// let b = BuildInfo::parse_uri("test?A=1+B='2 3'").unwrap();
    /// assert_eq!(b.as_file_string(), "A=1
    /// B='2 3'
    /// NAME=test
    /// ");
    /// ```
    pub fn as_file_string(&self) -> String {
        let mut s = String::new();
        for pair in self.0.iter() {
            write_pair(&mut s, pair).unwrap();
            write!(&mut s, "\n").unwrap();
        }
        s
    }

    /// Generate a new [`BuildInfo`] with only the NAME and any differences from `defaults`; any
    /// variables not known by `defaults` are discarded. Also, this ignores any variables that begin
    /// with `_`.
    ///
    /// ```
    /// # use sightglass_build::BuildInfo;
    /// let b1 = BuildInfo::parse_uri("test?A=1+B=42+_D=4").unwrap();
    /// let b2 = BuildInfo::parse_uri("test?A=1+B=2+C=3").unwrap();
    /// // Only the settings in `b1` that are different than `b2` are retained. `_D` is ignored.
    /// assert_eq!(b1.diff(b2).as_uri(), "test?B=42");
    /// ```
    pub fn diff(&self, defaults: BuildInfo) -> BuildInfo {
        BuildInfo::new(
            self.0
                .iter()
                .filter(|(var, _)| !var.starts_with("_"))
                .filter(|(var, val)| {
                    *var == "NAME"
                        || match defaults.0.get(*var) {
                            Some(default_val) => *val != default_val,
                            None => false,
                        }
                }),
        )
    }

    /// Merge the given [`BuildInfo`] on top of the `self` [`BuildInfo`].
    ///
    /// ```
    /// # use sightglass_build::BuildInfo;
    /// let b1 = BuildInfo::parse_uri("a=1+b=0").unwrap();
    /// let b2 = BuildInfo::parse_uri("b=2+c=3").unwrap();
    /// // `b2` overwrites `b1` as it is merged in
    /// assert_eq!(b1.merge(&b2).as_uri(), "a=1+b=2+c=3");
    /// ```
    pub fn merge(mut self, incoming: &BuildInfo) -> BuildInfo {
        for (var, val) in &incoming.0 {
            let _ = self.0.insert(var.to_string(), val.to_string());
        }
        self
    }
}

/// Collect a [`BuildInfo`] from a URI-like string.
///
/// # Panics
///
/// This will panic if the `+`-delimited parts cannot be split into pairs.
impl FromStr for BuildInfo {
    type Err = anyhow::Error;
    fn from_str(uri: &str) -> Result<Self, Self::Err> {
        BuildInfo::parse_uri(uri)
    }
}

/// Collect a [`BuildInfo`] from lines of text parsed into key-value pairs.
///
/// # Panics
///
/// This will panic if the lines cannot be split into pairs.
impl<L> FromIterator<L> for BuildInfo
where
    L: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = L>>(iter: T) -> Self {
        let mut map = BTreeMap::new();
        for line in iter {
            let (var, val) = split_pair(line.as_ref());
            map.insert(var.into(), val.into());
        }
        Self(map)
    }
}

impl fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_uri())
    }
}

/// Split a string by `+`.
///
/// # Panics
///
/// Panics if the string cannot be split.
fn split_pair(line: &str) -> (&str, &str) {
    let (var, val) = line
        .trim()
        .split_once("=")
        .expect("expected equals-delimited lines");
    (var, val.trim_matches(|c| c == '"' || c == '\''))
}

/// Write a variable-value pair, escaping as necessary.
fn write_pair(f: &mut dyn Write, (var, val): (&String, &String)) -> std::fmt::Result {
    assert!(
        !var.contains(" "),
        "build info variables cannot contain spaces"
    );
    assert!(
        !var.contains("+"),
        "build info variables cannot contain plus"
    );
    assert!(!val.contains("+"), "build info values cannot contain plus");
    assert!(
        !val.contains("'"),
        "build info values cannot contain single quotes"
    );
    if val.contains(" ") {
        write!(f, "{}='{}'", var, val)
    } else {
        write!(f, "{}={}", var, val)
    }
}
