pub(crate) mod expressions;
pub(crate) mod helpers;
pub(crate) mod patterns;
pub(crate) mod space;
pub(crate) mod space_from_pattern;
pub(crate) mod space_from_ty;
pub(crate) mod space_intersection;
pub(crate) mod space_subtraction;
pub(crate) mod space_to_pattern;
pub(crate) mod ty_system;
pub(crate) mod use_cases;

pub(crate) use expressions::{MatchArm, MatchExpression};
pub(crate) use patterns::Pattern;
pub(crate) use space::Space;
pub(crate) use ty_system::{ConstructorDefinition, Ty, TyDatabase, TyDefinition};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{self, ast_gen, parse};
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use std::rc::Rc;

    #[test]
    pub(crate) fn test_snapshot() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tests_dir = root_dir.join("./tests");

        let source_code = fs::read_to_string(&tests_dir.join("check.pmxclang")).unwrap();

        let root = Rc::new(parse::parse(Rc::new(source_code)));
        let token_range_map = syntax::TokenRangeMap::new(&root);

        let ast = ast_gen::gen_root(root);
        let mut model = helpers::lower::from_ast(&ast, token_range_map);
        helpers::lower::check(&mut model);

        let mut snapshot = vec![];
        write!(snapshot, "{:#?}\n", model.errors).unwrap();

        fs::write(&tests_dir.join("check_snapshot.txt"), snapshot).unwrap();
    }
}
