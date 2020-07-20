#[macro_use]
extern crate failure;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate xfailure;

mod bench;
mod config;
mod cpu_affinity;
mod errors;
mod out;
mod perf;
mod symbols;

use self::bench::bench;
use self::config::Config;
use self::errors::*;
use std::process;

fn _main() -> Result<(), BenchError> {
    let config = Config::parse()?;
    let _ = cpu_affinity::tune(&config);
    bench(&config)?;
    Ok(())
}

fn main() {
    process::exit(match _main() {
        Ok(_) => 0,
        Err(ref e) => {
            eprintln!("{}", e);
            1
        }
    });
}
