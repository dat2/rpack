use failure::Error;
use std::fs;
use std::path::{Path, PathBuf};

use modules::Module;

pub struct Context;

impl Context {
    pub fn resolve<P: AsRef<Path>>(&self, module: &Module, path: P) -> Result<PathBuf, Error> {
        self.resolve_relative(module, path)
    }

    fn resolve_relative<P: AsRef<Path>>(
        &self,
        module: &Module,
        path: P,
    ) -> Result<PathBuf, Error> {
        let mut module_path_buf = module.path.clone();
        module_path_buf.pop();
        module_path_buf.push(path);
        let result = fs::canonicalize(module_path_buf)?;
        Ok(result.to_owned())
    }
}
