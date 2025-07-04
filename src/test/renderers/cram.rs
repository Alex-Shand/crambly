use std::fmt;

use crate::{Test, test::TestCase};

pub(crate) struct CramRenderer<'a>(pub(crate) &'a Test);

impl fmt::Display for CramRenderer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CRAMBLY GENERATED CRAM TEST")?;
        writeln!(f, "ORIGINAL SOURCE: {}", self.0.path)?;
        for case in &self.0.cases {
            writeln!(f, "{}", CaseRenderer(case))?;
        }

        Ok(())
    }
}

struct CaseRenderer<'a>(pub(crate) &'a TestCase);

impl fmt::Display for CaseRenderer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "START TEST CASE: {}", self.0.name)?;
        for _ in 0..20 {
            writeln!(f)?;
        }
        writeln!(f, "  $ {}", self.0.command)?;
        for line in self.0.output.lines() {
            writeln!(f, "  {line}")?;
        }
        for _ in 0..20 {
            writeln!(f)?;
        }
        writeln!(f, "CRAMBLY TEST CASE METADATA")?;
        writeln!(f, "OUTPUT START LINE: {}", self.0.output_start_line)?;
        writeln!(f, "OUTPUT END LINE: {}", self.0.output_end_line)?;
        writeln!(f, "END TEST CASE: {}", self.0.name)?;
        Ok(())
    }
}
