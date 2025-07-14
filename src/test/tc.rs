use super::{command::Command, output::Output};

#[derive(Debug)]
pub(crate) struct TestCase {
    pub(crate) name: String,
    pub(crate) command: Command,
    pub(crate) output: Output,
    pub(crate) last: bool,
}
