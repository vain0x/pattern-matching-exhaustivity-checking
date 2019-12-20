//! スペースからパターンを作る機能
//!
//! パターンマッチから漏れているパターンを提示するのに使う。

use super::*;

fn constructor_to_pattern(
    name: String,
    constructor_definition: &ConstructorDefinition,
    td: &TyDatabase,
) -> Option<Pattern> {
    let args = constructor_definition
        .arg_tys
        .iter()
        .map(|arg_ty| ty_to_pattern(arg_ty, td))
        .collect::<Option<Vec<_>>>()?;
    Some(Pattern::Constructor { name, args })
}

fn ty_to_pattern(ty: &Ty, td: &TyDatabase) -> Option<Pattern> {
    match ty {
        Ty::Constructor { name } => {
            let (_, constructor_definition) = td.find_constructor_by_name(name)?;
            constructor_to_pattern(name.to_string(), constructor_definition, td)
        }
        Ty::Enum { name } => {
            let constructor_definitions = td.find_enum_definition(&name)?;
            let constructor_definition = constructor_definitions.iter().next()?;
            constructor_to_pattern(
                constructor_definition.name.to_string(),
                constructor_definition,
                td,
            )
        }
    }
}

pub(crate) fn space_to_pattern(space: Space, td: &TyDatabase) -> Option<Pattern> {
    match space {
        Space::Constructor { name, args } => {
            let args = args
                .into_iter()
                .map(|arg_space| space_to_pattern(arg_space, td))
                .collect::<Option<Vec<_>>>()?;
            Some(Pattern::Constructor { name, args })
        }
        Space::Union(spaces) => spaces
            .into_iter()
            .next()
            .and_then(|space| space_to_pattern(space, td)),

        Space::Ty(ty) => ty_to_pattern(&ty, td),
    }
}
