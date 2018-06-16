use context::Context;
use failure::Error;
use modules::Module;
use std::path::{Path, PathBuf};
use victoria_dom::DOM;

pub fn parse_html_module<P: AsRef<Path>>(context: &Context, input_path: &P) -> Result<(Module, Vec<PathBuf>), Error> {
    let mut dependencies = vec![];

    let module = Module::from_path(input_path)?;

    let dom = DOM::new(&module.content);

    let stylesheets = dom.find(r#"link[rel="stylesheet"][href]"#);
    for stylesheet in &stylesheets {
        let href = stylesheet.attr("href").unwrap().to_owned();
        dependencies.push(context.resolve(&module, &href)?);
    }

    let scripts = dom.find(r#"script[type="text/javascript"][src]"#);
    for script in &scripts {
        let src = script.attr("src").unwrap().to_owned();
        dependencies.push(context.resolve(&module, &src)?);
    }

    Ok((module, dependencies))
}
