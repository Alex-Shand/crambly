use anyhow::Result;
use camino::Utf8Path as Path;

pub(crate) fn walk(
    dir: impl AsRef<Path>,
    mut f: impl FnMut(&Path) -> Result<()>,
) -> Result<()> {
    fn aux(dir: &Path, f: &mut dyn FnMut(&Path) -> Result<()>) -> Result<()> {
        if dir.is_dir() {
            for entry in dir.read_dir_utf8()? {
                let entry = entry?;
                let path = entry.path();
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
