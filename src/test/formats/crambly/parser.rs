use std::collections::HashSet;

use camino::Utf8Path as Path;
use pratt::Error;

use super::{Test, token::TokenAndSpan};

mod case;
mod command;
mod input;
mod output;

type Lexer<'a> = pratt::LexerHandle<'a, TokenAndSpan, ()>;
type Result<Ast> = pratt::Result<TokenAndSpan, Ast>;

#[expect(clippy::result_large_err)]
pub(crate) fn parse(lexer: &mut Lexer<'_>, path: &Path) -> Result<Test> {
    let mut cases = Vec::new();
    let mut inputs = Vec::new();

    while lexer.peek(())?.is_some() {
        let (case, i) = case::parse(lexer)?;
        cases.push(case);
        inputs.extend(i);
    }

    if let Some(last) = cases.last_mut() {
        last.last = true;
    }

    let mut seen_default_input = false;
    let mut input_names = HashSet::new();
    for input in &inputs {
        if let Some(name) = &input.name {
            if input_names.contains(name) {
                return Err(Error::custom(format!(
                    "Duplicate input name: {name}"
                )));
            }
            let _ = input_names.insert(name.clone());
        } else {
            if seen_default_input {
                return Err(Error::custom("Only one unnamed input is allowed"));
            }
            seen_default_input = true;
        }
    }

    if seen_default_input && !input_names.is_empty() {
        return Err(Error::custom("Cannot mix named and unnamed inputs"));
    }

    Ok(Test {
        path: path.to_owned(),
        cases,
        inputs,
    })
}
