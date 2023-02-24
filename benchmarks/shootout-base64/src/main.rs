use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("Building native - shootout-base64");
    let output = Command::new("cc")
        .args(["-O3","-Dmain=native_entry", "-fPIC", "-I.", "-L../../engines/native/", "-shared", "-o", "./target/benchmark.so", "benchmark.c", "-lengine"])
        .output()
        .expect("failed to compile native benchmark");
    println!("status: {}",  output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
/*
    let output = Command::new("objcopy")
        .args(["--redefine-sym", "main=native_entry", "./target/benchmark.so", "./target/benchmark.so"])
        .output()
        .expect("rename native symbols failed");
    println!("status: {}",  output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    */
}

