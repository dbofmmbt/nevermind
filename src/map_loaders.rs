mod yaml;
pub use yaml::YamlMapLoader;

use crate::mind_map::MindMap;

pub trait MapLoader {
    fn load(&mut self, input: &str) -> anyhow::Result<MindMap>;
}
