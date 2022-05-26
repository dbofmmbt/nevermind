use std::{env::Args, path::PathBuf};

use anyhow::{anyhow, Context};
use nevermind::{
    map_loaders::{MapLoader, YamlMapLoader},
    map_views::DotView,
};

#[derive(Debug)]
struct Input {
    pub file: PathBuf,
}

impl TryFrom<Args> for Input {
    type Error = anyhow::Error;

    fn try_from(mut args: Args) -> Result<Self, Self::Error> {
        Ok(Self {
            file: args
                .nth(1)
                .ok_or_else(|| anyhow!("path is missing!"))?
                .into(),
        })
    }
}

fn main() -> anyhow::Result<()> {
    let input = Input::try_from(std::env::args()).context("getting input from CLI")?;
    let input = std::fs::read_to_string(input.file).context("reading file contents")?;
    let mind_map = YamlMapLoader
        .load(input.as_str())
        .context("converting input to mind map")?;
    println!("{}", DotView.to_dot(&mind_map));
    Ok(())
}
