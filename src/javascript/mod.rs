pub mod ast;
mod parser;

use self::ast::*;
use failure::{self, Error};
use io_utils;
use std::path::{Path, PathBuf};

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
                    let mut path_buf = PathBuf::from(path);
                    if path_buf.extension().is_none() {
                        path_buf.set_extension("js");
                    }
                    result.push(path_buf);
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
            program: program,
            path: absolute_path,
        }),
        Err(e) => Err(failure::err_msg(format!(
            "Error in {}\n{}",
            path.as_ref().display(),
            e
        ))),
    }
}
