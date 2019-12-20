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
        (Space::Ty(ref mut ty), Space::Constructor { ref name, .. })
            if ty.is_constructor_of_name(name) =>
        {
            let ty = std::mem::replace(ty, Ty::default());
            let first = space_decompose(Space::Ty(ty), td);
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
        (Space::Constructor { ref name, .. }, Space::Ty(ref ty))
            if ty.is_constructor_of_name(name) =>
        {
            Space::new_empty()
        }

        // コンストラクタが等しいコンストラクタスペースを引く。
        (
            Space::Constructor {
                ref mut name,
                args: ref mut first_args,
            },
            Space::Constructor {
                name: ref second_name,
                args: ref mut second_args,
            },
        ) if name == second_name => {
            let name = std::mem::replace(name, String::default());
            let first_args = std::mem::replace(first_args, vec![]);
            let second_args = std::mem::replace(second_args, vec![]);

            debug_assert_eq!(
                first_args.len(),
                second_args.len(),
                "同じコンストラクタの引数の個数は一致するはず"
            );

            // すべての引数がカバーされているなら空になる。
            // (これは最後のケースの特別な場合を効率よく処理するもの、だと思う。)
            let all_are_covered =
                first_args
                    .iter()
                    .zip(second_args.iter())
                    .all(|(first, second)| {
                        let leak = space_subtraction(first.clone(), second.clone(), td);
                        leak.is_empty()
                    });
            if all_are_covered {
                return Space::new_empty();
            }

            // いずれかの引数のスペースが直交していたら何もしない。
            // (これも最後のケースの特別な場合を効率よく処理するもの、だと思う。)
            // FIXME: 実装

            // いずれかの引数スペースの差を取って、残りはそのまま、というスペースの和を作る。
            // 例えば型 (bool, bool) のパターンマッチで (true, false) というケースがあるとき、
            // 残りのケースとして考えられるのは「.0 が true でない」または「.1 が false でない」。
            // この「～でない」を引き算で、「または」をユニオンで表している。
            let mut spaces = vec![];
            for t in 0..first_args.len() {
                let mut args = vec![];

                for i in 0..first_args.len() {
                    args.push(if i == t {
                        space_subtraction(first_args[i].clone(), second_args[i].clone(), td)
                    } else {
                        first_args[i].clone()
                    });
                }

                spaces.push(Space::Constructor {
                    name: name.to_string(),
                    args,
                });
            }
            Space::new_union(spaces)
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
