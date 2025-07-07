use anyhow::Result;
use camino::Utf8Path as Path;

use crate::{
    test::{Test, tc::TestCase},
    utils,
};

mod lexer;
mod parser;
mod token;

pub(crate) fn read(path: &Path) -> Result<Test> {
    let mut lexer = lexer::lexer(utils::read_file(path)?);
    Ok(parser::parse(&mut lexer, path)?)
}

pub(crate) fn render(test: &Test) -> Result<String> {
    let original = utils::read_file(&test.path)?;
    let mut err = original.lines().map(ToOwned::to_owned).collect::<Vec<_>>();
    for case in &test.cases {
        err = render_case(case, err);
    }
    Ok(err.join("\n"))
}

fn render_case(case: &TestCase, mut err: Vec<String>) -> Vec<String> {
    let iter = case.output.text.lines().map(ToOwned::to_owned);
    match (case.output.start_line, case.output.end_line) {
        (Some(start), Some(end)) => {
            if start >= err.len() {
                err.extend(iter);
            } else {
                drop(err.splice(start..=end, iter));
            }
        }
        _ => err.extend(iter),
    }
    err
}
