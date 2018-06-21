use failure::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn read_file<P: AsRef<Path>>(path: &P) -> Result<(PathBuf, String), Error> {
    let absolute_path = fs::canonicalize(path)?;
    let mut file = File::open(&absolute_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok((absolute_path, contents))
}
