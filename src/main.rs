// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::io;

use clap::{Arg, App};
use toml::Value;

error_chain!{}

fn main() {
    let matches = App::new(crate_name!())
                          .version(crate_version!())
                          .author(crate_authors!())
                          .about(crate_description!())
                          .arg(Arg::with_name("file")
                               .short("f")
                               .long("file")
                               .value_name("TOML_FILE")
                               .help("TOML file to read")
                               .takes_value(true))
                          .arg(Arg::with_name("PATTERN")
                               .help("Field to read from the TOML file")
                               .required(true)
                               .index(1))
                          // .arg(Arg::with_name("url")
                          //      .short("u")
                          //      .long("url")
                          //      .value_name("URL_PATH")
                          //      .help("TOML file URL to read")
                          //      .takes_value(true))
                          .get_matches();

    let toml_file = match (matches.value_of("file"), matches.value_of("url")) {
        (Some(f), None) => {
            load_toml_from_file(f).unwrap()
        }
        (None, Some(_u)) => {
            unimplemented!()
        }
        (None, None) => {
            eprintln!("Must specify URL or File to load!");
            ::std::process::exit(-1)
        }
        (Some(_), Some(_)) => {
            eprintln!("Cannot specify URL and File!");
            ::std::process::exit(-1)
        }
    };

    let pattern = matches
        .value_of("PATTERN").unwrap();

    let x = pattern
        .split('.')
        .fold(Some(&toml_file), |acc, key| {
            match acc {
                Some(a) => a.get(key),
                None => None
            }
        });

    exit(match x {
        Some(needle) => {
            println!("{}", format!("{}", needle).trim_matches('"'));
            0
        }
        None => {
            writeln!(io::stderr(), "{} not found!", pattern).unwrap();
            -1
        }

    });
}

fn load_toml_from_file(file_name: &str) -> Result<Value> {
    let mut file = File::open(file_name)
        .chain_err(|| format!("Failed to open file: {:?}", &file_name))?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    toml::from_str(&contents).chain_err(|| "Deserialize error")
}
