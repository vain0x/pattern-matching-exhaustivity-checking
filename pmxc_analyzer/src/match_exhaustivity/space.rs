use super::*;

/// スペース。「値の集合」を表現するもの。
#[derive(Clone, Debug)]
pub(crate) enum Space {
    /// コンストラクタスペース。コンストラクタ型の値の集合を表現している。
    Constructor { name: String, args: Vec<Space> },

    /// ユニオンスペース。和集合を表現している。
    Union(Vec<Space>),

    /// 型スペース。型の値の集合を表現している。
    /// 「分解」することによりコンストラクタスペースのユニオンに展開できる可能性がある。
    Ty(Ty),
}

impl Space {
    /// 空のスペースを作る。
    pub(crate) fn new_empty() -> Space {
        Space::Union(vec![])
    }

    /// スペースのリストからユニオンスペースを組み立てる。
    pub(crate) fn new_union(spaces: impl IntoIterator<Item = Space>) -> Space {
        fn flatten(space: Space, spaces: &mut Vec<Space>) {
            match space {
                Space::Constructor { .. } => spaces.push(space),
                Space::Ty(..) => spaces.push(space),
                Space::Union(union) => {
                    for space in union {
                        flatten(space, spaces);
                    }
                }
            }
        }

        let mut union = vec![];
        for space in spaces {
            flatten(space, &mut union);
        }

        // 1つのスペースからなるユニオンという無駄な構造は避けて、そのスペースを返す。
        if union.len() == 1 {
            return union.into_iter().next().unwrap();
        }

        Space::Union(union)
    }

    /// スペースが空か？
    ///
    /// スペースが表現している「値の集合」が空集合かを検査する。
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            // FIXME: 引数のいずれかが「空」なら true
            Space::Constructor { .. } => false,
            Space::Ty(..) => {
                // FIXME: コンストラクタを持たない enum 型なら true
                false
            }
            Space::Union(union) => union.iter().all(Space::is_empty),
        }
    }
}
