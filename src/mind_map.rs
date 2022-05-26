#[derive(Debug)]
pub struct MindMap {
    pub content: String,
    pub children: Vec<MindMap>,
}
