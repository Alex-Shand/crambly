#[derive(Debug, pratt::Token)]
pub(crate) enum Token {
    #[pratt(payload = format!("$ {it}"))]
    Command(String),

    #[pratt(payload = it.clone())]
    ImplicitOutput(String),
}

pub(crate) type TokenAndSpan = pratt::TokenAndSpan<Token>;
pub(crate) type TokenType = <Token as pratt::Token>::Type;
