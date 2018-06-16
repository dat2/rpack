use context::Context;
use easter::decl::{ConstDtor, Decl, Import};
use easter::expr::{Expr, ExprListItem};
use easter::id::Id;
use easter::stmt::{ModItem, StmtListItem};
use esprit;
use failure::Error;
use modules::Module;
use std::path::{Path, PathBuf};

pub fn parse_javascript_module<P: AsRef<Path>>(
    context: &Context,
    input_path: &P,
) -> Result<(Module, Vec<PathBuf>), Error> {
    let mut dependencies = vec![];

    let module = Module::from_path(input_path)?;

    let program = esprit::module(&module.content)?;
    for item in program.items {
        match item {
            ModItem::Import(Import::ForEffect(_, path)) => {
                dependencies.push(context.resolve(&module, &path.value)?);
            }
            // parse require with relative path
            ModItem::StmtListItem(StmtListItem::Decl(Decl::Const(_, dtors, _))) => {
                if let [ConstDtor { ref value, .. }] = dtors[..] {
                    if let Expr::Call(_, box Expr::Id(Id { ref name, .. }), args) = value {
                        if name.as_ref() == "require" {
                            if let [ExprListItem::Expr(Expr::String(_, ref string_lit))] = args[..]
                            {
                                let mut path_buf = PathBuf::from(&string_lit.value);
                                path_buf.set_extension("js");
                                dependencies.push(context.resolve(&module, &path_buf)?);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok((module, dependencies))
}
