#![crate_id="cargo-test"]
#![feature(phase)]

#[phase(plugin, link)]
extern crate cargo;
extern crate serialize;

#[phase(plugin, link)]
extern crate hammer;

use std::os;
use std::io::fs;

use cargo::ops;
use cargo::{execute_main_without_stdin};
use cargo::core::{MultiShell};
use cargo::util;
use cargo::util::{CliResult, CliError};
use cargo::util::important_paths::find_project;

#[deriving(PartialEq,Clone,Decodable)]
struct Options {
    manifest_path: Option<String>,
    rest: Vec<String>
}

hammer_config!(Options "Run the package's test suite")

fn main() {
    execute_main_without_stdin(execute);
}

fn execute(options: Options, shell: &mut MultiShell) -> CliResult<Option<()>> {
    let root = match options.manifest_path {
        Some(path) => Path::new(path),
        None => try!(find_project(os::getcwd(), "Cargo.toml")
                    .map(|path| path.join("Cargo.toml"))
                    .map_err(|_| {
                        CliError::new("Could not find Cargo.toml in this \
                                       directory or any parent directory",
                                      102)
                    }))
    };

    try!(ops::compile(&root, false, "test", shell).map(|_| None::<()>).map_err(|err| {
        CliError::from_boxed(err, 101)
    }));

    let test_dir = root.dir_path().join("target").join("tests");

    let mut walk = try!(fs::walk_dir(&test_dir).map_err(|e| {
        CliError::from_error(e, 1)
    }));

    for file in walk {
        try!(util::process(file).exec().map_err(|e| CliError::from_boxed(e.box_error(), 1)));
    }

    Ok(None)
}
