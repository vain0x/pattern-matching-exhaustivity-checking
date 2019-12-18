//! サンプル言語のパターン

/// パターン
#[derive(Clone)]
pub(crate) enum Pattern {
    /// `_`
    Discard,

    /// コンストラクタパターン
    ///
    /// enum の特定のコンストラクタにだけマッチするようなパターン。
    /// 例: `Boolean::True` や `Pair::Pair(_, _)` など。
    Constructor { name: String },
}
