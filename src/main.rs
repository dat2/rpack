#![feature(box_patterns)]

extern crate clap;
extern crate combine;
extern crate easter;
extern crate esprit;
extern crate failure;
extern crate hex;
extern crate joker;
extern crate petgraph;
extern crate ring;
extern crate victoria_dom;

mod codegen;
mod context;
mod dependency;
mod io_utils;
mod javascript;

use clap::{App, Arg, SubCommand};
use context::Context;
use failure::Error;
use std::env;

fn build(context: &Context, entry: &str) -> Result<(), Error> {
    let (graph, entry_point_id) = dependency::parse_dependency_graph(context, &entry.to_owned())?;
    codegen::codegen(&graph, entry_point_id)?;
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

    let cwd = env::current_dir()?;
    let resolve_path =
        env::var("RESOLVE_PATH").unwrap_or(format!("{}/node_modules", cwd.display()));
    let context = Context::new(&resolve_path)?;
    if let Some(matches) = matches.subcommand_matches("build") {
        build(&context, matches.value_of("ENTRY").unwrap())?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
