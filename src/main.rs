#![feature(box_patterns)]

extern crate clap;
extern crate easter;
extern crate esprit;
extern crate failure;
extern crate hex;
extern crate petgraph;
extern crate ring;
extern crate victoria_dom;

mod context;
mod modules;
mod parser;

use clap::{App, Arg, SubCommand};
use context::Context;
use failure::Error;
use parser::parse_module_graph;
use petgraph::visit::Dfs;
use petgraph::Incoming;

fn build(entry: &str) -> Result<(), Error> {
    let context = Context;
    let (mut graph, entry_point_id) = parse_module_graph(&context, &entry.to_owned())?;
    let mut dfs = Dfs::new(&graph, entry_point_id);
    while let Some(node_index) = dfs.next(&graph) {
        // use a walker -- a detached neighbors iterator
        let mut edges = graph.neighbors_directed(node_index, Incoming).detach();
        while let Some(edge) = edges.next_edge(&graph) {
            let (dep, _) = graph.index_twice_mut(node_index, edge);
            println!("{}", dep.id());
        }
    }
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
