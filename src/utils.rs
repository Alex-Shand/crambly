use std::fs;

use anyhow::{Context as _, Result, anyhow};
use camino::Utf8Path as Path;

pub(crate) fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| anyhow!("Failed to read `{path}`"))
}
