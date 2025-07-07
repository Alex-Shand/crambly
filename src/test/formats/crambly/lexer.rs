use pratt::{Lexer, lexer::Result, span::Chars};

use super::token::{Token, TokenAndSpan};

pub(crate) fn lexer(
    str: String,
) -> impl Lexer<Token = TokenAndSpan, Context = ()> {
    pratt::lexer::builder()
        .with_state::<State>()
        .with_token_fn(generate_token)
        .build(str)
}

#[derive(Default)]
struct State {
    has_output: bool,
}

fn generate_token(
    chars: &mut Chars,
    (): (),
    state: &mut State,
) -> Result<Option<TokenAndSpan>> {
    macro_rules! next {
        () => {{
            let _ = chars.next();
        }};
    }
    macro_rules! send {
        ($token:expr, $start:expr) => {{
            let start = $start;
            let token = $token;
            let span = chars.end_token(start);
            return Ok(Some(TokenAndSpan { token, span }));
        }};
    }
    while let Some(c) = chars.skip_whitespace() {
        let start = chars.start_token();

        // Comment
        if c == '#' {
            for _ in chars.take_while(|&c| c != '\n') {}
            continue;
        }

        // Command
        if c == '$' {
            next!();
            let command = chars
                .peek_while(|c| c != '\n')
                .collect::<String>()
                .trim()
                .to_owned();
            send!(Token::Command(command), start);
        }
        // Command continuation
        if c == '>' {
            next!();
            let command = chars
                .peek_while(|c| c != '\n')
                .collect::<String>()
                .trim()
                .to_owned();
            send!(Token::CommandCont(command), start);
        }

        // Anything not preceeded by a key symbol is implicit output
        state.has_output = true;
        let rest = chars.collect::<String>().trim().to_owned();
        send!(Token::ImplicitOutput(rest), start);
    }

    if !state.has_output {
        state.has_output = true;
        send!(Token::ImplicitOutput(String::new()), chars.start_token());
    }
    Ok(None)
}
