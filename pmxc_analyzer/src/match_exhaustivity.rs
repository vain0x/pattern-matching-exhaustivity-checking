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

    fn analyze_pat(pat: &Pat, ty: &Ty, m: &mut MatchExhaustivityModel) -> Option<Pattern> {
        match pat {
            Pat::Discard(..) => Some(Pattern::Discard),
            Pat::Ctor(CtorPat {
                name_opt: Some(ref name),
                ref node,
            }) => {
                // 型検査
                match (m.ty_database.find_constructor_by_name(name), ty) {
                    (Some((enum_name, _)), Ty::Enum { ref name }) if enum_name == name => {}
                    (constructor_opt, _) => {
                        let message = if constructor_opt.is_some() {
                            "型が異なります"
                        } else {
                            "定義されていません"
                        };
                        let range = node
                            .first_token(|token| token.token() == Token::Ident)
                            .and_then(|token| m.token_range_map.get(&token))
                            .cloned()
                            .unwrap_or_default();
                        m.errors.push((range, message.to_string()));
                        return None;
                    }
                }

                Some(Pattern::Constructor {
                    name: name.to_string(),
                })
            }
            _ => None,
        }
    }

    fn analyze_expr(expr: &Expr, m: &mut MatchExhaustivityModel) -> Option<Ty> {
        match expr {
            Expr::Ctor(CtorExpr {
                name_opt: Some(ref name),
                ref node,
            }) => {
                // FIXME: 未定義の名前はエラーにする
                let enum_name = match m.ty_database.find_constructor_by_name(name) {
                    None => {
                        let range = node
                            .first_token(|token| token.token() == Token::Ident)
                            .and_then(|token| m.token_range_map.get(&token))
                            .cloned()
                            .unwrap_or_default();
                        m.errors.push((range, "定義されていません".to_string()));
                        return None;
                    }
                    Some((enum_name, _)) => enum_name,
                };

                Some(Ty::Enum {
                    name: enum_name.to_string(),
                })
            }
            _ => None,
        }
    }

    fn analyze_match_arm(
        arm: &ast::MatchArm,
        ty: &Ty,
        m: &mut MatchExhaustivityModel,
    ) -> Option<expressions::MatchArm> {
        let pat = arm.pat_opt.as_ref()?;
        let pattern = analyze_pat(pat, ty, m)?;
        Some(expressions::MatchArm { pattern })
    }

    fn analyze_stmt(stmt: &Stmt, m: &mut MatchExhaustivityModel) {
        match stmt {
            Stmt::Match(MatchStmt {
                cond_opt: Some(ref cond),
                ref arms,
                ref node,
            }) => {
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
            Stmt::Enum(EnumDecl {
                name_opt: Some(ref name),
                ctors,
                ..
            }) => {
                let constructors = ctors
                    .iter()
                    .filter_map(|ctor| {
                        Some(ConstructorDefinition {
                            name: ctor.name_opt.as_ref()?.to_string(),
                        })
                    })
                    .collect();

                m.ty_database.definitions.push(TyDefinition::Enum {
                    name: name.to_string(),
                    constructors,
                });
            }
            _ => {}
        }
    }

    pub(crate) fn from_ast(root: &Root, token_range_map: TokenRangeMap) -> MatchExhaustivityModel {
        let mut m = MatchExhaustivityModel {
            ty_database: TyDatabase {
                definitions: vec![],
            },
            match_expressions: vec![],
            token_range_map,
            errors: vec![],
        };

        for stmt in root.stmts.iter() {
            analyze_stmt(stmt, &mut m);
        }

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
