use super::tokenize_context::TokenizeContext;
use super::*;
use std::rc::Rc;

pub(crate) fn tokenize(source_code: Rc<String>) -> Box<[TokenData]> {
    let mut t = TokenizeContext::new(source_code);
    tokenize_rules::tokenize_all(&mut t);
    t.finish()
}

pub(crate) fn tokenize_with_utf16_indices(source_code: Rc<String>) -> Box<[(Token, usize)]> {
    fn go(token: &TokenData, token_indices: &mut Vec<(Token, usize)>, index: &mut usize) {
        for trivia in token.leading() {
            match trivia {
                Trivia::Token(token) => {
                    go(token, token_indices, index);
                }
                Trivia::Error(_) => continue,
            }
        }

        let start = *index;
        *index += token.text().encode_utf16().count();

        token_indices.push((token.token(), start));

        for trivia in token.trailing() {
            match trivia {
                Trivia::Token(token) => {
                    go(token, token_indices, index);
                }
                Trivia::Error(_) => continue,
            }
        }
    }

    let tokens = tokenize(source_code);

    let mut token_indices = vec![];
    let mut index = 0;

    for token in tokens.iter() {
        go(token, &mut token_indices, &mut index);
    }

    token_indices.into_boxed_slice()
}
