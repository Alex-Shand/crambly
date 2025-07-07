use pratt::{
    Lexer,
    lexer::{LexError, Result},
    span::Chars,
};

use super::token::{Token, TokenAndSpan};

pub(crate) fn lexer(
    str: String,
) -> impl Lexer<Token = TokenAndSpan, Context = ()> {
    pratt::lexer::builder()
        .with_token_fn(generate_token)
        .build(str)
}

fn generate_token(chars: &mut Chars, (): ()) -> Result<Option<TokenAndSpan>> {
    while let Some(c) = chars.skip_whitespace() {
        let start = chars.start_token();
        macro_rules! next {
            () => {{
                let _ = chars.next();
            }};
        }

        macro_rules! send {
            ($token:expr) => {{
                let token = $token;
                let span = chars.end_token(start);
                return Ok(Some(TokenAndSpan { token, span }));
            }};
        }

        macro_rules! keywords {
            ($($tt:tt)*) => {
                pratt::lexer::keywords!(chars = chars, match = {$($tt)*})
            }
        }

        // Comment
        if c == '#' {
            for _ in chars.take_while(|&c| c != '\n') {}
            continue;
        }

        // Command
        if c == '$' {
            next!();
            let command = chars
                .peek_while(not_newline)
                .collect::<String>()
                .trim()
                .to_owned();
            send!(Token::Command(command));
        }

        // Command continuation
        if c == '>' {
            next!();
            let command = chars
                .peek_while(not_newline)
                .collect::<String>()
                .trim()
                .to_owned();
            send!(Token::CommandCont(command));
        }

        keywords! {
            "@@@@@@" [is_newline] => {
                // Skip over the newline
                next!();
                let mut have_end = false;
                let mut output = String::new();
                while let Some(c) = chars.peek() {
                    if c == '\n' {
                        // Might be an end marker, check
                        let mut checkpoint = chars.checkpoint();
                        let str = (&mut checkpoint).take(7).collect::<String>();
                        if str == "\n@@@@@@" {
                            // Is an end marker
                            checkpoint.commit();
                            have_end = true;
                            break;
                        }
                        // Not an end marker
                        checkpoint.abort();
                    }
                    output.push(c);
                    next!();
                }
                // If we got here we either saw an end marker or ran out of
                // characters. We emit at least the start marker and the output
                // we gathered.

                // Report an error if we didn't see the end marker
                if !have_end {
                    return Err(LexError::unexpected_eof_msg("Expected `@@@@@@` to terminate delimited output"));
                }

                // We give the output token a span that covers everything
                // including the start and end markers, this is wrong but the
                // parser accounts for it
                send!(Token::DelimitedOutput(output));
            }
        }

        // Anything not preceeded by a key symbol is implicit output
        let rest = chars.collect::<String>().trim().to_owned();
        send!(Token::Output(rest));
    }
    Ok(None)
}

fn is_newline(c: char) -> bool {
    c == '\n'
}

fn not_newline(c: char) -> bool {
    c != '\n'
}
