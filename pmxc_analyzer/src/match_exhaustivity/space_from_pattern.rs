use super::*;

pub(crate) fn space_from_pattern(pattern: Pattern, ty: Ty) -> Space {
    match pattern {
        Pattern::Discard => Space::Ty(ty),
        Pattern::Constructor { name } => Space::Constructor { name },
    }
}
