//! 抽象構文木

use super::*;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct Ty {
    pub(crate) name_opt: Option<String>,
    pub(crate) node: Rc<NodeData>,
}

#[derive(Debug)]
pub(crate) struct DiscardPat {
    pub(crate) node: Rc<NodeData>,
}

#[derive(Debug)]
pub(crate) struct CtorPat {
    pub(crate) name_opt: Option<String>,
    pub(crate) tuple_opt: Option<Vec<Pat>>,
    pub(crate) node: Rc<NodeData>,
}

#[derive(Debug)]
pub(crate) enum Pat {
    Discard(DiscardPat),
    Ctor(CtorPat),
}

#[derive(Debug)]
pub(crate) struct CtorExpr {
    pub(crate) name_opt: Option<String>,
    pub(crate) tuple_opt: Option<Vec<Expr>>,
    pub(crate) node: Rc<NodeData>,
}

#[derive(Debug)]
pub(crate) enum Expr {
    Ctor(CtorExpr),
}

#[derive(Debug)]
pub(crate) struct MatchArm {
    pub(crate) pat_opt: Option<Pat>,
    pub(crate) node: Rc<NodeData>,
}

#[derive(Debug)]
pub(crate) struct MatchStmt {
    pub(crate) cond_opt: Option<Expr>,
    pub(crate) arms: Vec<MatchArm>,
    pub(crate) node: Rc<NodeData>,
}

#[derive(Debug)]
pub(crate) struct TupleDecl {
    pub(crate) fields: Vec<Ty>,
    pub(crate) node: Rc<NodeData>,
}

#[derive(Debug)]
pub(crate) struct CtorDecl {
    pub(crate) name_opt: Option<String>,
    pub(crate) tuple_decl_opt: Option<TupleDecl>,
    pub(crate) node: Rc<NodeData>,
}

#[derive(Debug)]
pub(crate) struct EnumDecl {
    pub(crate) name_opt: Option<String>,
    pub(crate) ctors: Vec<CtorDecl>,
    pub(crate) node: Rc<NodeData>,
}

#[derive(Debug)]
pub(crate) enum Stmt {
    Match(MatchStmt),
    Enum(EnumDecl),
}

#[derive(Debug)]
pub(crate) struct Root {
    pub(crate) stmts: Vec<Stmt>,
    pub(crate) node: Rc<NodeData>,
}
