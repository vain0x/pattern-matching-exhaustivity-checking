//! 具象構文木 → 抽象構文木

use super::*;
use std::rc::Rc;

impl Node {
    pub(crate) fn is_pat(self) -> bool {
        self == Node::Name || self == Node::Group
    }

    pub(crate) fn is_expr(self) -> bool {
        self == Node::Name || self == Node::Group
    }

    pub(crate) fn is_stmt(self) -> bool {
        self == Node::MatchStmt || self == Node::EnumDecl
    }
}

impl NodeData {
    pub(crate) fn first_token<P: Fn(&TokenData) -> bool>(&self, pred: P) -> Option<&TokenData> {
        self.children()
            .iter()
            .filter_map(|child| match child {
                Element::Token(token) if pred(token) => Some(token),
                _ => None,
            })
            .next()
    }

    pub(crate) fn first_node<P: Fn(&NodeData) -> bool>(&self, pred: P) -> Option<Rc<NodeData>> {
        self.children()
            .iter()
            .filter_map(|child| match child {
                Element::Node(child) if pred(child) => Some(Rc::clone(child)),
                _ => None,
            })
            .next()
    }

    pub(crate) fn filter_node<P: Fn(&NodeData) -> bool>(&self, pred: P) -> Vec<Rc<NodeData>> {
        self.children()
            .iter()
            .filter_map(|child| match child {
                Element::Node(child) if pred(child) => Some(Rc::clone(child)),
                _ => None,
            })
            .collect()
    }

    pub(crate) fn first_ident(&self) -> Option<String> {
        self.first_token(|token| token.token() == Token::Ident)
            .map(|token| token.text().to_string())
    }
}

fn gen_pat(node: Rc<NodeData>) -> Option<Pat> {
    assert!(node.node().is_pat());

    match node.node() {
        Node::Name => {
            if node
                .first_token(|token| token.token() == Token::Underscore)
                .is_some()
            {
                return Some(Pat::Discard(DiscardPat { node }));
            }

            let name_opt = node.first_ident();
            Some(Pat::Ctor(CtorPat { name_opt, node }))
        }
        Node::Group => node
            .first_node(|child| child.node().is_pat())
            .and_then(gen_pat),
        _ => None,
    }
}

fn gen_expr(node: Rc<NodeData>) -> Option<Expr> {
    assert!(node.node().is_expr());

    match node.node() {
        Node::Name => {
            let name_opt = node.first_ident();
            Some(Expr::Ctor(CtorExpr { name_opt, node }))
        }
        Node::Group => node
            .first_node(|child| child.node().is_expr())
            .and_then(gen_expr),
        _ => None,
    }
}

fn gen_match_arm(node: Rc<NodeData>) -> Option<MatchArm> {
    assert_eq!(node.node(), Node::MatchArm);

    let pat_opt = node
        .first_node(|child| child.node().is_pat())
        .and_then(gen_pat);

    Some(MatchArm { pat_opt, node })
}

fn gen_match_stmt(node: Rc<NodeData>) -> Option<MatchStmt> {
    let cond_opt = node
        .first_node(|child| child.node().is_expr())
        .and_then(gen_expr);

    let arms = node
        .filter_node(|child| child.node() == Node::MatchArm)
        .into_iter()
        .filter_map(|child| gen_match_arm(child))
        .collect();

    Some(MatchStmt {
        cond_opt,
        arms,
        node,
    })
}

fn gen_enum_decl(node: Rc<NodeData>) -> Option<EnumDecl> {
    if node.node() != Node::EnumDecl {
        return None;
    }

    let name_opt = node.first_ident();

    let ctors = node
        .filter_node(|child| child.node() == Node::CtorDecl)
        .into_iter()
        .filter_map(|child| gen_ctor_decl(child))
        .collect();

    Some(EnumDecl {
        name_opt,
        ctors,
        node,
    })
}

fn gen_ctor_decl(node: Rc<NodeData>) -> Option<CtorDecl> {
    if node.node() != Node::CtorDecl {
        return None;
    }

    let name_opt = node.first_ident();

    Some(CtorDecl { name_opt, node })
}

fn gen_stmt(node: Rc<NodeData>) -> Option<Stmt> {
    match node.node() {
        Node::MatchStmt => gen_match_stmt(node).map(Stmt::Match),
        Node::EnumDecl => gen_enum_decl(node).map(Stmt::Enum),
        _ => {
            assert!(!node.node().is_stmt());
            None
        }
    }
}

fn gen_stmts(node: Rc<NodeData>) -> Vec<Stmt> {
    node.filter_node(|child| child.node().is_stmt())
        .into_iter()
        .filter_map(|child| gen_stmt(child))
        .collect()
}

pub(crate) fn gen_root(node: Rc<NodeData>) -> Root {
    assert_eq!(node.node(), Node::Root);

    let stmts = gen_stmts(node.clone());
    Root { stmts, node }
}
