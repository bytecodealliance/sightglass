use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("Building native shootout-seqhash");
    let output = Command::new("cc")
        .arg("-O3")
        .arg("-fPIC")
        .arg("-I.")
        .arg("-L../../engines/native/")
        .arg("-shared")
        .arg("-o")
        .arg("bench_native.so")
        .arg("benchmark.c")
        .arg("-lengine")
        .output()
        .expect("failed to execute process");

    println!("status: {}",  output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
 }
