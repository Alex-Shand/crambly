#[derive(Debug)]
pub(crate) struct Output {
    pub(crate) text: String,
    pub(crate) start_line: Option<isize>,
    pub(crate) end_line: Option<isize>,
}
