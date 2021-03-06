use failure::{self, Error};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Context {
    resolve_paths: Vec<PathBuf>,
}

impl Context {
    pub fn new(resolve_path: &str) -> Result<Context, Error> {
        let mut resolve_paths = Vec::new();
        for path in resolve_path.split(':') {
            resolve_paths.push(fs::canonicalize(path)?);
        }
        Ok(Context { resolve_paths })
    }

    pub fn resolve<P: AsRef<Path>, P2: AsRef<Path>>(
        &self,
        module_path: &P,
        path: P2,
    ) -> Result<PathBuf, Error> {
        let p = path.as_ref();
        if p.starts_with(".") {
            self.resolve_relative(module_path, p)
        } else {
            self.resolve_absolute(p)
        }
    }

    fn resolve_relative<P: AsRef<Path>, P2: AsRef<Path>>(
        &self,
        module_path: &P,
        path: P2,
    ) -> Result<PathBuf, Error> {
        let mut module_path_buf = module_path.as_ref().to_owned();
        module_path_buf.pop();
        module_path_buf.push(path);
        if module_path_buf.is_dir() {
            module_path_buf.push("index.js");
        } else if module_path_buf.extension().is_none() {
            module_path_buf.set_extension("js");
        }
        let result = fs::canonicalize(module_path_buf)?;
        Ok(result.to_owned())
    }

    fn resolve_absolute<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, Error> {
        let p = path.as_ref();
        self.resolve_paths
            .iter()
            .map(|path_buf| {
                let mut mutable_path_buf = path_buf.clone();
                mutable_path_buf.push(&p);
                if mutable_path_buf.is_dir() {
                    mutable_path_buf.push("index.js");
                }
                mutable_path_buf
            })
            .find(|path| path.exists())
            .ok_or_else(|| failure::err_msg(format!("Could not find '{}'", p.display())))
    }
}
