//! 約物 (punctuations)

use super::*;

pub(crate) static PUN_TABLE: &[(Token, &str)] = &[
    (Token::LeftParen, "("),
    (Token::RightParen, ")"),
    (Token::LeftAngle, "<"),
    (Token::RightAngle, ">"),
    (Token::LeftBracket, "["),
    (Token::RightBracket, "]"),
    (Token::LeftBrace, "{"),
    (Token::RightBrace, "}"),
    (Token::Colon, ":"),
    (Token::Comma, ","),
    (Token::Dot, "."),
    (Token::Equal, "="),
    (Token::Minus, "-"),
    (Token::Semi, ";"),
];

impl Token {
    pub(crate) fn parse_pun(text: &str) -> Option<Token> {
        PUN_TABLE
            .iter()
            .filter_map(
                |&(token, pun_text)| {
                    if text == pun_text {
                        Some(token)
                    } else {
                        None
                    }
                },
            )
            .next()
    }
}
