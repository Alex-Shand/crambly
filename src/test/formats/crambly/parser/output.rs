use super::{Lexer, Result};
use crate::test::{formats::crambly::token::Token, output::Output};

pub(crate) fn parse(lexer: &mut Lexer<'_>) -> Result<Output> {
    parse_inner(lexer, ())
}

#[pratt::free]
fn parse_inner(lexer: &mut Lexer<'_>, (): ()) -> Result<Output> {
    let (text, start_line, end_line) = if check!(Token::Output(_)) {
        let text = demand!(Token::Output(text) => text.clone());
        let span = current_span!();
        // Subtract 1 because Span reports lines 1 indexed, we need 0 indexed
        (
            text,
            span.start_line().map(|x| x - 1),
            span.end_line().map(|x| x - 1),
        )
    } else if check!(Token::DelimitedOutput(_)) {
        let text = demand!(Token::DelimitedOutput(text) => text.clone());
        let span = current_span!();
        // This span includes the start and end markers so the start line is
        // actually correct & the end line is 2 ahead of where it should be
        (text, span.start_line(), span.end_line().map(|x| x - 2))
    } else {
        (String::new(), None, None)
    };
    let start_line = start_line.map(|x| x as isize);
    let end_line = end_line.map(|x| x as isize);
    Ok(Output {
        text,
        start_line,
        end_line,
    })
}
