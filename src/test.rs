use anyhow::Result;
use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};

use self::{
    formats::{Read, Render},
    input::Input,
    tc::TestCase,
};

mod command;
pub(crate) mod formats;
pub(crate) mod input;
mod output;
mod tc;

#[derive(Debug)]
pub(crate) struct Test {
    path: PathBuf,
    cases: Vec<TestCase>,
    inputs: Vec<Input>,
}

impl Test {
    pub(crate) fn read<Format: Read>(path: &Path) -> Result<Self> {
        Format::read(path)
    }

    pub(crate) fn render<Format: Render>(
        &self,
        aux: Format::Aux,
    ) -> Result<String> {
        Format::render(self, aux)
    }

    pub(crate) fn err_path(&self) -> PathBuf {
        self.path.with_extension("err")
    }

    pub(crate) fn inputs(
        &self,
        base: &Path,
        uniq: &Path,
    ) -> impl Iterator<Item = (String, String, PathBuf, String)> {
        let base = base.join(uniq);
        self.inputs
            .iter()
            .map(|input| {
                let (data_var, file_var, path) = if let Some(name) = &input.name
                {
                    (
                        format!("{name}_DATA"),
                        format!("{name}_FILE"),
                        base.join("named").join(name),
                    )
                } else {
                    (
                        String::from("INPUT"),
                        String::from("INPUT_FILE"),
                        base.join("defaut"),
                    )
                };
                (data_var, file_var, path, input.contents.clone())
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}
