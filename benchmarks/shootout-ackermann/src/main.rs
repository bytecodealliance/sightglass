use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("Building native - shootout-ackermann");
    let output = Command::new("cc")
        .args(["-O3", "-fPIC", "-I.", "-L../../engines/native/", "-shared", "-o", "./target/benchmark.so", "benchmark.c", "-lengine"])
        .output()
        .expect("failed to compile benchmark");
    println!("status: {}",  output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
