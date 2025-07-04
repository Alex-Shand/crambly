use std::fs;

use anyhow::{Result, anyhow};
use camino::Utf8Path as Path;

pub(crate) fn walk(
    dir: impl AsRef<Path>,
    mut f: impl FnMut(&Path) -> Result<()>,
) -> Result<()> {
    fn aux(dir: &Path, f: &mut dyn FnMut(&Path) -> Result<()>) -> Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let path = entry?.path();
                let path = Path::from_path(&path)
                    .ok_or_else(|| anyhow!("Non-Utf8 path: {:?}", path))?;
                if path.is_dir() {
                    aux(path, f)?;
                } else {
                    f(path)?;
                }
            }
        }
        Ok(())
    }
    aux(dir.as_ref(), &mut f)
}
