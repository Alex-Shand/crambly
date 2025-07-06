use std::{env, process};

use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};

use crate::Result;

#[derive(Debug)]
pub(crate) struct Context {
    pub(crate) exe: PathBuf,
    pub(crate) test_dir: PathBuf,
    pub(crate) tmp_dir: PathBuf,
}

impl Context {
    pub(crate) fn new(
        exe: &str,
        test_dir: impl AsRef<Path>,
        tmp_dir: impl AsRef<Path>,
    ) -> Result<Self> {
        let test_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
            .join("tests")
            .join(test_dir);
        Ok(Self {
            exe: PathBuf::from(exe),
            test_dir,
            tmp_dir: tmp_dir.as_ref().join(process::id().to_string()),
        })
    }
}
