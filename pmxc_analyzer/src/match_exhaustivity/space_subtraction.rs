use super::space_from_ty::{space_can_decompose, space_decompose};
use super::*;

/// スペースからスペースを引く。
pub(crate) fn space_subtraction(mut first: Space, mut second: Space, td: &TyDatabase) -> Space {
    // 空のスペースからは何を引いても空。
    if first.is_empty() {
        return Space::new_empty();
    }

    // 空のスペースを引いても変化しない。
    if second.is_empty() {
        return first;
    }

    match (&mut first, &mut second) {
        // 部分型スペースから上位型スペースを引くと空になる。
        (Space::Ty(ref subty), Space::Ty(ref super_ty)) if td.is_subtype_of(subty, super_ty) => {
            Space::new_empty()
        }

        // コンストラクタ型スペースからコンストラクタスペースを引く。
        // 左辺をコンストラクタスペースにばらすだけ。
        (Space::Ty(ref mut ty), Space::Constructor { ref name })
            if ty.is_constructor_of_name(name) =>
        {
            let name = match std::mem::replace(ty, Ty::default()) {
                Ty::Constructor { name } => name,
                _ => unreachable!(),
            };
            let first = Space::Constructor { name };
            space_subtraction(first, second, td)
        }

        // ユニオンを分配する。
        // (x | y) \ z = x \ z | y \ z
        (Space::Union(ref mut union), _) => {
            let union = std::mem::replace(union, vec![]);
            Space::new_union(
                union
                    .into_iter()
                    .map(|subspace| space_subtraction(subspace, second.clone(), td)),
            )
        }
        // x \ (y | z) = x \ y \ z
        (_, Space::Union(ref mut union)) => {
            let union = std::mem::replace(union, vec![]);
            union
                .into_iter()
                .fold(first, |first, second| space_subtraction(first, second, td))
        }

        // コンストラクタスペースから、そのコンストラクタを含む型のスペースを引くと、空になる。
        (Space::Constructor { ref name }, Space::Ty(ref ty)) if ty.is_constructor_of_name(name) => {
            Space::new_empty()
        }

        // コンストラクタが等しいコンストラクタスペースを引く。
        (
            Space::Constructor { ref name },
            Space::Constructor {
                name: ref second_name,
            },
        ) if name == second_name => {
            // FIXME: 引数
            Space::new_empty()
        }

        // 型スペースを分解して差をとる。
        (&mut ref s, _) if space_can_decompose(s, td) => {
            let first = space_decompose(first, td);
            space_subtraction(first, second, td)
        }
        (_, &mut ref s) if space_can_decompose(s, td) => {
            let second = space_decompose(second, td);
            space_subtraction(first, second, td)
        }

        _ => first,
    }
}
