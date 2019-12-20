use super::space_from_ty::{space_can_decompose, space_decompose};
use super::*;

/// スペースの交差 (共通部分) を求める。
pub(crate) fn space_intersection(mut first: Space, mut second: Space, td: &TyDatabase) -> Space {
    // 空のスペースの交差は常に空になる。
    if first.is_empty() || second.is_empty() {
        return Space::new_empty();
    }

    match (&mut first, &mut second) {
        // 型スペース同士の交差は、2つの型の間に部分型関係があれば、
        // 部分型の方のスペースになる。
        // (S⊂T → S∩T = S)
        (Space::Ty(ref subty), Space::Ty(ref super_ty)) if td.is_subtype_of(subty, super_ty) => {
            first
        }
        // 左右対称
        (Space::Ty(ref super_ty), Space::Ty(ref subty)) if td.is_subtype_of(subty, super_ty) => {
            second
        }

        // 型 T のスペースと、それに含まれるコンストラクタ K のスペースの交差は、
        // コンストラクタ K に絞られる。
        // (K ⊂ T → K∩T = K)
        (Space::Ty(ref ty), Space::Constructor { ref name, .. })
            if td.is_subtype_of_constructor(ty, name) =>
        {
            second
        }
        // 左右対称
        (Space::Constructor { ref name, .. }, Space::Ty(ty))
            if td.is_subtype_of_constructor(ty, name) =>
        {
            first
        }

        // ユニオンを分配する。
        (Space::Union(ref mut union), _) => {
            let union = std::mem::replace(union, vec![]);

            Space::new_union(
                union
                    .into_iter()
                    .map(|subspace| space_intersection(subspace, second.clone(), td)),
            )
        }
        // 左右対称
        (_, Space::Union(..)) => space_intersection(second, first, td),

        // 同じコンストラクタ同士の交差は、各フィールドの交差をとる。
        // FIXME: 実装
        (
            Space::Constructor { ref name, .. },
            Space::Constructor {
                name: ref second_name,
                ..
            },
        ) if name == second_name => first,

        // 型スペースを分解して交差を取る。
        (&mut ref s, _) if space_can_decompose(&s, td) => {
            let first = space_decompose(first, td);
            space_intersection(first, second, td)
        }
        // 左右対称
        (_, &mut ref s) if space_can_decompose(&s, td) => space_intersection(second, first, td),

        _ => Space::new_empty(),
    }
}
