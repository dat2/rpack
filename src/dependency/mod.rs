use context::Context;
use failure::Error;
use javascript::ast::*;
use javascript::{parse_js_module, JsModule};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::path::{Path, PathBuf};

fn resolve_paths_to_absolute(
    context: &Context,
    module: &mut JsModule,
) -> Result<Vec<PathBuf>, Error> {
    let mut dependencies = Vec::new();
    for mut statement in &mut module.program.body {
        if let match_ast!(import [_] from [ref mut import_path]) = statement {
            let resolved_dep = context.resolve(&module.path, &import_path)?;
            *import_path = format!("{}", resolved_dep.display());
            dependencies.push(resolved_dep);
        }
    }
    Ok(dependencies)
}

fn parse_with_graph_recursive<P: AsRef<Path>>(
    context: &Context,
    path: &P,
    mut graph: &mut Graph<JsModule, usize>,
) -> Result<NodeIndex, Error> {
    let mut module = parse_js_module(&path)?;
    let dependencies = resolve_paths_to_absolute(context, &mut module)?;

    let module_index = graph.add_node(module);
    for dep in &dependencies {
        let dep_index = parse_with_graph_recursive(context, &dep, &mut graph)?;
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
