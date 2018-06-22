pub mod ast;
mod parser;

use self::ast::*;
use failure::{self, Error};
use io_utils;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct JsModule {
    program: Program,
    pub path: PathBuf,
}

impl JsModule {
    pub fn get_dependencies(&self) -> Vec<PathBuf> {
        let mut result = Vec::new();
        for statement in &self.program.statements {
            match statement {
                Statement::Import(_, path) => {
                    result.push(PathBuf::from(path));
                }
            }
        }
        result
    }
}

pub fn parse_js_module<P: AsRef<Path>>(path: &P) -> Result<JsModule, Error> {
    let (absolute_path, contents) = io_utils::read_file(&path)?;
    match self::parser::parse(&contents) {
        Ok(program) => Ok(JsModule {
            program,
            path: absolute_path,
        }),
        Err(e) => Err(failure::err_msg(format!(
            "Error in {}\n{}",
            path.as_ref().display(),
            e
        ))),
    }
}
