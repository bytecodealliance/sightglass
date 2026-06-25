use super::util::sightglass_cli;
use assert_cmd::prelude::*;

/// clean does not accept unknown flags.
#[test]
fn clean_rejects_unknown_flags() {
    sightglass_cli()
        .args(["clean", "--unknown-flag"])
        .assert()
        .failure();
}
