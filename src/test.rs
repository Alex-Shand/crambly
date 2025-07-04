use anyhow::Result;
use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};

use self::renderers::CramRenderer;

mod lexer;
mod parser;
mod renderers;
mod token;

#[derive(Debug)]
pub(crate) struct Test {
    path: PathBuf,
    cases: Vec<TestCase>,
}

#[derive(Debug)]
struct TestCase {
    name: String,
    command: String,
    output: String,
    output_start_line: usize,
    output_end_line: usize,
}

impl Test {
    pub(crate) fn read(path: &Path, str: String) -> Result<Self> {
        let mut lexer = lexer::lexer(str);
        Ok(parser::parse(&mut lexer, path)?)
    }

    pub(crate) fn read_cram(str: String) -> Self {
        let mut lines = str.lines().filter(|s| !s.is_empty()).peekable();
        assert_eq!(lines.next(), Some("CRAMBLY GENERATED CRAM TEST"));
        let path = lines
            .next()
            .unwrap()
            .strip_prefix("ORIGINAL SOURCE: ")
            .unwrap()
            .into();
        let mut cases = Vec::new();
        while lines.peek().is_some() {
            cases.push(TestCase::read_cram(&mut lines));
        }
        Test { path, cases }
    }

    pub(crate) fn render(&self) -> String {
        CramRenderer(self).to_string()
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn err_path(&self) -> PathBuf {
        self.path.with_extension("err")
    }

    pub(crate) fn render_err(&self, original: &str) -> String {
        let mut err =
            original.lines().map(ToOwned::to_owned).collect::<Vec<_>>();
        for case in &self.cases {
            err = case.render_err(err);
        }
        err.join("\n")
    }
}

impl TestCase {
    fn render_err(&self, mut err: Vec<String>) -> Vec<String> {
        drop(err.splice(
            self.output_start_line..=self.output_end_line,
            self.output.lines().map(ToOwned::to_owned),
        ));
        err
    }

    fn read_cram<'a>(lines: &mut impl Iterator<Item = &'a str>) -> TestCase {
        let name = lines
            .next()
            .unwrap()
            .strip_prefix("START TEST CASE: ")
            .unwrap()
            .to_owned();
        let command = lines
            .next()
            .unwrap()
            .strip_prefix("  $ ")
            .unwrap()
            .to_owned();
        let output = lines
            .take_while(|&l| l != "CRAMBLY TEST CASE METADATA")
            .map(|l| l.strip_prefix("  ").unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        let output_start_line = lines
            .next()
            .unwrap()
            .strip_prefix("OUTPUT START LINE: ")
            .unwrap()
            .parse()
            .unwrap();
        let output_end_line = lines
            .next()
            .unwrap()
            .strip_prefix("OUTPUT END LINE: ")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(
            lines.next(),
            Some(format!("END TEST CASE: {name}").as_str())
        );
        TestCase {
            name,
            command,
            output,
            output_start_line,
            output_end_line,
        }
    }
}
