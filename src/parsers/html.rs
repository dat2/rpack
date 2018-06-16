use failure::Error;
use modules::Module;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use victoria_dom::DOM;

pub fn parse_html_module<P: AsRef<Path>>(input_path: &P) -> Result<(Module, Vec<PathBuf>), Error> {
    let mut dependencies = vec![];

    let absolute_path = fs::canonicalize(input_path)?;
    let mut file = File::open(&absolute_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let module = Module::new(&absolute_path, contents);

    let mut base_path = module.path.clone();
    base_path.pop();
    let dom = DOM::new(&module.content);

    let stylesheets = dom.find(r#"link[rel="stylesheet"][href]"#);
    for stylesheet in &stylesheets {
        let mut href_path = base_path.clone();
        let href = stylesheet.attr("href").unwrap().to_owned();
        href_path.push(href);
        dependencies.push(fs::canonicalize(href_path)?);
    }

    let scripts = dom.find(r#"script[type="text/javascript"][src]"#);
    for script in &scripts {
        let mut src_path = base_path.clone();
        let src = script.attr("src").unwrap().to_owned();
        src_path.push(src);
        dependencies.push(fs::canonicalize(src_path)?);
    }

    Ok((module, dependencies))
}
