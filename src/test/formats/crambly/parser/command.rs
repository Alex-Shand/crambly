use super::{Lexer, Result};
use crate::test::{command::Command, formats::crambly::token::Token};

pub(crate) fn parse(lexer: &mut Lexer<'_>) -> Result<Command> {
    parse_inner(lexer, ())
}

#[pratt::free]
fn parse_inner(lexer: &mut Lexer<'_>, (): ()) -> Result<Command> {
    let first_line = require!(Token::Command(command) => command.clone(), "Expect command to start test case");
    let mut rest_lines = Vec::new();
    let mut last_span = current_span!();
    while check!(Token::CommandCont(_)) {
        rest_lines.push(demand!(Token::CommandCont(cont) => cont.clone()));
        last_span = current_span!();
    }
    Ok(Command {
        first_line,
        rest_lines,
        end_line: last_span.end_line().unwrap() - 1,
    })
}
