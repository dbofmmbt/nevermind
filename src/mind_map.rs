#[derive(Debug)]
pub struct MindMap {
    pub content: String,
    pub children: Vec<MindMap>,
}

impl MindMap {
    pub fn leaf(content: String) -> Self {
        Self {
            content,
            children: Default::default(),
        }
    }
}
