use std::iter;

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
    let mut offset = 0;
    for case in &test.cases {
        let (new_err, new_offset) = render_case(case, err, offset);
        err = new_err;
        offset += new_offset;
    }
    Ok(err.join("\n"))
}

fn render_case(
    case: &TestCase,
    mut err: Vec<String>,
    offset: isize,
) -> (Vec<String>, isize) {
    // This function will only be called if some test case in the file failed,
    // it might not have been this one though
    let new_line_count = case.output.text.lines().count() as isize;
    let iter = case.output.text.lines().map(ToOwned::to_owned);
    let offset = match (case.output.start_line, case.output.end_line) {
        // There was already expected output from the command we can write the
        // new output into the space where the old output was supposed to go. We
        // return the difference between the original line count and the new
        // line count to adjust subsequent test cases appropriatly
        (Some(start), Some(end)) => {
            let old_line_count = end - start + 1;
            // Adjust start and end for the configured offset & cast to usize
            // for splice
            let start = (start + offset) as usize;
            let end = (end + offset) as usize;
            drop(err.splice(start..=end, iter));
            new_line_count - old_line_count
        }
        // There was no expected output from the command and we didn't get any
        // this time either. This is fine
        _ if case.output.text.is_empty() => 0,
        // From this point on we know we didn't expect any output but we got
        // some. What we do about it depends on whether the command we're
        // looking at is the last one in the file.
        //
        // If it is the last command we can just append the output verbatim
        _ if case.last => {
            err.extend(iter);
            // 0 isn't right but this is the last command so who cares
            0
        }
        // It isn't the last command, we have to splice in the new output right
        // after the command, bracketed by `@@@@@@`
        _ => {
            let line = case.command.end_line + (offset as usize) + 1;
            drop(
                err.splice(
                    line..line,
                    iter::once(String::from("@@@@@@"))
                        .chain(iter)
                        .chain(iter::once(String::from("@@@@@@"))),
                ),
            );
            dbg!(new_line_count);
            // All of the lines are new + 2 for the brackets
            new_line_count + 2
        }
    };
    (err, offset)
}
