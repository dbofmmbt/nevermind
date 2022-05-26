use anyhow::Context;
use yaml_rust::{yaml, YamlLoader};

use crate::mind_map::MindMap;

use super::MapLoader;

pub struct YamlMapLoader;

impl MapLoader for YamlMapLoader {
    fn load(&mut self, input: &str) -> anyhow::Result<MindMap> {
        let yaml = YamlLoader::load_from_str(input)?
            .into_iter()
            .next()
            .context("yaml missing")?;

        let hash = yaml.into_hash().context("not a mind map")?;
        let mind_map = mind_map_from_yaml(hash)?
            .into_iter()
            .next()
            .context("map missing")?;

        Ok(mind_map)
    }
}

fn mind_map_from_yaml(hash: yaml::Hash) -> anyhow::Result<Vec<MindMap>> {
    hash.into_iter()
        .map(|(key, value)| {
            let content = key.as_str().context("map content is not a string")?.into();

            let children = value
                .into_hash()
                .map(mind_map_from_yaml)
                .unwrap_or_else(|| Ok(vec![]))?;

            Ok(MindMap { content, children })
        })
        .collect::<anyhow::Result<Vec<_>>>()
}
