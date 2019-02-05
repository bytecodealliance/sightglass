use super::bench::{Test, TestBody, TestBodyFn, TestSetupFn, TestTeardownFn};
use super::errors::*;
use goblin::mach::{self, Mach};
use goblin::Object;
use libloading::{Library, Symbol};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Names of the setup/body/teardown symbols for a given test
#[derive(Debug)]
pub struct TestSymbol {
    pub test_name: String,
    pub setup: Option<String>,
    pub bodies: Vec<String>,
    pub teardown: Option<String>,
}

/// Return the list of exported functon names (including weak symbols for ELF)
/// contained in a shared library.
/// ELF and Mach-O (binary) are supported.
fn exported_names_from_library<P: AsRef<Path>>(path: P) -> Result<Vec<String>, BenchError> {
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;
    let mut names = vec![];
    match Object::parse(&buffer).map_err(|e| BenchError::ParseError(e.to_string()))? {
        Object::Elf(elf) => {
            for symbol in elf
                .dynsyms
                .iter()
                .filter(|symbol| symbol.st_info == 0x12 || symbol.st_info == 0x22)
            {
                names.push(
                    elf.dynstrtab
                        .get(symbol.st_name)
                        .ok_or_else(|| {
                            BenchError::ParseError(format!("symbol not found: {}", symbol.st_name))
                        })?
                        .map_err(|e| BenchError::ParseError(e.to_string()))?
                        .to_string(),
                );
            }
        }
        Object::Mach(Mach::Binary(macho)) => {
            for symbol in macho
                .symbols
                .ok_or_else(|| BenchError::ParseError("symbols not found".to_string()))?
                .iter()
            {
                match symbol {
                    Ok((
                        name,
                        mach::symbols::Nlist {
                            n_type: 0xf,
                            n_sect: 1,
                            ..
                        },
                    )) if name.len() > 1 && name.starts_with('_') => {
                        names.push(name[1..].to_string())
                    }
                    _ => {}
                }
            }
        }
        _ => xbail!(BenchError::Unsupported),
    }
    Ok(names)
}

/// Returns the test name according to a function name
/// or `None` if the function name doesn't match SightGlass conventions
fn body_name_root(name: &str) -> Option<&str> {
    if name.starts_with('_') || name.ends_with("_setup") || name.ends_with("_teardown") {
        return None;
    }
    if name.ends_with("_body") {
        return Some(&name[..name.len() - "_body".len()]);
    }
    if name.contains("_body_") {
        return name.split("_body_").next();
    }
    None
}

/// Registers a test. If there was already a body with the same root,
/// the same `setup` and `teardown` functions will be used.
fn register_test_symbol(
    tests_symbols: &mut Vec<TestSymbol>,
    names: &[String],
    body_name: String,
) -> Result<(), BenchError> {
    let test_name = body_name_root(&body_name)
        .ok_or(BenchError::InternalError("Bogus body name"))?
        .to_string();
    if let Some(previous_test_symbol) = tests_symbols
        .iter_mut()
        .find(|test_symbol| test_symbol.test_name == test_name)
    {
        previous_test_symbol.bodies.push(body_name);
        return Ok(());
    };
    let setup = {
        let name = format!("{}_setup", test_name);
        if names.contains(&name) {
            Some(name)
        } else {
            None
        }
    };
    let teardown = {
        let name = format!("{}_teardown", test_name);
        if names.contains(&name) {
            Some(name)
        } else {
            None
        }
    };
    let test_symbol = TestSymbol {
        test_name,
        bodies: vec![body_name],
        setup,
        teardown,
    };
    tests_symbols.push(test_symbol);
    Ok(())
}

/// Extract all the test-related symbols from a shared library.
pub fn extract_tests_symbols<P: AsRef<Path>>(path: P) -> Result<Vec<TestSymbol>, BenchError> {
    let names = exported_names_from_library(path)?;
    let mut body_names: Vec<String> = names
        .iter()
        .filter_map(|name| body_name_root(&name).and(Some(name.to_string())))
        .collect();
    body_names.sort();
    let mut test_symbols: Vec<TestSymbol> = vec![];
    for body_name in body_names {
        register_test_symbol(&mut test_symbols, &names, body_name)?;
    }
    Ok(test_symbols)
}

/// Resolve symbol names to function pointers
pub fn resolve(tests_symbols: &[TestSymbol], library: &Library) -> Vec<Test> {
    tests_symbols
        .iter()
        .map(|test_symbol| {
            let bodies: Vec<_> = test_symbol
                .bodies
                .iter()
                .map(|body| unsafe {
                    let body_sym: Symbol<TestBodyFn> = library.get(body.as_bytes()).unwrap();
                    TestBody {
                        name: body.to_string(),
                        body_fn: *body_sym.into_raw(),
                    }
                })
                .collect();
            let setup_fn = test_symbol.setup.as_ref().map(|setup| unsafe {
                let setup_sym: Symbol<TestSetupFn> = library.get(setup.as_bytes()).unwrap();
                *setup_sym.into_raw()
            });
            let teardown_fn = test_symbol.teardown.as_ref().map(|teardown| unsafe {
                let teardown_sym: Symbol<TestTeardownFn> =
                    library.get(teardown.as_bytes()).unwrap();
                *teardown_sym.into_raw()
            });
            Test {
                name: test_symbol.test_name.clone(),
                bodies,
                setup_fn,
                teardown_fn,
            }
        })
        .collect()
}
