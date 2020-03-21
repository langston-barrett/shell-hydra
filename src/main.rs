// --------------------------------------------------------------
// * shell-hydra

extern crate shell_hydra_lib;
use shell_hydra_lib::{go, Element};

use std::fs::File;
use std::io::prelude::*;

extern crate serde_json;

extern crate clap;
use clap::{App, Arg};

// --------------------------------------------------------------
// ** Main

fn main() -> std::io::Result<()> {
    let matches = App::new("shell-hydra")
        .version("0.1")
        .about("Create keybindings for groups of related shell commands")
        .author("Langston B.")
        .arg(
            Arg::with_name("CONFIG")
                .help("Configuration file")
                .required(true)
                .index(1),
        )
        .get_matches();
    let path = matches.value_of("CONFIG").unwrap().to_owned();
    let mut file = File::open(&path).expect(&format!("Couldn't open config: {}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Couldn't read config: {}", path));
    let elts: Vec<Element> =
        serde_json::from_str(&contents).expect(&format!("Couldn't parse config: {}", path));
    go(elts)
}
