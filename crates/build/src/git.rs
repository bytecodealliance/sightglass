use anyhow::{bail, Result};

/// Use the `git2` crate to resolve a revision in a repository into a commit SHA.
///
/// See [this example] for a more comprehensive version of this function.
///
/// [this example]: https://github.com/rust-lang/git2-rs/blob/master/examples/ls-remote.rs
pub fn resolve_to_commit(repository: &str, revision: &str) -> Result<String> {
    let mut remote = git2::Remote::create_detached(repository)?;
    let connection = remote.connect_auth(git2::Direction::Fetch, None, None)?;
    for head in connection.list()?.iter() {
        if head.name().ends_with(revision) {
            return Ok(head.oid().to_string());
        }
    }
    bail!(
        "in repository {}, unable to find commit for revision: {}",
        repository,
        revision
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve() {
        let repo = "https://github.com/bytecodealliance/wasmtime";
        let revision = "v0.33.1"; // A tag (but a branch or commit would work as well).
        let commit = resolve_to_commit(repo, revision).unwrap();
        assert_eq!(&commit[0..7], "5215c78");
    }
}
