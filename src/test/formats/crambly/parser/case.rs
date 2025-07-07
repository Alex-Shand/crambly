use super::{Lexer, Result, command};
use crate::test::{formats::crambly::token::Token, tc::TestCase};

pub(crate) fn parse(lexer: &mut Lexer<'_>) -> Result<TestCase> {
    parse_inner(lexer, ())
}

#[pratt::free]
fn parse_inner(lexer: &mut Lexer<'_>, (): ()) -> Result<TestCase> {
    let command = command::parse(lexer)?;
    let output = require!(Token::ImplicitOutput(output) => output.clone(), "Expect output following command");
    let output_span = current_span!();
    Ok(TestCase {
        name: String::from("DEFAULT"),
        command,
        output,
        output_start_line: output_span.start_line().unwrap() - 1,
        output_end_line: output_span.end_line().unwrap() - 1,
    })
}
