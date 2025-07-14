#[derive(Debug, PartialEq)]
pub(crate) struct Command {
    pub(crate) first_line: String,
    pub(crate) rest_lines: Vec<String>,
    pub(crate) end_line: usize,
}
