use std::{iter::Peekable, str::FromStr};

use anyhow::Result;
use camino::Utf8Path as Path;

use super::magic;
use crate::{
    test::{Test, command::Command, output::Output, tc::TestCase},
    utils,
};

pub(crate) fn read(path: &Path) -> Result<Test> {
    let str = utils::read_file(path)?;
    let mut lines = str.lines().filter(|s| !s.is_empty()).peekable();
    let lines = &mut lines;
    next_eq(lines, magic::HEADER);
    let path = next_prefix(lines, magic::PATH_PREFIX);
    let mut cases = Vec::new();
    while lines.peek().is_some() {
        cases.push(read_case(lines));
    }
    Ok(Test { path, cases })
}

fn read_case<'a>(
    lines: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> TestCase {
    let name = next_prefix(lines, magic::NAME_PREFIX);
    let mut command = read_command(lines);
    let output = lines
        .take_while(|&l| l != magic::METADATA_HEADER)
        .map(|l| l.strip_prefix(magic::OUTPUT_PREFIX).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    command.end_line =
        next_prefix(lines, magic::METADATA_COMMAND_END_LINE_PREFIX);
    let output_start_line =
        next_if_prefix(lines, magic::METADATA_OUTPUT_START_PREFIX);
    let output_end_line =
        next_if_prefix(lines, magic::METADATA_OUTPUT_END_PREFIX);
    let last = next_prefix(lines, magic::METADATA_LAST_PREFIX);
    next_eq(lines, format!("{}{name}", magic::END_TEST_CASE));
    TestCase {
        name,
        command,
        output: Output {
            text: output,
            start_line: output_start_line,
            end_line: output_end_line,
        },
        last,
    }
}

fn read_command<'a>(
    lines: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> Command {
    let first_line = next_prefix(lines, magic::COMMAND_PREFIX);
    let mut rest_lines = Vec::new();
    while let Some(line) = lines.peek() {
        if !line.starts_with(magic::COMMAND_CONT_PREFIX) {
            break;
        }
        rest_lines.push(next_prefix(lines, magic::COMMAND_CONT_PREFIX));
    }
    Command {
        first_line,
        rest_lines,
        end_line: 0,
    }
}

fn next_eq<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    expected: impl AsRef<str>,
) {
    assert_eq!(lines.next(), Some(expected.as_ref()));
}

fn next_if_prefix<'a, R: FromStr>(
    lines: &mut Peekable<impl Iterator<Item = &'a str>>,
    prefix: impl AsRef<str>,
) -> Option<R> {
    let str = lines.peek()?.strip_prefix(prefix.as_ref())?;
    let _ = lines.next();
    Some(str.parse().ok().unwrap())
}

fn next_prefix<'a, R: FromStr>(
    lines: &mut Peekable<impl Iterator<Item = &'a str>>,
    prefix: impl AsRef<str>,
) -> R {
    next_if_prefix(lines, prefix).unwrap()
}
