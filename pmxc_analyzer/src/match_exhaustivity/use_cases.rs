use super::space_from_pattern::space_from_pattern;
use super::space_from_ty::space_from_ty;
use super::space_subtraction::space_subtraction;
use super::space_to_pattern::space_to_pattern;
use super::*;

/// 1つの match 式の網羅性検査を行う。
pub(crate) fn check_exhaustivity(
    expression: &MatchExpression,
    td: &TyDatabase,
) -> (bool, Option<Pattern>) {
    // 条件式が作るスペース。
    let ty_space = space_from_ty(expression.condition_ty.clone());

    // アームのパターンを | でつないだパターンのスペース。
    let pat_space = Space::new_union(
        expression
            .arms
            .iter()
            .map(|arm| space_from_pattern(arm.pattern.clone())),
    );

    // 条件式のスペースからアーム全体のスペースを引く。
    let leaked_space = space_subtraction(ty_space, pat_space, td);

    // スペースが残らなければ網羅的といえる。
    let ok = leaked_space.is_empty();

    // マッチしないケースの例を1つ構築する。
    let leaked_pattern = space_to_pattern(leaked_space, td);

    (ok, leaked_pattern)
}

/// match 式が網羅的か？
pub(crate) fn is_exhaustive(expression: &MatchExpression, td: &TyDatabase) -> bool {
    let (ok, _) = check_exhaustivity(expression, td);
    ok
}

// -----------------------------------------------
// テスト
// -----------------------------------------------

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
    fn test_boolean_exhaustive_by_enumeration() {
        let td = new_ty_database();
        let boolean_ty = Ty::Enum {
            name: "Boolean".to_string(),
        };

        // match bool_value { true => {}, false => {} }
        let match_expression = MatchExpression {
            condition_ty: boolean_ty,
            arms: vec![
                MatchArm {
                    pattern: Pattern::Constructor {
                        name: "True".to_string(),
                        args: vec![],
                    },
                },
                MatchArm {
                    pattern: Pattern::Constructor {
                        name: "False".to_string(),
                        args: vec![],
                    },
                },
            ],
        };

        assert!(is_exhaustive(&match_expression, &td));
    }

    #[test]
    fn test_boolean_exhaustive_by_discard() {
        let td = new_ty_database();
        let boolean_ty = Ty::Enum {
            name: "Boolean".to_string(),
        };

        // match bool_value { true => {}, _ => {} }
        let match_expression = MatchExpression {
            condition_ty: boolean_ty.clone(),
            arms: vec![
                MatchArm {
                    pattern: Pattern::Constructor {
                        name: "True".to_string(),
                        args: vec![],
                    },
                },
                MatchArm {
                    pattern: Pattern::Discard {
                        ty: boolean_ty.clone(),
                    },
                },
            ],
        };

        assert!(is_exhaustive(&match_expression, &td));
    }

    #[test]
    fn test_boolean_nonexhaustive_leaking_false() {
        let td = new_ty_database();
        let boolean_ty = Ty::Enum {
            name: "Boolean".to_string(),
        };

        // match bool_value { true => {} }
        let match_expression = MatchExpression {
            condition_ty: boolean_ty.clone(),
            arms: vec![MatchArm {
                pattern: Pattern::Constructor {
                    name: "True".to_string(),
                    args: vec![],
                },
            }],
        };

        assert!(!is_exhaustive(&match_expression, &td));
    }
}
