use std::collections::HashMap;

use anyhow::Result;
use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};

use super::Test;

mod cram;
mod crambly;

pub(crate) trait Read {
    fn read(path: &Path) -> Result<Test>;
}

pub(crate) trait Render {
    type Aux;
    fn render(test: &Test, aux: Self::Aux) -> Result<String>;
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
    type Aux = ();
    fn render(test: &Test, (): ()) -> Result<String> {
        crambly::render(test)
    }
}

impl Render for Cram {
    type Aux = HashMap<String, PathBuf>;
    fn render(test: &Test, aux: Self::Aux) -> Result<String> {
        Ok(cram::render(test, &aux))
    }
}
