//! サンプル言語の式

use super::*;

/// match 式のアーム (=> の部分)。
///
/// マッチ時に実行される式は網羅性検査には影響しないので省略する。
///
/// この網羅性検査はガード (if 節) があるアームを無視するため、
/// ガードの条件式も省略している。
pub(crate) struct MatchArm {
    pub(crate) pattern: Pattern,
}

pub(crate) struct MatchExpression {
    /// パターンマッチの対象となる式の型。
    /// この網羅性検査では型しか利用しないので、式は持たない。
    pub(crate) condition_ty: Ty,

    pub(crate) arms: Vec<MatchArm>,
}
