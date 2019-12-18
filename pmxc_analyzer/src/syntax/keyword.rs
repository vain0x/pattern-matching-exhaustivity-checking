use super::*;

pub(crate) static KEYWORD_TABLE: &[(Token, &str)] = &[
    (Token::Enum, "enum"),
    (Token::Let, "let"),
    (Token::Match, "match"),
    (Token::Struct, "struct"),
    (Token::Underscore, "_"),
];

impl Token {
    pub(crate) fn is_control_keyword(self) -> bool {
        self == Token::Match
    }

    pub(crate) fn is_keyword(self) -> bool {
        self == Token::Enum
            || self == Token::Let
            || self == Token::Struct
            || self == Token::Underscore
            || self.is_control_keyword()
    }

    pub(crate) fn parse_keyword(text: &str) -> Option<Token> {
        KEYWORD_TABLE
            .iter()
            .filter_map(|&(keyword, keyword_text)| {
                if text == keyword_text {
                    Some(keyword)
                } else {
                    None
                }
            })
            .next()
    }
}
