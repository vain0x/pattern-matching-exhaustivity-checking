use super::parse_context::ParseContext;
use super::*;

impl Token {
    pub(crate) fn is_pat_atom_first(self) -> bool {
        self == Token::Number
            || self == Token::Ident
            || self == Token::LeftParen
            || self == Token::Underscore
    }

    pub(crate) fn is_pat_first(self) -> bool {
        self.is_pat_atom_first()
    }
}

pub(crate) fn parse_pat_atom(p: &mut ParseContext) -> Option<NodeData> {
    match p.next() {
        Token::Number => {
            let mut node = NodeData::new();
            p.bump(&mut node);
            Some(node.set_node(Node::NumberLiteral))
        }
        Token::Ident => {
            let mut node = NodeData::new();
            p.bump(&mut node);
            Some(node.set_node(Node::Name))
        }
        Token::Underscore => {
            let mut node = NodeData::new();
            p.bump(&mut node);
            Some(node.set_node(Node::Name))
        }
        Token::LeftParen => {
            let mut node = NodeData::new();
            p.bump(&mut node);

            if let Some(body) = parse_pat(p) {
                node.push_node(body);
            } else {
                node.push_error(ParseError::ExpectedExpr);
            }

            if p.next() == Token::RightParen {
                p.bump(&mut node);
            } else {
                node.push_error(ParseError::ExpectedRightParen);
            }

            Some(node.set_node(Node::Group))
        }
        _ => {
            debug_assert!(!p.next().is_pat_atom_first());
            None
        }
    }
}

pub(crate) fn parse_pat_call(p: &mut ParseContext) -> Option<NodeData> {
    let mut callee = parse_pat_atom(p)?;

    if p.next() == Token::LeftParen {
        // FIXME: callee が識別子でなければ構文エラー
        let mut node = NodeData::new_before(callee);
        p.bump(&mut node);

        while let Some(arg) = parse_pat(p) {
            let arg = NodeData::new_before(arg);
            node.push_node(arg.set_node(Node::Argument));

            p.eat(&mut node, Token::Comma);
        }

        if !p.eat(&mut node, Token::RightParen) {
            node.push_error(ParseError::ExpectedRightParen);
        }

        callee = node.set_node(Node::Call);
    }

    Some(callee)
}

pub(crate) fn parse_pat(p: &mut ParseContext) -> Option<NodeData> {
    parse_pat_call(p)
}
