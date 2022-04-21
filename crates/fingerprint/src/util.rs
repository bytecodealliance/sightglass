use std::ffi::OsStr;

/// Provide a common way to create `String`s from `OsStr`.
pub(crate) fn stringify<S: AsRef<OsStr>>(s: S) -> String {
    s.as_ref().to_string_lossy().to_string()
}
