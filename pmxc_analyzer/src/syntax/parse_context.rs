//! 構文解析の状態管理

use super::*;
use std::rc::Rc;

type TokenIndex = usize;

type TokenList = Rc<[TokenData]>;

pub(crate) struct ParseContext {
    tokens: TokenList,
    index: TokenIndex,
}

impl ParseContext {
    pub(crate) fn new(tokens: TokenList) -> Self {
        ParseContext { tokens, index: 0 }
    }

    pub(crate) fn assert_invariants(&self) {
        assert!(self.index <= self.tokens.len());
    }

    pub(crate) fn at_eof(&self) -> bool {
        self.next() == Token::Eof
    }

    /// 次の字句との間に改行があるか？
    pub(crate) fn at_eol(&self) -> bool {
        if self.index >= self.tokens.len() {
            return true;
        }

        self.tokens[self.index].contains_eol()
    }

    fn nth(&self, offset: usize) -> Option<&TokenData> {
        self.tokens.get(self.index + offset)
    }

    pub(crate) fn next(&self) -> Token {
        self.nth(0).map_or(Token::Eof, |token| token.token())
    }

    /// 次の字句を読み進める。
    pub(crate) fn bump(&mut self, node: &mut NodeData) {
        assert!(self.index + 1 <= self.tokens.len());

        let token = &self.tokens[self.index];

        node.push_token(token.clone());

        self.index += 1;
        self.assert_invariants();
    }

    /// 次の字句が指定された種類なら読み進める。
    pub(crate) fn eat(&mut self, node: &mut NodeData, token: Token) -> bool {
        if self.next() == token {
            self.bump(node);
            true
        } else {
            false
        }
    }

    /// 指定された種類の字句が並んでいて、間に空白等がなければ読み進める。
    pub(crate) fn eat_puns<'a>(
        &'a mut self,
        node: &'a mut NodeData,
        tokens: impl IntoIterator<Item = &'a Token>,
    ) -> bool {
        let mut len = 0;

        for (i, token) in tokens.into_iter().enumerate() {
            len += 1;

            if !self.nth(i).map_or(false, |t| t.token() == *token) {
                return false;
            }

            if i >= 1 && !self.nth(i - 1).map_or(false, |t| t.trailing().is_empty()) {
                return false;
            }

            if i >= 1 && !self.nth(i).map_or(false, |t| t.leading().is_empty()) {
                return false;
            }
        }

        for _ in 0..len {
            self.bump(node);
        }
        true
    }

    pub(crate) fn finish(mut self, root: &mut NodeData) {
        assert_eq!(self.index, self.tokens.len() - 1);
        assert_eq!(root.node(), Node::Root);

        self.bump(root);
    }
}
