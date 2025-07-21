use std::{collections::HashMap, fmt};

use camino::Utf8PathBuf as PathBuf;

use super::magic;
use crate::test::{Test, tc::TestCase};

pub(crate) fn render(test: &Test, vars: &HashMap<String, PathBuf>) -> String {
    TestRenderer(test, vars).to_string()
}

struct TestRenderer<'a>(&'a Test, &'a HashMap<String, PathBuf>);

impl fmt::Display for TestRenderer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", magic::HEADER)?;
        writeln!(f, "{}{}", magic::PATH_PREFIX, self.0.path)?;
        for (var, path) in self.1 {
            writeln!(f, "{}export {var}_FILE={path}", magic::COMMAND_PREFIX)?;
            writeln!(
                f,
                "{}export {var}=$(cat ${var}_FILE)",
                magic::COMMAND_PREFIX
            )?;
        }
        writeln!(f, "{}", magic::INPUT_END)?;
        for case in &self.0.cases {
            writeln!(f, "{}", CaseRenderer(case))?;
        }

        Ok(())
    }
}

struct CaseRenderer<'a>(&'a TestCase);

impl fmt::Display for CaseRenderer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}{}", magic::NAME_PREFIX, self.0.name)?;
        for _ in 0..20 {
            writeln!(f)?;
        }
        writeln!(f, "{}{}", magic::COMMAND_PREFIX, self.0.command.first_line)?;
        for line in &self.0.command.rest_lines {
            writeln!(f, "{}{}", magic::COMMAND_CONT_PREFIX, line)?;
        }
        for line in self.0.output.text.lines() {
            writeln!(f, "{}", OutputRenderer(line))?;
        }
        for _ in 0..20 {
            writeln!(f)?;
        }
        writeln!(f, "{}", magic::METADATA_HEADER)?;
        writeln!(
            f,
            "{}{}",
            magic::METADATA_COMMAND_END_LINE_PREFIX,
            self.0.command.end_line
        )?;
        if let Some(line) = self.0.output.start_line {
            writeln!(f, "{}{line}", magic::METADATA_OUTPUT_START_PREFIX)?;
        }
        if let Some(line) = self.0.output.end_line {
            writeln!(f, "{}{line}", magic::METADATA_OUTPUT_END_PREFIX)?;
        }
        writeln!(f, "{}{}", magic::METADATA_LAST_PREFIX, self.0.last)?;
        writeln!(f, "{}{}", magic::END_TEST_CASE, self.0.name)?;
        Ok(())
    }
}

struct OutputRenderer<'a>(&'a str);

impl fmt::Display for OutputRenderer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.0.strip_prefix("$ ") {
            // If the output line starts with a "$ " then cram will think it's a
            // command even if crambly doesn't. We escape the $ by using a
            // single character character class in regex mode
            write!(f, "{}[$] {} (re)", magic::OUTPUT_PREFIX, s)?;
        } else {
            // Normal output line
            write!(f, "{}{}", magic::OUTPUT_PREFIX, self.0)?;
        }
        Ok(())
    }
}
