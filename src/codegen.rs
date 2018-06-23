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
    match statement {
        Statement::Import(ImportSpecifier::ImportDefault(id), path) => {
            FunctionBodyStatement::Statement(Statement::VariableDeclaration {
                declaration: VariableDeclaration {
                    kind: VariableDeclarationKind::Var,
                    declarations: vec![VariableDeclarator {
                        id: Pattern::Id { id: id.clone() },
                        init: Some(Expression::Call {
                            callee: Box::new(Expression::Id {
                                id: "require".to_string(),
                            }),
                            arguments: vec![Expression::Literal {
                                literal: Literal::StringLiteral(path.clone()),
                            }],
                        }),
                    }],
                },
            })
        }
        other => FunctionBodyStatement::Statement(other.clone()),
    }
}

pub fn codegen(graph: &Graph<JsModule, usize>, entry_point_id: NodeIndex) -> Result<(), Error> {
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
                body: node.program.body.iter().map(map_statements).collect(),
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

    let object_statement = Statement::VariableDeclaration {
        declaration: VariableDeclaration {
            kind: VariableDeclarationKind::Var,
            declarations: vec![VariableDeclarator {
                id: Pattern::Id {
                    id: "modules".to_string(),
                },
                init: Some(Expression::Object { properties }),
            }],
        },
    };

    result_ast.body.push(object_statement);
    println!("{:?}", result_ast);

    Ok(())
}

pub fn generate_module_id(source: &str) -> String {
    let bytes = digest::digest(&digest::SHA512, source.as_bytes());
    let mut hex_encoded = hex::encode(bytes);
    hex_encoded.truncate(4);
    hex_encoded
}
