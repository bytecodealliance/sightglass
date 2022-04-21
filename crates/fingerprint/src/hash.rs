//! Provide a consistent way for hashing build files and strings.
use sha2::{Digest, Sha256};
use std::{fs::File, io, path::Path};

/// Calculate the SHA256 hash of a file.
pub fn file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(&path).expect("the benchmark to be a file that can be opened");
    let mut hasher = Sha256::new();
    let _ = io::copy(&mut file, &mut hasher).expect("to be able to hash the benchmark bytes");
    let hash = hasher.finalize();
    hexify(hash.as_slice())
}

/// Calculate the SHA256 hash of a string.
pub(crate) fn string(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    hexify(hash.as_slice())
}

/// Create a hexadecimal string from a sequence of bytes.
pub(crate) fn hexify(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    for byte in bytes {
        write!(&mut s, "{:x}", byte).expect("unable to write byte as hex");
    }
    s
}

/// Create a hexadecimal string from a sequence of bytes.
pub fn slug(s: &str) -> &str {
    &s[0..8]
}
