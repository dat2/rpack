// ast printer
use javascript::ast::*;
use std::fmt;

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.body
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(";")
        )
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::VariableDeclaration { declaration } => write!(f, "{}", declaration),
            Statement::Expression { expression } => write!(f, "{}", expression),
            _ => write!(f, ""),
        }
    }
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.kind,
            self.declarations
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl fmt::Display for VariableDeclarationKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariableDeclarationKind::Var => write!(f, "var"),
            VariableDeclarationKind::Let => write!(f, "let"),
            VariableDeclarationKind::Const => write!(f, "const"),
        }
    }
}

impl fmt::Display for VariableDeclarator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.init {
            Some(ref e) => write!(f, "{} = {}", self.id, e),
            None => write!(f, "{}", self.id),
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pattern::Id { id } => write!(f, "{}", id),
            _ => write!(f, ""),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::IdReference { id } => write!(f, "{}", id),
            Expression::Literal { literal } => write!(f, "{}", literal),
            Expression::Call { callee, arguments } => write!(
                f,
                "{}({})",
                callee,
                arguments
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Expression::ObjectLiteral { properties } => write!(
                f,
                "{{{}}}",
                properties
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Expression::Function {
                id,
                ref generator,
                params,
                body,
                ref async,
            } => write!(
                f,
                "{}function{}{}({}){{{}}}",
                if *async { "async " } else { "" },
                id.clone().unwrap_or_default(),
                if *generator { "*" } else { "" },
                params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                body.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(";"),
            ),
            _ => write!(f, ""),
        }
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}:{}", self.kind, self.key, self.value)
    }
}

impl fmt::Display for PropertyKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PropertyKind::Init => write!(f, ""),
            PropertyKind::Get => write!(f, "get "),
            PropertyKind::Set => write!(f, "set "),
        }
    }
}

impl fmt::Display for NullLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "null")
    }
}

impl fmt::Display for RegexLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}/{}", self.pattern, self.flags)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::StringLiteral(s) => write!(f, "'{}'", s),
            Literal::BooleanLiteral(b) => write!(f, "{}", b),
            Literal::NullLiteral(n) => write!(f, "{}", n),
            Literal::NumberLiteral(n) => write!(f, "{}", n),
        }
    }
}
