//! 型からスペースを作る機能
//!
//! これは型システムに強く依存する。
//! ここでは通常の関数として定義しているが、トレイトやインターフェイスやモジュールを使って、API として渡す方がいい。

use super::*;

pub(crate) fn space_from_ty(ty: Ty) -> Space {
    Space::Ty(ty)
}

pub(crate) fn space_can_decompose(space: &Space, td: &TyDatabase) -> bool {
    match space {
        Space::Ty(Ty::Enum { ref name }) => td.find_enum_definition(name).is_some(),
        Space::Ty(Ty::Constructor { ref name }) => td.find_constructor_definition(name).is_some(),
        _ => false,
    }
}

pub(crate) fn space_decompose(space: Space, td: &TyDatabase) -> Space {
    assert!(space_can_decompose(&space, td));

    match space {
        Space::Ty(Ty::Enum { ref name }) => {
            let constructor_definitions = match td.find_enum_definition(name) {
                Some(x) => x,
                None => unreachable!(),
            };

            Space::new_union(constructor_definitions.iter().map(|kd| {
                space_from_ty(Ty::Constructor {
                    name: kd.name.to_string(),
                })
            }))
        }
        Space::Ty(Ty::Constructor { ref name }) => {
            let constructor_definition = match td.find_constructor_definition(name) {
                Some(x) => x,
                None => unreachable!(),
            };

            Space::Constructor {
                name: constructor_definition.name.to_string(),
                args: constructor_definition
                    .arg_tys
                    .iter()
                    .map(|arg_ty| space_from_ty(arg_ty.clone()))
                    .collect(),
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_ty_database() -> TyDatabase {
        let mut td = TyDatabase::default();

        td.definitions.push(TyDefinition::Enum {
            name: "Boolean".to_string(),
            constructors: vec![
                ConstructorDefinition {
                    name: "False".to_string(),
                    arg_tys: vec![],
                },
                ConstructorDefinition {
                    name: "True".to_string(),
                    arg_tys: vec![],
                },
            ],
        });

        td
    }

    #[test]
    fn test_decompose_boolean() {
        let td = new_ty_database();

        let boolean_ty = Ty::Enum {
            name: "Boolean".to_string(),
        };
        let ty_space = space_from_ty(boolean_ty);
        let decomposed_space = space_decompose(ty_space, &td);

        let spaces = match decomposed_space {
            Space::Union(spaces) => spaces,
            _ => unreachable!("ユニオンスペースのはず"),
        };

        let mut constructor_names = spaces
            .iter()
            .map(|s| match s {
                Space::Ty(Ty::Constructor { name }) => name.as_str(),
                _ => unreachable!("コンストラクタ型スペースのはず"),
            })
            .collect::<Vec<_>>();

        constructor_names.sort();
        assert_eq!(constructor_names, vec!["False", "True"]);
    }

    #[test]
    fn test_decompose_true() {
        let td = new_ty_database();

        let true_ty = Ty::Constructor {
            name: "True".to_string(),
        };
        let ty_space = space_from_ty(true_ty);
        let decomposed_space = space_decompose(ty_space, &td);

        let name = match decomposed_space {
            Space::Constructor { ref name, .. } => name.as_str(),
            _ => unreachable!("コンストラクタスペースのはず"),
        };
        assert_eq!(name, "True");
    }
}
