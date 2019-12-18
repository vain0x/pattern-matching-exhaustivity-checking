use super::*;

/// スペース
#[derive(Clone, Debug)]
pub(crate) enum Space {
    /// コンストラクタ
    Constructor {
        name: String,
    },

    Union(Vec<Space>),

    Ty(Ty),
}

impl Space {
    pub(crate) fn new_empty() -> Space {
        Space::Union(vec![])
    }

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

        if union.len() == 1 {
            union.into_iter().next().unwrap()
        } else {
            Space::Union(union)
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        match self {
            Space::Constructor { .. } => false,
            Space::Ty(..) => {
                // FIXME: コンストラクタを持たない enum 型なら true
                false
            }
            Space::Union(union) => union.iter().all(|space| space.is_empty()),
        }
    }
}
