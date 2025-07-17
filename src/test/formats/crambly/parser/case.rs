use super::{Lexer, Result, command, input, output};
use crate::test::{input::Input, tc::TestCase};

pub(crate) fn parse(lexer: &mut Lexer<'_>) -> Result<(TestCase, Vec<Input>)> {
    parse_inner(lexer, ())
}

#[pratt::free]
fn parse_inner(
    lexer: &mut Lexer<'_>,
    (): (),
) -> Result<(TestCase, Vec<Input>)> {
    let mut inputs = Vec::new();
    input::parse_if_present(lexer, |i| inputs.push(i))?;
    let command = command::parse(lexer)?;
    input::parse_if_present(lexer, |i| inputs.push(i))?;
    let output = output::parse(lexer)?;
    input::parse_if_present(lexer, |i| inputs.push(i))?;
    Ok((
        TestCase {
            name: String::new(),
            command,
            output,
            last: false,
        },
        inputs,
    ))
}
