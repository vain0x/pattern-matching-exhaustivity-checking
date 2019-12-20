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
    use crate::syntax::{
        self, CtorExpr, CtorPat, EnumDecl, Expr, MatchStmt, Node, Pat, Root, Stmt, TextRange,
        Token, TokenRangeMap,
    };

    pub(crate) struct MatchExhaustivityModel {
        ty_database: TyDatabase,
        match_expressions: Vec<(MatchExpression, TextRange)>,
        token_range_map: TokenRangeMap,
        pub(crate) errors: Vec<(TextRange, String)>,
    }

    fn resolve_ty(ty: &syntax::Ty, m: &mut MatchExhaustivityModel) -> Option<Ty> {
        let ty_name = ty.name_opt.as_ref()?;

        if m.ty_database.find_enum_definition(&ty_name).is_none() {
            let range = ty
                .node
                .first_node(|node| node.node() == Node::Name)
                .and_then(|name| {
                    name.first_token(|token| token.token() == Token::Ident)
                        .and_then(|token| m.token_range_map.get(&token))
                })
                .cloned()
                .unwrap_or_default();
            m.errors.push((range, "定義されていません".to_string()));
        }

        Some(Ty::Enum {
            name: ty_name.to_string(),
        })
    }

    fn analyze_pat(pat: &Pat, ty: &Ty, m: &mut MatchExhaustivityModel) -> Option<Pattern> {
        match pat {
            Pat::Discard(..) => Some(Pattern::Discard { ty: ty.clone() }),
            Pat::Ctor(CtorPat {
                name_opt: Some(ref name),
                ref tuple_opt,
                ref node,
            }) => {
                let constructor_definition =
                    match (m.ty_database.find_constructor_by_name(name), ty) {
                        (Some((enum_name, constructor_definition)), Ty::Enum { ref name })
                            if enum_name == name =>
                        {
                            constructor_definition
                        }
                        (constructor_opt, _) => {
                            let message = if constructor_opt.is_some() {
                                "型が異なります"
                            } else {
                                "定義されていません"
                            };
                            // FIXME: パターンが K のときと K(..) のときで字句の見つけ方が異なる。
                            let range = node
                                .first_token(|token| token.token() == Token::Ident)
                                .and_then(|token| m.token_range_map.get(&token))
                                .or_else(|| {
                                    node.first_node(|node| node.node() == Node::Name).and_then(
                                        |name| {
                                            name.first_token(|token| token.token() == Token::Ident)
                                                .and_then(|token| m.token_range_map.get(&token))
                                        },
                                    )
                                })
                                .cloned()
                                .unwrap_or_default();
                            m.errors.push((range, message.to_string()));
                            return None;
                        }
                    };

                let arity = constructor_definition.arg_tys.len();
                let given_arity = tuple_opt.as_ref().map_or(0, |t| t.len());
                if arity != given_arity {
                    let range = node
                        .first_token(|token| token.token() == Token::Ident)
                        .and_then(|token| m.token_range_map.get(&token))
                        .or_else(|| {
                            node.first_node(|node| node.node() == Node::Name)
                                .and_then(|name| {
                                    name.first_token(|token| token.token() == Token::Ident)
                                        .and_then(|token| m.token_range_map.get(&token))
                                })
                        })
                        .cloned()
                        .unwrap_or_default();
                    m.errors.push((
                        range,
                        format!("引数の数が異なります ({} → {})", given_arity, arity,),
                    ));
                    return None;
                }

                let mut args = vec![];
                if let Some(field_pats) = tuple_opt.as_ref() {
                    let arg_tys = constructor_definition.arg_tys.clone();

                    for (field_pat, ty) in field_pats.iter().zip(arg_tys) {
                        let arg_pat = analyze_pat(&field_pat, &ty, m)
                            .unwrap_or_else(|| Pattern::Discard { ty });
                        args.push(arg_pat);
                    }
                }

                Some(Pattern::Constructor {
                    name: name.to_string(),
                    args,
                    ty: ty.clone(),
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
                ..
            }) => {
                let (enum_name, _) = match m.ty_database.find_constructor_by_name(name) {
                    None => {
                        let range = node
                            .first_token(|token| token.token() == Token::Ident)
                            .and_then(|token| m.token_range_map.get(&token))
                            .cloned()
                            .unwrap_or_default();
                        m.errors.push((range, "定義されていません".to_string()));
                        return None;
                    }
                    Some(t) => t,
                };

                // FIXME: 引数を型検査

                Some(Ty::Enum {
                    name: enum_name.to_string(),
                })
            }
            _ => None,
        }
    }

    fn analyze_match_arm(
        arm: &syntax::MatchArm,
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
                // 自己参照のために型だけ定義する。
                let i = m.ty_database.definitions.len();
                m.ty_database.definitions.push(TyDefinition::Enum {
                    name: name.to_string(),
                    constructors: vec![],
                });

                let constructors = ctors
                    .iter()
                    .filter_map(|ctor| {
                        let mut arg_tys = vec![];
                        if let Some(tuple_decl) = ctor.tuple_decl_opt.as_ref() {
                            arg_tys = tuple_decl
                                .fields
                                .iter()
                                .map(|ty| {
                                    resolve_ty(ty, m).unwrap_or_else(|| Ty::Enum {
                                        name: "???".to_string(),
                                    })
                                })
                                .collect();
                        }

                        Some(ConstructorDefinition {
                            name: ctor.name_opt.as_ref()?.to_string(),
                            arg_tys,
                        })
                    })
                    .collect();

                m.ty_database.definitions[i] = TyDefinition::Enum {
                    name: name.to_string(),
                    constructors,
                };
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
        let mut model = lower::from_ast(&ast, token_range_map);
        lower::check(&mut model);

        let mut snapshot = vec![];
        write!(snapshot, "{:#?}\n", model.errors).unwrap();

        fs::write(&tests_dir.join("check_snapshot.txt"), snapshot).unwrap();
    }
}
