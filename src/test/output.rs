#[derive(Debug)]
pub(crate) struct Output {
    pub(crate) text: String,
    pub(crate) start_line: Option<usize>,
    pub(crate) end_line: Option<usize>,
}
