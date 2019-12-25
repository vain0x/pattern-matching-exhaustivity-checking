use super::*;

/// 字句の種類
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Token {
    Eof,
    Eol,
    Space,
    Comment,
    Number,
    Ident,
    Other,

    // キーワード
    Enum,
    Let,
    Match,
    Struct,
    Underscore,

    // 約物
    /// (
    LeftParen,
    /// )
    RightParen,
    /// <
    LeftAngle,
    /// >
    RightAngle,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// :
    Colon,
    /// ,
    Comma,
    /// .
    Dot,
    /// =
    Equal,
    /// -
    Minus,
    /// ;
    Semi,
}

impl Token {
    pub(crate) fn is_leading_trivia(self) -> bool {
        self == Token::Eol || self.is_trailing_trivia()
    }

    pub(crate) fn is_trailing_trivia(self) -> bool {
        self == Token::Space || self == Token::Comment || self == Token::Other
    }

    pub(crate) fn is_trivia(self) -> bool {
        debug_assert!(!self.is_trailing_trivia() || self.is_leading_trivia());

        self.is_leading_trivia()
    }
}

/// 字句のデータ
#[derive(Clone, Debug)]
pub(crate) struct TokenData {
    token: Token,
    text: String,
    leading: Vec<Trivia>,
    trailing: Vec<Trivia>,
}

impl TokenData {
    pub(crate) fn new(token: Token, text: String) -> Self {
        TokenData {
            token,
            text,
            leading: vec![],
            trailing: vec![],
        }
    }

    pub(crate) fn token(&self) -> Token {
        self.token
    }

    pub(crate) fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn leading(&self) -> &[Trivia] {
        &self.leading
    }

    pub(crate) fn trailing(&self) -> &[Trivia] {
        &self.trailing
    }

    pub(crate) fn push_leading_token(&mut self, token: TokenData) {
        self.leading.push(token.into());
    }

    pub(crate) fn push_trailing_token(&mut self, token: TokenData) {
        self.trailing.push(token.into());
    }

    fn traverse_tokens<F: FnMut(&TokenData) -> bool>(&self, f: &mut F) -> bool {
        for trivia in self.leading() {
            if !trivia.as_token().traverse_tokens(f) {
                return false;
            }
        }

        if !f(self) {
            return false;
        }

        for trivia in self.trailing() {
            if !trivia.as_token().traverse_tokens(f) {
                return false;
            }
        }

        true
    }

    pub(crate) fn contains_eol(&self) -> bool {
        let mut ok = false;

        self.traverse_tokens(&mut |token| {
            if token.token() == Token::Eol {
                ok = true;
                return false;
            }
            true
        });

        ok
    }
}
