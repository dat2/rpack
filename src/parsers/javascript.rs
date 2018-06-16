use easter::decl::{ConstDtor, Decl, Import};
use easter::expr::{Expr, ExprListItem};
use easter::id::Id;
use easter::stmt::{ModItem, StmtListItem};
use esprit;
use failure::Error;
use modules::Module;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn parse_javascript_module<P: AsRef<Path>>(
    input_path: &P,
) -> Result<(Module, Vec<PathBuf>), Error> {
    let mut dependencies = vec![];

    let absolute_path = fs::canonicalize(input_path)?;
    let mut file = File::open(&absolute_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let module = Module::new(&absolute_path, contents);

    let program = esprit::module(&module.content)?;
    for item in program.items {
        match item {
            ModItem::Import(Import::ForEffect(_, path)) => {
                dependencies.push(module.resolve_relative(&path.value)?);
            }
            ModItem::StmtListItem(StmtListItem::Decl(Decl::Const(_, dtors, _))) => {
                if let [ConstDtor { ref value, .. }] = dtors[..] {
                    if let Expr::Call(_, box Expr::Id(Id { ref name, .. }), args) = value {
                        if name.as_ref() == "require" {
                            if let [ExprListItem::Expr(Expr::String(_, ref string_lit))] = args[..]
                            {
                                let mut path_buf = PathBuf::from(&string_lit.value);
                                path_buf.set_extension("js");
                                dependencies.push(module.resolve_relative(&path_buf)?);
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
