use failure::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Context;

impl Context {
    pub fn new() -> Context {
        Context
    }

    pub fn resolve<P: AsRef<Path>, P2: AsRef<Path>>(
        &self,
        module_path: &P,
        path: P2,
    ) -> Result<PathBuf, Error> {
        self.resolve_relative(module_path, path)
    }

    fn resolve_relative<P: AsRef<Path>, P2: AsRef<Path>>(
        &self,
        module_path: &P,
        path: P2,
    ) -> Result<PathBuf, Error> {
        let mut module_path_buf = module_path.as_ref().to_owned();
        module_path_buf.pop();
        module_path_buf.push(path);
        let result = fs::canonicalize(module_path_buf)?;
        Ok(result.to_owned())
    }
}
