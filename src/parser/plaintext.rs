use failure::Error;
use modules::Module;
use std::path::{Path, PathBuf};

pub fn parse_plaintext_module<P: AsRef<Path>>(
    input_path: &P,
) -> Result<(Module, Vec<PathBuf>), Error> {
    let module = Module::from_path(input_path)?;
    Ok((module, Vec::new()))
}
