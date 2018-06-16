use failure::Error;
use modules::Module;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::path::Path;

mod html;
mod javascript;
mod unknown;

fn parse_with_graph_recursive<P: AsRef<Path>>(
    input_path: &P,
    mut graph: &mut Graph<Module, usize>,
) -> Result<NodeIndex, Error> {
    let p = input_path.as_ref();
    let (module, dependencies) = match p.extension() {
        None => unknown::parse_unknown_module(input_path)?,
        Some(os_str) => match os_str.to_str() {
            Some("html") => html::parse_html_module(input_path)?,
            Some("js") => javascript::parse_javascript_module(input_path)?,
            _ => unknown::parse_unknown_module(input_path)?,
        },
    };
    let module_index = graph.add_node(module);

    for dep in dependencies {
        let dep_index = parse_with_graph_recursive(&dep, &mut graph)?;
        graph.add_edge(module_index, dep_index, 0);
    }

    Ok(module_index)
}

pub fn parse_module_graph<P: AsRef<Path>>(input_path: &P) -> Result<Graph<Module, usize>, Error> {
    let mut result = Graph::new();
    parse_with_graph_recursive(input_path, &mut result)?;
    Ok(result)
}
