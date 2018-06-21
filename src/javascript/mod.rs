pub mod ast;
mod parser;
pub mod queries;

use self::ast::Program;
use failure::Error;
use io_utils;
use std::path::{Path, PathBuf};

pub struct JsModule {
    program: Program,
    path: PathBuf,
}

pub fn parse_js_module<P: AsRef<Path>>(path: &P) -> Result<JsModule, Error> {
    let (absolute_path, contents) = io_utils::read_file(&path)?;
    let program = self::parser::parse(&contents)?;
    Ok(JsModule {
        program: program,
        path: absolute_path,
    })
}
