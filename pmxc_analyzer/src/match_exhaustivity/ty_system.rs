//! サンプル言語の型システム

// NOTE: type は Rust の予約語なので ty と略す。

/// 式の型
#[derive(Clone, Debug)]
pub(crate) enum Ty {
    /// enum の1つのコンストラクタが表す型。
    /// 例えば true 型など。
    Constructor {
        name: String,
    },
    Enum {
        name: String,
    },
}

impl Ty {
    /// この型は名前が `constructor_name` のコンストラクタ型か？
    pub(crate) fn is_constructor_of_name(&self, constructor_name: &str) -> bool {
        match self {
            Ty::Constructor { ref name } => name == constructor_name,
            _ => false,
        }
    }
}

impl Default for Ty {
    fn default() -> Ty {
        Ty::Constructor {
            name: String::new(),
        }
    }
}

/// 型定義
pub(crate) enum TyDefinition {
    Enum {
        name: String,
        constructors: Vec<ConstructorDefinition>,
    },
}

/// コンストラクタ定義
pub(crate) struct ConstructorDefinition {
    pub(crate) name: String,
}

/// 型に関する知識を提供する。
#[derive(Default)]
pub(crate) struct TyDatabase {
    pub(crate) definitions: Vec<TyDefinition>,
}

impl TyDatabase {
    pub(crate) fn find_enum_definition(&self, enum_name: &str) -> Option<&[ConstructorDefinition]> {
        self.definitions
            .iter()
            .filter_map(|d| match d {
                TyDefinition::Enum {
                    ref name,
                    ref constructors,
                } if name == enum_name => Some(constructors.as_slice()),
                _ => None,
            })
            .next()
    }

    pub(crate) fn find_constructor_definition(
        &self,
        constructor_name: &str,
    ) -> Option<&ConstructorDefinition> {
        self.definitions
            .iter()
            .filter_map(|d| match d {
                TyDefinition::Enum {
                    ref constructors, ..
                } => constructors
                    .iter()
                    .filter_map(|k| {
                        if k.name == constructor_name {
                            Some(k)
                        } else {
                            None
                        }
                    })
                    .next(),
            })
            .next()
    }

    pub(crate) fn find_constructor_by_name(
        &self,
        constructor_name: &str,
    ) -> Option<(&str, &ConstructorDefinition)> {
        self.definitions
            .iter()
            .filter_map(|d| match d {
                TyDefinition::Enum {
                    ref name,
                    ref constructors,
                } => constructors
                    .iter()
                    .filter_map(|k| {
                        if k.name == constructor_name {
                            Some((name.as_str(), k))
                        } else {
                            None
                        }
                    })
                    .next(),
            })
            .next()
    }

    /// subty が super_ty の部分型であるか？
    pub(crate) fn is_subtype_of(&self, subty: &Ty, super_ty: &Ty) -> bool {
        match super_ty {
            Ty::Enum { ref name } => self.is_subtype_of_enum(subty, name),
            Ty::Constructor { ref name } => self.is_subtype_of_constructor(subty, name),
        }
    }

    /// 型 subty が enum 型の部分型か？
    ///
    /// - 同じ enum 型なら OK
    /// - subty がその enum 型に含まれるコンストラクタのコンストラクタ型なら OK
    /// - それ以外は NG
    fn is_subtype_of_enum(&self, subty: &Ty, enum_name: &str) -> bool {
        match subty {
            Ty::Enum { ref name } => name == enum_name,
            Ty::Constructor { ref name } => {
                let constructor_definitions = match self.find_enum_definition(enum_name) {
                    Some(x) => x,
                    None => return false,
                };
                constructor_definitions
                    .iter()
                    .any(|kd| kd.name.as_str() == name)
            }
        }
    }

    /// 型 subty がコンストラクタ型の部分型か？
    ///
    /// - 同じコンストラクタ型なら OK
    ///     - なお、ここではコンストラクタの名前は重複しないものとする。
    /// - それ以外なら NG
    pub(crate) fn is_subtype_of_constructor(&self, subty: &Ty, constructor_name: &str) -> bool {
        match subty {
            Ty::Constructor { ref name } => name == constructor_name,
            _ => false,
        }
    }
}
