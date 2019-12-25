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
        self.on_token(trivia.as_token())
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
