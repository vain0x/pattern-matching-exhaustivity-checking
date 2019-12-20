use super::parse_context::ParseContext;
use super::parse_exprs::*;
use super::parse_pats::*;
use super::*;

impl Token {
    pub(crate) fn is_stmt_first(self) -> bool {
        self == Token::Match || self == Token::Enum || self.is_expr_first()
    }
}

fn parse_name(p: &mut ParseContext) -> NodeData {
    let mut node = NodeData::new();
    p.eat(&mut node, Token::Ident);
    node.set_node(Node::Name)
}

fn parse_tuple_decl(p: &mut ParseContext) -> Option<NodeData> {
    if p.next() != Token::LeftParen {
        return None;
    }

    let mut node = NodeData::new();
    p.bump(&mut node);

    let mut boundary = true;
    while p.next() == Token::Ident {
        if !boundary && !p.at_eol() {
            node.push_error(ParseError::ExpectedCommaOrEol);
        }

        let mut field = NodeData::new();
        let ty = parse_name(p);
        field.push_node(ty);
        node.push_node(field.set_node(Node::TupleFieldDecl));

        boundary = p.eat(&mut node, Token::Comma);
    }

    if !p.eat(&mut node, Token::RightParen) {
        node.push_error(ParseError::ExpectedRightParen);
    }

    Some(node.set_node(Node::TupleDecl))
}

pub(crate) fn parse_ctor_decl(p: &mut ParseContext) -> Option<NodeData> {
    if p.next() != Token::Ident {
        return None;
    }

    let mut node = NodeData::new();
    p.bump(&mut node);

    if let Some(tuple_decl) = parse_tuple_decl(p) {
        node.push_node(tuple_decl);
    }

    Some(node.set_node(Node::CtorDecl))
}

fn parse_match_arm(p: &mut ParseContext) -> Option<NodeData> {
    let pat = parse_pat(p)?;

    let mut node = NodeData::new_before(pat);

    if !p.eat_puns(&mut node, &[Token::Equal, Token::RightAngle]) {
        node.push_error(ParseError::ExpectedFatArrow);
    }

    if !p.eat(&mut node, Token::LeftBrace) {
        node.push_error(ParseError::ExpectedLeftBrace);
    }

    if !p.eat(&mut node, Token::RightBrace) {
        node.push_error(ParseError::ExpectedRightBrace);
    }

    Some(node.set_node(Node::MatchArm))
}

pub(crate) fn parse_stmt(p: &mut ParseContext) -> Option<NodeData> {
    match p.next() {
        Token::Match => {
            let mut node = NodeData::new();
            p.bump(&mut node);

            if let Some(cond) = parse_cond(p) {
                node.push_node(cond);
            } else {
                node.push_error(ParseError::ExpectedExpr);
            }

            if !p.eat(&mut node, Token::LeftBrace) {
                node.push_error(ParseError::ExpectedLeftBrace);
            }

            while let Some(arm) = parse_match_arm(p) {
                node.push_node(arm);
            }

            if !p.eat(&mut node, Token::RightBrace) {
                node.push_error(ParseError::ExpectedRightBrace);
            }

            Some(node.set_node(Node::MatchStmt))
        }
        Token::Enum => {
            let mut node = NodeData::new();
            p.bump(&mut node);

            if !p.eat(&mut node, Token::Ident) {
                node.push_error(ParseError::ExpectedIdent);
            }

            if !p.eat(&mut node, Token::LeftBrace) {
                node.push_error(ParseError::ExpectedLeftBrace);
            }

            while let Some(ctor_decl) = parse_ctor_decl(p) {
                node.push_node(ctor_decl);

                if !p.eat(&mut node, Token::Comma) && !p.at_eol() {
                    node.push_error(ParseError::ExpectedCommaOrEol);
                }
            }

            if !p.eat(&mut node, Token::RightBrace) {
                node.push_error(ParseError::ExpectedRightBrace);
            }

            Some(node.set_node(Node::EnumDecl))
        }
        _ => {
            if let Some(expr) = parse_expr(p) {
                let node = NodeData::new_before(expr);
                return Some(node.set_node(Node::ExprStmt));
            }

            debug_assert!(!p.next().is_stmt_first());
            None
        }
    }
}

pub(crate) fn parse_root(p: &mut ParseContext) -> NodeData {
    let mut root = NodeData::new();

    while !p.at_eof() {
        if let Some(stmt) = parse_stmt(p) {
            root.push_node(stmt);
        } else {
            p.bump(&mut root);
            root.push_error(ParseError::ExpectedExpr);

            // エラー回復
            while !p.at_eof() && !p.next().is_stmt_first() {
                p.bump(&mut root);
            }
        }
    }

    root.set_node(Node::Root)
}
