use failure::Error;
use modules::Module;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn parse_unknown_module<P: AsRef<Path>>(input_path: &P) -> Result<(Module, Vec<PathBuf>), Error> {
    let absolute_path = fs::canonicalize(input_path)?;
    let mut file = File::open(&absolute_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let module = Module::new(&absolute_path, contents);
    Ok((module, Vec::new()))
}
