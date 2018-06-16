use failure::Error;
use hex;
use ring::digest;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ModuleType {
    JavaScript,
    Html,
    Css,
    PlainText,
}

impl ModuleType {
    pub fn parse_from_path<P: AsRef<Path>>(path: &P) -> ModuleType {
        match path.as_ref().extension() {
            None => ModuleType::PlainText,
            Some(os_str) => match os_str.to_str() {
                Some("html") => ModuleType::Html,
                Some("js") => ModuleType::JavaScript,
                Some("css") => ModuleType::Css,
                _ => ModuleType::PlainText,
            },
        }
    }
}

#[derive(Debug)]
pub struct Module {
    pub path: PathBuf,
    pub content: String,
    pub mtype: ModuleType,
}

impl Module {
    pub fn new<P: AsRef<Path>>(path: &P, content: String) -> Module {
        Module {
            path: path.as_ref().to_owned(),
            content: content,
            mtype: ModuleType::parse_from_path(path),
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
