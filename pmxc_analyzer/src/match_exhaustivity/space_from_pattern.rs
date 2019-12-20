use super::*;

pub(crate) fn space_from_pattern(pattern: Pattern) -> Space {
    match pattern {
        Pattern::Discard { ty } => Space::Ty(ty),
        Pattern::Constructor { name, args, .. } => Space::Constructor {
            name,
            args: args.into_iter().map(space_from_pattern).collect(),
        },
    }
}
