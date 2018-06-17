use assets::{Asset, AssetType};
use failure::Error;
use modules::{ImportType, Module, ModuleType};
use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Graph;
use petgraph::Incoming;
use std::ops::Index;

pub fn codegen(
    mut graph: Graph<Module, ImportType>,
    entry_point_id: NodeIndex,
) -> Result<(), Error> {
    let mut main_js_asset = Asset::new(&"build/example.js".to_owned(), AssetType::JavaScriptChunk);

    let mut dfs = Dfs::new(&graph, entry_point_id);
    let mut strings = Vec::new();
    while let Some(node_index) = dfs.next(&graph) {
        let node = graph.index(node_index);
        if node.mtype == ModuleType::JavaScript {
            strings.push(format!(
                "  {id}: function (module, exports, require) {{\n// {filepath} \n{content}\n  }}",
                filepath = node.path.display(),
                id = node.id(),
                content = node.content
            ));
        }
    }

    main_js_asset.push_str(&format!("{{\n{}\n}}\n", strings.join(",\n")));
    main_js_asset.write()?;

    Ok(())
}
