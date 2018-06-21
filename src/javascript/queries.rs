use context::Context;
use javascript::JsModule;
use std::path::PathBuf;

pub fn get_dependencies(_context: &Context, _module: &JsModule) -> Vec<PathBuf> {
    Vec::new()
}
