fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    prost_build::Config::new()
        .out_dir(&out_dir)
        .compile_protos(&["../schema.proto"], &["../"])
        .unwrap();
}
