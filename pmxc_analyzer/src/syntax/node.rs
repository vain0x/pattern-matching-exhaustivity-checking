use super::*;
use std::fmt::{self, Debug};

///　構文ノードの種類
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Node {
    Name,
    NumberLiteral,
    Group,
    StructLiteral,
    StructBody,
    StructField,
    ExprStmt,
    LetStmt,
    MatchStmt,
    MatchArm,
    EnumDecl,
    StructDecl,
    /// K or K(...) or K {...}
    CtorDecl,
    Root,
    NotSpecified,
}

/// 構文ノードのデータ
pub(crate) struct NodeData {
    node: Node,
    children: Vec<Element>,
}

impl NodeData {
    pub(crate) fn new() -> Self {
        NodeData {
            node: Node::NotSpecified,
            children: vec![],
        }
    }

    pub(crate) fn new_before(child: NodeData) -> Self {
        let mut parent = NodeData::new();
        parent.push_node(child);
        parent
    }

    pub(crate) fn node(&self) -> Node {
        self.node
    }

    pub(crate) fn set_node(mut self, node: Node) -> Self {
        assert_eq!(self.node, Node::NotSpecified);
        assert_ne!(node, Node::NotSpecified);

        self.node = node;
        self
    }

    pub(crate) fn children(&self) -> &[Element] {
        &self.children
    }

    pub(crate) fn push_token(&mut self, token: TokenData) {
        self.children.push(token.into())
    }

    pub(crate) fn push_error(&mut self, error: ParseError) {
        self.children.push(error.into())
    }

    pub(crate) fn push_node(&mut self, node: NodeData) {
        assert_ne!(node.node(), Node::NotSpecified);

        self.children.push(node.into())
    }
}

impl Debug for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NodeData(..)")
    }
}

pub(crate) mod token_range_map {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    pub(crate) struct TokenRangeMap {
        map: HashMap<usize, TextRange>,
        cursor: TextCursor,
    }

    impl TokenRangeMap {
        fn get_key(token: &TokenData) -> usize {
            token as *const TokenData as usize
        }

        pub(crate) fn new(node: &NodeData) -> Self {
            let mut map = Self::default();
            map.on_node(&node);
            map
        }

        fn on_token(&mut self, token: &TokenData) {
            for trivia in token.leading() {
                self.on_trivia(trivia);
            }

            let key = Self::get_key(token);
            let start = self.cursor.current();

            self.cursor.advance(token.text());

            let end = self.cursor.current();
            self.map.insert(key, TextRange::new(start, end));

            for trivia in token.trailing() {
                self.on_trivia(trivia);
            }
        }

        fn on_trivia(&mut self, trivia: &Trivia) {
            match trivia {
                Trivia::Token(token) => self.on_token(token),
                Trivia::Error(_) => {}
            }
        }

        fn on_node(&mut self, node: &NodeData) {
            for element in node.children() {
                self.on_element(element);
            }
        }

        fn on_element(&mut self, element: &Element) {
            match element {
                Element::Token(token) => {
                    self.on_token(token);
                }
                Element::Error(_) => {}
                Element::Node(node) => {
                    self.on_node(&node);
                }
            }
        }

        pub(crate) fn get(&self, key: &TokenData) -> Option<&TextRange> {
            self.map.get(&Self::get_key(key))
        }
    }
}
