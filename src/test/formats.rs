use anyhow::Result;
use camino::Utf8Path as Path;

use super::Test;

mod cram;
mod crambly;

pub(crate) trait Read {
    fn read(path: &Path) -> Result<Test>;
}

pub(crate) trait Render {
    fn render(test: &Test) -> Result<String>;
}

pub(crate) struct Crambly;
pub(crate) struct Cram;

impl Read for Crambly {
    fn read(path: &Path) -> Result<Test> {
        crambly::read(path)
    }
}

impl Read for Cram {
    fn read(path: &Path) -> Result<Test> {
        cram::read(path)
    }
}

impl Render for Crambly {
    fn render(test: &Test) -> Result<String> {
        crambly::render(test)
    }
}

impl Render for Cram {
    fn render(test: &Test) -> Result<String> {
        Ok(cram::render(test))
    }
}
