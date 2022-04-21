use super::util::sightglass_cli;
use assert_cmd::prelude::*;

#[test]
fn help() {
    sightglass_cli().arg("help").assert().success();
}
