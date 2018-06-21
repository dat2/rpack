use context::Context;
use failure::Error;
use hex;
use javascript::{parse_js_module, JsModule};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use ring::digest;
use std::path::Path;

fn parse_with_graph_recursive<P: AsRef<Path>>(
    context: &Context,
    path: &P,
    mut graph: &mut Graph<JsModule, usize>,
) -> Result<NodeIndex, Error> {
    let module = parse_js_module(&path)?;
    let dependencies = module.get_dependencies();

    let module_path = module.path.clone();
    let module_index = graph.add_node(module);
    for dep in dependencies {
        let resolved_dep = context.resolve(&module_path, dep)?;
        let dep_index = parse_with_graph_recursive(context, &resolved_dep, &mut graph)?;
        graph.add_edge(module_index, dep_index, 0);
    }

    Ok(module_index)
}

pub fn parse_dependency_graph<P: AsRef<Path>>(
    context: &Context,
    input_path: &P,
) -> Result<(Graph<JsModule, usize>, NodeIndex), Error> {
    let mut result = Graph::new();
    let entry_point_id = parse_with_graph_recursive(context, input_path, &mut result)?;
    Ok((result, entry_point_id))
}

pub fn generate_module_id(source: &str) -> String {
    let bytes = digest::digest(&digest::SHA512, source.as_bytes());
    let mut hex_encoded = hex::encode(bytes);
    hex_encoded.truncate(4);
    hex_encoded
}
