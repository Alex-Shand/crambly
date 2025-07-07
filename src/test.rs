use anyhow::Result;
use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};
use formats::{Read, Render};

use self::tc::TestCase;

mod command;
pub(crate) mod formats;
mod tc;

#[derive(Debug, PartialEq)]
pub(crate) struct Test {
    path: PathBuf,
    cases: Vec<TestCase>,
}

impl Test {
    pub(crate) fn read<Format: Read>(path: &Path) -> Result<Self> {
        Format::read(path)
    }

    pub(crate) fn render<Format: Render>(&self) -> Result<String> {
        Format::render(self)
    }

    pub(crate) fn err_path(&self) -> PathBuf {
        self.path.with_extension("err")
    }
}
