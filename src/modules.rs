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
}
