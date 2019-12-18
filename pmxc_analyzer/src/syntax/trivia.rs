use super::*;

/// トリビア
///
/// ここでは構文的にあまり意味のない字句をトリビアを呼んでいる。
/// 空白やコメント、解釈できない文字など。
#[derive(Clone, Debug)]
pub(crate) enum Trivia {
    Token(TokenData),
    Error(ParseError),
}

impl From<TokenData> for Trivia {
    fn from(token: TokenData) -> Trivia {
        Trivia::Token(token)
    }
}

impl From<ParseError> for Trivia {
    fn from(error: ParseError) -> Trivia {
        Trivia::Error(error)
    }
}
