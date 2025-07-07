use super::command::Command;

#[derive(Debug, PartialEq)]
pub(crate) struct TestCase {
    pub(crate) name: String,
    pub(crate) command: Command,
    pub(crate) output: String,
    pub(crate) output_start_line: usize,
    pub(crate) output_end_line: usize,
}
