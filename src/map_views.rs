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

fn with_cluster(output: &mut String, level: usize, f: impl FnOnce(&mut String, usize)) {
    output.push_str(&format!(
        "subgraph cluster_{level} {{ color=black;style=\"rounded\";"
    ));

    f(output, level);

    output.push('}');
}

fn chain_nodes(output: &mut String, level: usize, mind_map: &MindMap) {
    let content = &mind_map.content;

    with_cluster(output, level, |output, _| {
        output.push_str(&format!("\"{content}\";"));
    });

    with_cluster(output, level + 1, |output, number| {
        if let Some(child) = mind_map.children.first() {
            let child = &child.content;
            output.push_str(&format!(
                "\"{content}\" -> \"{child}\" [lhead=\"cluster_{number}\"];"
            ));
        }
        for map in mind_map.children.iter() {
            let child = map.content.as_str();
            output.push_str(&format!("\"{child}\";"));
        }
    });

    for map in mind_map.children.iter() {
        chain_nodes(output, level + 1, map);
    }
}
