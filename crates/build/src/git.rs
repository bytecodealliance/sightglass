use serde::{Deserialize, Serialize};
/// Describe a specific Git commit in a repository; this is important for replicating the creation
/// of artifacts. The fields are optional (for now), since in certain cases they can be assumed:
/// the repository of some engines is well-known (e.g. Wasmtime) and the default branch can be used
/// for the revision.
///
/// TODO eventually this and GitSource should merge together.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GitLocation {
    /// The URL of the Git repository.
    pub repository: Option<String>,
    /// A revision in the repository. According to [the Git
    /// documentation](https://mirrors.edge.kernel.org/pub/software/scm/git/docs/gitrevisions.html#_specifying_revisions)
    /// this could be, e.g.:
    /// - a branch name
    /// - a commit hash
    /// - a tag
    pub revision: Option<String>,
}
