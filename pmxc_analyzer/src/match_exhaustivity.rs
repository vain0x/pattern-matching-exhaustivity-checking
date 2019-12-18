pub(crate) mod expressions;
pub(crate) mod patterns;
pub(crate) mod space;
pub(crate) mod space_from_pattern;
pub(crate) mod space_from_ty;
pub(crate) mod space_intersection;
pub(crate) mod space_subtraction;
pub(crate) mod ty_system;
pub(crate) mod use_cases;

pub(crate) use expressions::{MatchArm, MatchExpression};
pub(crate) use patterns::Pattern;
pub(crate) use space::Space;
pub(crate) use ty_system::{ConstructorDefinition, Ty, TyDatabase, TyDefinition};

/// 抽象構文木から網羅性検査用の中間表現を生成する。
/// (網羅性検査アルゴリズムとは無関係。)
pub(crate) mod lower {
    use super::*;
    use crate::syntax::*;

    pub(crate) struct MatchExhaustivityModel {
        ty_database: TyDatabase,
        match_expressions: Vec<(MatchExpression, TextRange)>,
        token_range_map: TokenRangeMap,
        pub(crate) errors: Vec<(TextRange, String)>,
    }

    fn analyze_pat(ast: &Ast, ty: &Ty, m: &mut MatchExhaustivityModel) -> Option<Pattern> {
        match ast {
            Ast::DiscardPat { .. } => Some(Pattern::Discard),
            Ast::CtorPat {
                name_opt: Some(ref name),
                ..
            } => {
                match (m.ty_database.find_constructor_by_name(name), ty) {
                    (Some((enum_name, _)), Ty::Enum { ref name }) if enum_name == name => {}
                    _ => return None,
                }
                Some(Pattern::Constructor {
                    name: name.to_string(),
                })
            }
            _ => None,
        }
    }

    fn analyze_expr(ast: &Ast, m: &MatchExhaustivityModel) -> Option<Ty> {
        match ast {
            Ast::CtorExpr {
                name_opt: Some(ref name),
                ..
            } => {
                let (enum_name, _) = m.ty_database.find_constructor_by_name(name)?;
                Some(Ty::Enum {
                    name: enum_name.to_string(),
                })
            }
            _ => None,
        }
    }

    fn analyze_match_arm(ast: &Ast, ty: &Ty, m: &mut MatchExhaustivityModel) -> Option<MatchArm> {
        match ast {
            Ast::MatchArm {
                pat_opt: Some(ref pat),
                ..
            } => {
                let pattern = analyze_pat(pat, ty, m)?;
                Some(MatchArm { pattern })
            }
            _ => None,
        }
    }

    fn analyze_stmt(ast: &Ast, m: &mut MatchExhaustivityModel) {
        match ast {
            Ast::MatchStmt {
                cond_opt: Some(ref cond),
                ref arms,
                ref node,
            } => {
                let cond_ty = match analyze_expr(cond, m) {
                    Some(ty) => ty,
                    None => return,
                };

                let arms = arms
                    .iter()
                    .filter_map(|arm| analyze_match_arm(arm, &cond_ty, m))
                    .collect();

                let range = node
                    .first_token(|token| token.token() == Token::Match)
                    .and_then(|token| m.token_range_map.get(token))
                    .cloned()
                    .unwrap_or_default();

                m.match_expressions.push((
                    MatchExpression {
                        condition_ty: cond_ty,
                        arms,
                    },
                    range,
                ));
            }
            Ast::EnumDecl {
                name_opt: Some(ref name),
                ctors,
                ..
            } => {
                let constructors = ctors
                    .iter()
                    .filter_map(|ctor| match ctor {
                        Ast::CtorDecl {
                            name_opt: Some(ref name),
                            ..
                        } => Some(ConstructorDefinition {
                            name: name.to_string(),
                        }),
                        _ => None,
                    })
                    .collect();

                m.ty_database.definitions.push(TyDefinition::Enum {
                    name: name.to_string(),
                    constructors,
                });
            }
            Ast::Root { stmts, .. } => {
                for stmt in stmts {
                    analyze_stmt(stmt, m);
                }
            }
            _ => {
                // unexpected
            }
        }
    }

    pub(crate) fn from_ast(ast: &Ast, token_range_map: TokenRangeMap) -> MatchExhaustivityModel {
        let mut m = MatchExhaustivityModel {
            ty_database: TyDatabase {
                definitions: vec![],
            },
            match_expressions: vec![],
            token_range_map,
            errors: vec![],
        };

        analyze_stmt(ast, &mut m);

        m
    }

    pub(crate) fn check(model: &mut MatchExhaustivityModel) {
        for i in 0..model.match_expressions.len() {
            let (match_expression, range) = &model.match_expressions[i];
            if !use_cases::is_exhaustive(match_expression, &model.ty_database) {
                let range = range.clone();
                model
                    .errors
                    .push((range, "網羅的ではありません".to_string()));
            }
        }
    }
}
