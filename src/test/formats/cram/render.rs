use std::fmt;

use super::magic;
use crate::test::{Test, tc::TestCase};

pub(crate) fn render(test: &Test) -> String {
    TestRenderer(test).to_string()
}

struct TestRenderer<'a>(pub(crate) &'a Test);

impl fmt::Display for TestRenderer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", magic::HEADER)?;
        writeln!(f, "{}{}", magic::PATH_PREFIX, self.0.path)?;
        for case in &self.0.cases {
            writeln!(f, "{}", CaseRenderer(case))?;
        }

        Ok(())
    }
}

struct CaseRenderer<'a>(pub(crate) &'a TestCase);

impl fmt::Display for CaseRenderer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}{}", magic::NAME_PREFIX, self.0.name)?;
        for _ in 0..20 {
            writeln!(f)?;
        }
        writeln!(f, "{}{}", magic::COMMAND_PREFIX, self.0.command)?;
        for line in self.0.output.lines() {
            writeln!(f, "{}{line}", magic::OUTPUT_PREFIX)?;
        }
        for _ in 0..20 {
            writeln!(f)?;
        }
        writeln!(f, "{}", magic::METADATA_HEADER)?;
        writeln!(
            f,
            "{}{}",
            magic::METADATA_OUTPUT_START_PREFIX,
            self.0.output_start_line
        )?;
        writeln!(
            f,
            "{}{}",
            magic::METADATA_OUTPUT_END_PREFIX,
            self.0.output_end_line
        )?;
        writeln!(f, "{}{}", magic::END_TEST_CASE, self.0.name)?;
        Ok(())
    }
}
