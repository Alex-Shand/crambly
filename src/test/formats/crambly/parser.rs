use camino::Utf8Path as Path;
use pratt::lexer::LexError;

use super::{Test, token::TokenAndSpan};

mod case;

type Lexer<'a> = pratt::LexerHandle<'a, TokenAndSpan, ()>;
type Table<Ast> = pratt::Table<TokenAndSpan, (), Ast>;
type Result<Ast> = pratt::Result<TokenAndSpan, Ast>;

pub(crate) fn parse(lexer: &mut Lexer<'_>, path: &Path) -> Result<Test> {
    let mut cases = Vec::new();

    while lexer.peek(()).map_err(LexError::widen)?.is_some() {
        cases.push(case::parse(lexer)?);
    }

    Ok(Test {
        path: path.to_owned(),
        cases,
    })
}
