//! サンプル言語のパターン

use super::*;

/// パターン
#[derive(Clone)]
pub(crate) enum Pattern {
    /// `_`
    Discard { ty: Ty },

    /// コンストラクタパターン
    ///
    /// enum の特定のコンストラクタにだけマッチするようなパターン。
    /// 例: `Boolean::True` や `Pair::Pair(_, _)` など。
    Constructor { name: String, args: Vec<Pattern> },
}
