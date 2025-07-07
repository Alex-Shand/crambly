use super::{Lexer, Result, command, output};
use crate::test::tc::TestCase;

pub(crate) fn parse(lexer: &mut Lexer<'_>) -> Result<TestCase> {
    parse_inner(lexer, ())
}

#[pratt::free]
fn parse_inner(lexer: &mut Lexer<'_>, (): ()) -> Result<TestCase> {
    let command = command::parse(lexer)?;
    let output = output::parse(lexer)?;
    Ok(TestCase {
        name: String::new(),
        command,
        output,
    })
}
