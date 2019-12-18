use super::space_from_pattern::space_from_pattern;
use super::space_from_ty::space_from_ty;
use super::space_subtraction::space_subtraction;
use super::*;

pub(crate) fn is_exhaustive(expression: &MatchExpression, td: &TyDatabase) -> bool {
    // 条件式が作るスペース。
    let ty_space = space_from_ty(expression.condition_ty.clone());

    // アームのパターンを | でつないだパターンのスペース。
    let pat_space = Space::new_union(
        expression
            .arms
            .iter()
            .map(|arm| space_from_pattern(arm.pattern.clone(), expression.condition_ty.clone())),
    );

    // 条件式のスペースからアーム全体のスペースを引く。
    let leaked_space = space_subtraction(ty_space, pat_space, td);

    // スペースが残らなければ網羅的といえる。
    leaked_space.is_empty()
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
                },
                ConstructorDefinition {
                    name: "True".to_string(),
                },
            ],
        });

        td
    }

    #[test]
    fn test_boolean_exhaustive_by_enumeration() {
        let td = new_ty_database();

        // match bool_value { true => {}, false => {} }
        let match_expression = MatchExpression {
            condition_ty: Ty::Enum {
                name: "Boolean".to_string(),
            },
            arms: vec![
                MatchArm {
                    pattern: Pattern::Constructor {
                        name: "True".to_string(),
                    },
                },
                MatchArm {
                    pattern: Pattern::Constructor {
                        name: "False".to_string(),
                    },
                },
            ],
        };

        assert!(is_exhaustive(&match_expression, &td));
    }

    #[test]
    fn test_boolean_exhaustive_by_discard() {
        let td = new_ty_database();

        // match bool_value { true => {}, _ => {} }
        let match_expression = MatchExpression {
            condition_ty: Ty::Enum {
                name: "Boolean".to_string(),
            },
            arms: vec![
                MatchArm {
                    pattern: Pattern::Constructor {
                        name: "True".to_string(),
                    },
                },
                MatchArm {
                    pattern: Pattern::Discard,
                },
            ],
        };

        assert!(is_exhaustive(&match_expression, &td));
    }

    #[test]
    fn test_boolean_nonexhaustive_leaking_false() {
        let td = new_ty_database();

        // match bool_value { true => {} }
        let match_expression = MatchExpression {
            condition_ty: Ty::Enum {
                name: "Boolean".to_string(),
            },
            arms: vec![MatchArm {
                pattern: Pattern::Constructor {
                    name: "True".to_string(),
                },
            }],
        };

        assert!(!is_exhaustive(&match_expression, &td));
    }
}
