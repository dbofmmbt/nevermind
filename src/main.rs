use std::path::PathBuf;

use anyhow::Context;
use nevermind::{
    map_loaders::{MapLoader, YamlMapLoader},
    map_views::DotView,
};

use clap::Parser;

#[derive(Debug, Parser)]
/// Mind Maps as code. Generate Mind Map images based on yaml files.
struct Input {
    /// Path to the mind map
    pub file: PathBuf,
    pub out: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let input = Input::parse();
    let input_file = std::fs::read_to_string(&input.file).context("reading file contents")?;

    let mind_map = YamlMapLoader
        .load(input_file.as_str())
        .context("converting input to mind map")?;
    let dot = DotView.to_dot(&mind_map);

    let out_path = input
        .out
        .unwrap_or_else(|| input.file.with_extension("png"));

    nevermind::save_as_image(out_path, &dot).context("convert dot to image")?;

    Ok(())
}
