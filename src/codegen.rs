use failure::Error;
use hex;
use javascript::ast::*;
use javascript::JsModule;
use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Graph;
use ring::digest;
use std::ops::Index;

struct Minifier;

impl Visitor for Minifier {
    type Result = String;

    fn visit_program(&mut self, program: &Program) -> Self::Result {
        let mut result = String::new();
        for statement in &program.body {
            let statement_result = self.visit_statement(statement);
            result.push_str(&statement_result);
        }
        result
    }

    fn visit_statement(&mut self, _statement: &Statement) -> Self::Result {
        String::new()
    }
}

pub fn codegen(graph: &Graph<JsModule, usize>, entry_point_id: NodeIndex) -> Result<(), Error> {
    // collect all js files into 1 big asset
    let mut result_ast = Program {
        source_type: SourceType::Module,
        body: Vec::new(),
    };

    let mut properties = Vec::new();
    let mut dfs = Dfs::new(&graph, entry_point_id);
    while let Some(node_index) = dfs.next(&graph) {
        let node = graph.index(node_index);

        // TODO add comment explaining which file this came from
        let function_expression = Expression::Function {
            function: Function {
                id: None,
                params: vec![
                    Pattern::Id {
                        id: "module".to_string(),
                    },
                    Pattern::Id {
                        id: "exports".to_string(),
                    },
                    Pattern::Id {
                        id: "require".to_string(),
                    },
                ],
                body: node
                    .program
                    .body
                    .iter()
                    .map(|s| FunctionBodyStatement::Statement(s.clone()))
                    .collect(),
                generator: false,
            },
        };
        let generated_module_id = generate_module_id(&node.source);
        let property = Property {
            key: PropertyKey::Id(generated_module_id),
            value: function_expression,
            kind: PropertyKind::Init,
        };
        properties.push(property);
    }

    let mut object_expression = Expression::Object { properties };
    println!("{:?}", object_expression);

    Ok(())
}

pub fn generate_module_id(source: &str) -> String {
    let bytes = digest::digest(&digest::SHA512, source.as_bytes());
    let mut hex_encoded = hex::encode(bytes);
    hex_encoded.truncate(4);
    hex_encoded
}
