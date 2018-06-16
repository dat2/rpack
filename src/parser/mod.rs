use context::Context;
use failure::Error;
use modules::{Module, ModuleType};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::path::Path;

mod html;
mod javascript;
mod plaintext;

fn parse_with_graph_recursive<P: AsRef<Path>>(
    context: &Context,
    input_path: &P,
    mut graph: &mut Graph<Module, usize>,
) -> Result<NodeIndex, Error> {
    let module_type = ModuleType::parse_from_path(&input_path);
    let (module, dependencies) = match module_type {
        ModuleType::Html => html::parse_html_module(context, input_path)?,
        ModuleType::JavaScript => javascript::parse_javascript_module(&context, input_path)?,
        ModuleType::Css => plaintext::parse_plaintext_module(input_path)?,
        ModuleType::PlainText => plaintext::parse_plaintext_module(input_path)?,
    };
    let module_index = graph.add_node(module);

    for dep in dependencies {
        let dep_index = parse_with_graph_recursive(context, &dep, &mut graph)?;
        graph.add_edge(module_index, dep_index, 0);
    }

    Ok(module_index)
}

pub fn parse_module_graph<P: AsRef<Path>>(
    context: &Context,
    input_path: &P,
) -> Result<(Graph<Module, usize>, NodeIndex), Error> {
    let mut result = Graph::new();
    let entry_point_id = parse_with_graph_recursive(context, input_path, &mut result)?;
    Ok((result, entry_point_id))
}
