// some conventions:
// [] means recursively call the build_ast macro again
// {} means take a rust expression
// when we need to recursively call the macro for like a call expr,
// we do this: [ [arg1] [arg2] [arg3] ]

#[macro_export]
macro_rules! build_ast {
    ([$($many:tt)+]) => {
        build_ast!($($many)+)
    };
    (var) => {
        VariableDeclarationKind::Var
    };
    (let) => {
        VariableDeclarationKind::Let
    };
    (const) => {
        VariableDeclarationKind::Const
    };
    (pat_id $id:expr) => {
        Pattern::Id {
            id: $id
        }
    };
    (expr_id $id:expr) => {
        Expression::Id {
            id: $id
        }
    };
    (expr_str $lit:expr) => {
        Expression::Literal {
            literal: Literal::StringLiteral($lit)
        }
    };
    (expr_call [$($id:tt)+] [$($args:tt)+]) => {
        Expression::Call {
            callee: Box::new(build_ast!($($id)+)),
            arguments: vec![$(build_ast!($args)),+]
        }
    };
    (expr_func [$($params:tt),+] {$body:expr}) => {
        Expression::Function {
            function: Function {
                id: None,
                params: vec![$(build_ast!($params)),+],
                body: $body,
                generator: false
            }
        }
    };
    (expr_obj {$properties:expr}) => {
        Expression::Object {
            properties: $properties
        }
    };
    (prop_str_key $key:expr) => {
        PropertyKey::Literal(Literal::StringLiteral($key))
    };
    (prop [$($key:tt)+] [$($value:tt)+]) => {
        Property {
            key: build_ast!($($key)+),
            value: build_ast!($($value)+),
            kind: PropertyKind::Init,
        }
    };
    ($var:tt [$($id:tt)+]) => {
        Statement::VariableDeclaration {
            declaration: VariableDeclaration {
                kind: build_ast!($var),
                declarations: vec![VariableDeclarator {
                    id: build_ast!($($id)+),
                    init: None,
                }],
            },
        }
    };
    // [] means build another ast
    ($var:tt [$($id:tt)+] = [$($tail:tt)+]) => {
        Statement::VariableDeclaration {
            declaration: VariableDeclaration {
                kind: build_ast!($var),
                declarations: vec![VariableDeclarator {
                    id: build_ast!($($id)+),
                    init: Some(build_ast!($($tail)+)),
                }],
            },
        }
    };
    // { } means defer to rust expression
    ($var:tt [$($id:tt)+] = {$($init:tt)+}) => {
        Statement::VariableDeclaration {
            declaration: VariableDeclaration {
                kind: build_ast!($var),
                declarations: vec![VariableDeclarator {
                    id: build_ast!($($id)+),
                    init: Some($($init)+)
                }],
            },
        }
    };
}

#[macro_export]
macro_rules! match_ast {
    (import [$($id:tt)+] from [$($path:tt)+]) => {
        Statement::Import(ImportSpecifier::ImportDefault($($id)+), $($path)+)
    }
}
