#[derive(Debug)]
pub(crate) struct TestCase {
    pub(crate) name: String,
    pub(crate) command: String,
    pub(crate) output: String,
    pub(crate) output_start_line: usize,
    pub(crate) output_end_line: usize,
}
