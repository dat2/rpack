#![feature(box_patterns)]

extern crate clap;
extern crate easter;
extern crate esprit;
extern crate failure;
extern crate hex;
extern crate petgraph;
extern crate ring;
extern crate victoria_dom;

mod assets;
mod context;
mod codegen;
mod modules;
mod parser;

use clap::{App, Arg, SubCommand};
use context::Context;
use failure::Error;

fn build(entry: &str) -> Result<(), Error> {
    let context = Context::new();
    let (graph, entry_point_id) = parser::parse_module_graph(&context, &entry.to_owned())?;
    codegen::codegen(graph, entry_point_id)?;
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
