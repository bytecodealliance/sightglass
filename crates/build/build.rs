use std::path::PathBuf;
fn main() {
    println!(
        "cargo:rustc-env=SIGHTGLASS_PROJECT_DIRECTORY={}",
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .display()
    );
}
