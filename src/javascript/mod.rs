pub mod ast;
mod parser;
mod display;

use self::ast::*;
use failure::{self, Error};
use io_utils;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct JsModule {
    pub source: String,
    pub program: Program,
    pub path: PathBuf,
}

pub fn parse_js_module<P: AsRef<Path>>(path: &P) -> Result<JsModule, Error> {
    let (absolute_path, source) = io_utils::read_file(&path)?;
    match self::parser::parse(&source) {
        Ok(program) => Ok(JsModule {
            source,
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
