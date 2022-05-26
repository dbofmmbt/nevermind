use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::Context;

pub mod map_loaders;
pub mod map_views;
pub mod mind_map;

pub fn save_as_image<T: AsRef<Path>>(path: T, dot: &str) -> anyhow::Result<()> {
    let handle = Command::new("dot")
        .arg("-Tpng")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("failed to start dot command. Is graphviz installed?")?;

    handle
        .stdin
        .context("trying to get child stdin")?
        .write_all(dot.as_bytes())?;

    let mut stdout = handle.stdout.context("getting stdout")?;
    let mut out_file = std::fs::File::create(path).context("create the image file")?;

    std::io::copy(&mut stdout, &mut out_file).context("write image to file")?;

    Ok(())
}
