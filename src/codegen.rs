use failure::Error;
use javascript::JsModule;
use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Graph;
use std::ops::Index;

pub fn codegen(graph: &Graph<JsModule, usize>, entry_point_id: NodeIndex) -> Result<(), Error> {
    // collect all js files into 1 big asset
    // let mut main_js_asset = Asset::new(&"build/example.js".to_owned(), AssetType::JavaScriptChunk);
    // eg. { 1234: function(m,e,r) {  }, ab14: function(m,e,r) {} }

    let mut dfs = Dfs::new(&graph, entry_point_id);
    // let mut strings = Vec::new();
    while let Some(node_index) = dfs.next(&graph) {
        let node = graph.index(node_index);
        println!("{:?}", node);
        /*
        if node.mtype == ModuleType::JavaScript {
            strings.push(format!(
                "  {id}: function (module, exports, require) {{\n// {filepath} \n{content}\n  }}",
                filepath = node.path.display(),
                id = node.id(),
                content = node.content
            ));
        }*/
    }

    // main_js_asset.push_str(&format!("{{\n{}\n}}\n", strings.join(",\n")));
    // main_js_asset.write()?;

    Ok(())
}
