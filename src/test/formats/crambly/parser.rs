use camino::Utf8Path as Path;

use super::{Test, token::TokenAndSpan};

mod case;
mod command;
mod output;

type Lexer<'a> = pratt::LexerHandle<'a, TokenAndSpan, ()>;
type Table<Ast> = pratt::Table<TokenAndSpan, (), Ast>;
type Result<Ast> = pratt::Result<TokenAndSpan, Ast>;

pub(crate) fn parse(lexer: &mut Lexer<'_>, path: &Path) -> Result<Test> {
    let mut cases = Vec::new();

    while lexer.peek(())?.is_some() {
        cases.push(case::parse(lexer)?);
    }

    if let Some(last) = cases.last_mut() {
        last.last = true;
    }

    Ok(Test {
        path: path.to_owned(),
        cases,
    })
}
