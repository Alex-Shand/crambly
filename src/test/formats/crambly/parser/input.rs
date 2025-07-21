use super::{Lexer, Result};
use crate::test::{formats::crambly::token::Token, input::Input};

#[expect(clippy::result_large_err)]
pub(crate) fn parse_if_present(
    lexer: &mut Lexer<'_>,
    mut f: impl FnMut(Input),
) -> Result<()> {
    let Some(input) = parse_inner(lexer, ())? else {
        return Ok(());
    };
    f(input);
    Ok(())
}

#[pratt::free]
fn parse_inner(lexer: &mut Lexer<'_>, (): ()) -> Result<Option<Input>> {
    if check!(Token::Input(_)) {
        let contents = demand!(Token::Input(i) => i.clone());
        return Ok(Some(Input {
            name: None,
            contents,
        }));
    }

    if check!(Token::NamedInput(_, _)) {
        let (name, contents) = demand!(Token::NamedInput(name, contents) => (name.clone(), contents.clone()));
        return Ok(Some(Input {
            name: Some(name),
            contents,
        }));
    }

    Ok(None)
}
