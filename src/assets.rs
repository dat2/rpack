use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum AssetType {
    JavaScriptChunk,
    Html,
    CssChunk,
}

#[derive(Debug)]
pub struct Asset {
    pub output_path: PathBuf,
    pub content: String,
    pub atype: AssetType,
}

impl Asset {
    pub fn new<P: AsRef<Path>>(path: &P, atype: AssetType) -> Asset {
        Asset {
            output_path: path.as_ref().to_owned(),
            content: String::new(),
            atype: atype,
        }
    }

    pub fn push_str(&mut self, s: &str) {
        self.content.push_str(s);
    }

    pub fn write(&self) -> Result<(), Error> {
        let mut file = File::create(&self.output_path)?;
        file.write_all(self.content.as_bytes())?;
        Ok(())
    }
}
