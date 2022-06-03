use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::mind_map::MindMap;

pub struct DotView;

impl DotView {
    pub fn to_dot(&self, mind_map: &MindMap) -> String {
        let mut nodes = String::new();
        chain_nodes(&mut nodes, 0, mind_map);

        format!(
            r#"
digraph {{
compound = true;
overlap = "compress";
rankdir = "LR"
splines = "true";
layout = "dot";
style="rounded";
node [
    shape = "none",
];
edge [
    arrowhead = "none"
]
{nodes}
}}    
"#
        )
    }
}

fn with_cluster(output: &mut String, label: u64, f: impl FnOnce(&mut String)) {
    output.push_str(&format!(
        "subgraph cluster_{label} {{ color=black;style=\"rounded\";"
    ));

    f(output);

    output.push('}');
}

fn chain_nodes(output: &mut String, label: u64, mind_map: &MindMap) {
    let content = &mind_map.content;
    
    with_cluster(output, label, |output| {
        output.push_str(&format!("\"{content}\";"));
    });
    
    let parent_label = hash_from(&content);
    
    with_cluster(output, parent_label, |output| {
        if let Some(child) = mind_map.children.first() {
            let child = &child.content;
            output.push_str(&format!(
                "\"{content}\" -> \"{child}\" [lhead=\"cluster_{parent_label}\"];"
            ));
        }
        for map in mind_map.children.iter() {
            let child = map.content.as_str();
            output.push_str(&format!("\"{child}\";"));
        }
    });

    for map in mind_map.children.iter() {
        chain_nodes(output, parent_label, map);
    }
}

fn hash_from<T: Hash>(t: T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
