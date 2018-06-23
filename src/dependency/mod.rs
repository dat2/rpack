use context::Context;
use failure::Error;
use javascript::ast::*;
use javascript::{parse_js_module, JsModule};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::path::{Path, PathBuf};

struct DependencyVisitor;

impl Visitor for DependencyVisitor {
    type Result = Vec<PathBuf>;

    fn visit_program(&mut self, program: &Program) -> Self::Result {
        let mut result = Vec::new();
        for statement in &program.body {
            let mut statement_result = self.visit_statement(statement);
            result.append(&mut statement_result);
        }
        result
    }

    fn visit_statement(&mut self, statement: &Statement) -> Self::Result {
        match statement {
            Statement::Import(_, path) => vec![PathBuf::from(path)],
            _ => Vec::new(),
        }
    }
}

fn parse_with_graph_recursive<P: AsRef<Path>>(
    context: &Context,
    path: &P,
    mut graph: &mut Graph<JsModule, usize>,
) -> Result<NodeIndex, Error> {
    let module = parse_js_module(&path)?;
    let mut visitor = DependencyVisitor;
    let dependencies = module.program.accept(&mut visitor);

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
