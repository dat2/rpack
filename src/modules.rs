use failure::Error;
use hex;
use ring::digest;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Module {
    pub path: PathBuf,
    pub content: String,
}

impl Module {
    pub fn new<P: AsRef<Path>>(path: &P, content: String) -> Module {
        Module {
            path: path.as_ref().to_owned(),
            content: content,
        }
    }

    pub fn resolve_relative<P: AsRef<Path>>(&self, dependency_path: P) -> Result<PathBuf, Error> {
        let mut module_path_buf = self.path.clone();
        module_path_buf.pop();
        module_path_buf.push(dependency_path);
        let result = fs::canonicalize(module_path_buf)?;
        Ok(result.to_owned())
    }

    pub fn id(&self) -> String {
        let bytes = digest::digest(&digest::SHA512, self.content.as_bytes());
        let mut hex_encoded = hex::encode(bytes);
        hex_encoded.truncate(4);
        hex_encoded
    }
}
