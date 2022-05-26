use crate::mind_map::MindMap;

pub struct DotView;

impl DotView {
    pub fn to_dot(&self, mind_map: &MindMap) -> String {
        let mut nodes = String::new();
        chain_nodes(&mut nodes, mind_map);

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

// TODO remove this gambiarra
static mut CLUSTER_NUMBER: usize = 0;

fn with_cluster(output: &mut String, f: impl FnOnce(&mut String, usize)) {
    let cluster_number;

    unsafe {
        cluster_number = CLUSTER_NUMBER;
        CLUSTER_NUMBER += 1;
    }

    output.push_str(&format!(
        "subgraph cluster_{cluster_number} {{ color=black;style=\"rounded\";"
    ));

    f(output, cluster_number);

    output.push('}');
}

fn chain_nodes(output: &mut String, mind_map: &MindMap) {
    let content = &mind_map.content;

    with_cluster(output, |output, _| {
        output.push_str(&format!("\"{content}\";"));
    });

    with_cluster(output, |output, number| {
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
        chain_nodes(output, map);
    }
}
