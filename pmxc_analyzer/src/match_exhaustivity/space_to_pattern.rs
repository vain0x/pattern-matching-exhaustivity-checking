//! スペースからパターンを作る機能
//!
//! パターンマッチから漏れているパターンを提示するのに使う。

use super::*;

/// コンストラクタ定義からパターンを構成する。
fn constructor_to_pattern(name: String, constructor_definition: &ConstructorDefinition) -> Pattern {
    let args = constructor_definition
        .arg_tys
        .iter()
        .map(|arg_ty| Pattern::Discard { ty: arg_ty.clone() })
        .collect::<Vec<_>>();
    Pattern::Constructor { name, args }
}

/// 型の値の例として、パターンを1つ構成する。
fn ty_to_pattern(ty: &Ty, td: &TyDatabase) -> Option<Pattern> {
    match ty {
        Ty::Constructor { name } => {
            let (_, constructor_definition) = td.find_constructor_by_name(name)?;
            Some(constructor_to_pattern(
                name.to_string(),
                constructor_definition,
            ))
        }
        Ty::Enum { name } => {
            let constructor_definitions = td.find_enum_definition(&name)?;
            let constructor_definition = constructor_definitions.iter().next()?;
            Some(constructor_to_pattern(
                constructor_definition.name.to_string(),
                constructor_definition,
            ))
        }
    }
}

/// スペースから、そのスペースに含まれるパターンを1つ例示する。
pub(crate) fn space_to_pattern(space: Space, td: &TyDatabase) -> Option<Pattern> {
    match space {
        Space::Constructor { name, args } => {
            // すべての引数からパターンの例示が取れたら、それらを引数とするコンストラクタパターンを作る。
            let args = args
                .into_iter()
                .map(|arg_space| space_to_pattern(arg_space, td))
                .collect::<Option<Vec<_>>>()?;
            Some(Pattern::Constructor { name, args })
        }
        Space::Union(spaces) => spaces
            .into_iter()
            .filter_map(|space| space_to_pattern(space, td))
            .next(),
        Space::Ty(ty) => ty_to_pattern(&ty, td),
    }
}
