#![feature(box_patterns)]

extern crate clap;
extern crate easter;
extern crate esprit;
extern crate failure;
extern crate petgraph;
extern crate victoria_dom;

mod modules;
mod parsers;

use clap::{App, Arg, SubCommand};
use failure::Error;
use parsers::parse_module_graph;

fn build(entry: &str) -> Result<(), Error> {
    let graph = parse_module_graph(&entry.to_owned());
    println!("{:?}", graph);
    Ok(())
}

fn run() -> Result<(), Error> {
    let matches = App::new("Rust Pack")
        .version("0.1")
        .author("Nick D. <nickdujay@gmail.com>")
        .about("Compiles a web project")
        .subcommand(
            SubCommand::with_name("build")
                .about("Builds the project for production")
                .arg(
                    Arg::with_name("ENTRY")
                        .help("Sets the entry file to use")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        build(matches.value_of("ENTRY").unwrap())?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
