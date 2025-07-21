#[derive(Debug, pratt::Token)]
pub(crate) enum Token {
    #[pratt(payload = format!("$ {it}"))]
    Command(String),
    #[pratt(payload = format!("> {it}"))]
    CommandCont(String),

    #[pratt(payload = format!("%%%%%%\n{it}\n%%%%%%"))]
    Input(String),
    #[pratt(payload = format!("%%% {} %%%\n{}\n%%%%%%", it.0, it.1))]
    NamedInput(String, String),

    #[pratt(payload = it.clone())]
    Output(String),
    #[pratt(payload = format!("@@@@@@\n{it}\n@@@@@@"))]
    DelimitedOutput(String),
}

pub(crate) type TokenAndSpan = pratt::TokenAndSpan<Token>;
