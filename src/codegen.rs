use failure::Error;
use hex;
use javascript::ast::*;
use javascript::JsModule;
use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Graph;
use ring::digest;
use std::ops::Index;

fn map_statements(statement: &Statement) -> FunctionBodyStatement {
    // TODO match path to module id
    FunctionBodyStatement::Statement(match statement {
        Statement::Import(ImportSpecifier::ImportDefault(id), path) => {
            build_ast! {
                var [pat_id id.clone()] = [
                    expr_call
                        [expr_id "_rpack_require".to_string()]
                        [[expr_str path.clone()]]
                ]
            }
        }
        other => other.clone(),
    })
}

pub fn codegen(graph: &Graph<JsModule, usize>, entry_point_id: NodeIndex) -> Result<String, Error> {
    // collect all js files into 1 big asset
    let mut result_ast = Program {
        source_type: SourceType::Script,
        body: Vec::new(),
    };

    let mut properties = Vec::new();
    let mut dfs = Dfs::new(&graph, entry_point_id);
    while let Some(node_index) = dfs.next(&graph) {
        let node = graph.index(node_index);

        // TODO add comment explaining which file this came from
        let property = build_ast! {
            // '<module_id>': function(module, exports, _rpack_require) { body }
            prop
                [prop_str_key generate_module_id(&node.source)]
                [expr_func
                    [
                        [pat_id "module".to_string()],
                        [pat_id "exports".to_string()],
                        [pat_id "_rpack_require".to_string()]
                    ]
                    {
                        node.program.body.iter().map(map_statements).collect()
                    }
                ]
        };
        properties.push(property);
    }

    let object_statement = build_ast! {
        var [pat_id "modules".to_string()] = [expr_obj {properties}]
    };

    result_ast.body.push(object_statement);
    Ok(result_ast.to_string())
}

pub fn generate_module_id(source: &str) -> String {
    let bytes = digest::digest(&digest::SHA512, source.as_bytes());
    let mut hex_encoded = hex::encode(bytes);
    hex_encoded.truncate(4);
    hex_encoded
}
