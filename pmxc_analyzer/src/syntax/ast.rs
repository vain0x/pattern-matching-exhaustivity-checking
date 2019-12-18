use super::*;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) enum Ast {
    DiscardPat {
        node: Rc<NodeData>,
    },
    CtorPat {
        name_opt: Option<String>,
        node: Rc<NodeData>,
    },
    CtorExpr {
        name_opt: Option<String>,
        node: Rc<NodeData>,
    },
    MatchStmt {
        cond_opt: Option<Box<Ast>>,
        arms: Vec<Ast>,
        node: Rc<NodeData>,
    },
    MatchArm {
        pat_opt: Option<Box<Ast>>,
        node: Rc<NodeData>,
    },
    EnumDecl {
        name_opt: Option<String>,
        ctors: Vec<Ast>,
        node: Rc<NodeData>,
    },
    CtorDecl {
        name_opt: Option<String>,
        node: Rc<NodeData>,
    },
    Root {
        stmts: Vec<Ast>,
        node: Rc<NodeData>,
    },
}
