pub(crate) mod ast;
pub(crate) mod ast_gen;
pub(crate) mod element;
pub(crate) mod keyword;
pub(crate) mod node;
pub(crate) mod parse;
pub(crate) mod parse_context;
pub(crate) mod parse_error;
pub(crate) mod parse_exprs;
pub(crate) mod parse_pats;
pub(crate) mod parse_stmts;
pub(crate) mod pun;
pub(crate) mod source;
pub(crate) mod text_cursor;
pub(crate) mod token;
pub(crate) mod tokenize;
pub(crate) mod tokenize_context;
pub(crate) mod tokenize_rules;
pub(crate) mod trivia;

pub(crate) use ast::Ast;
pub(crate) use element::Element;
pub(crate) use node::{token_range_map::TokenRangeMap, Node, NodeData};
pub(crate) use parse::parse;
pub(crate) use parse_error::ParseError;
pub(crate) use text_cursor::TextCursor;
pub(crate) use token::{Token, TokenData};
pub(crate) use tokenize::tokenize;
pub(crate) use trivia::Trivia;

/// 行番号と列番号で表されるテキスト上の位置。(1 から始まる。)
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct TextPosition {
    /// 1-based index.
    line: usize,

    /// 1-based index.
    character: usize,
}

impl TextPosition {
    pub(crate) fn new(line: usize, character: usize) -> Self {
        assert!(line >= 1);
        assert!(character >= 1);

        TextPosition { line, character }
    }

    pub(crate) fn line(&self) -> usize {
        self.line
    }

    pub(crate) fn character(&self) -> usize {
        self.character
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct TextRange {
    start: TextPosition,
    end: TextPosition,
}

impl TextRange {
    pub(crate) fn new(start: TextPosition, end: TextPosition) -> Self {
        TextRange { start, end }
    }

    pub(crate) fn start(&self) -> TextPosition {
        self.start
    }

    pub(crate) fn end(&self) -> TextPosition {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{self, Write};
    use std::path::PathBuf;
    use std::rc::Rc;

    fn indent(depth: usize) -> &'static str {
        const SPACES: &str = "                                ";
        &SPACES[0..depth * 4]
    }

    fn go(token: &TokenData, indent: &str, out: &mut Vec<u8>) -> io::Result<()> {
        let leading_indent = format!("{} v", indent);

        for trivia in token.leading() {
            match trivia {
                Trivia::Token(token) => {
                    go(token, &leading_indent, out)?;
                }
                Trivia::Error(error) => {
                    write!(out, "{}{:?}\n", &leading_indent, error)?;
                }
            }
        }

        write!(out, "{}{:?} {:?}\n", indent, token.token(), token.text())?;

        let trailing_indent = format!("{} ^", indent);

        for trivia in token.trailing() {
            match trivia {
                Trivia::Token(token) => {
                    go(token, &trailing_indent, out)?;
                }
                Trivia::Error(error) => {
                    write!(out, "{}{:?}\n", &trailing_indent, error).unwrap();
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_tokenize() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tests_dir = root_dir.join("./tests");

        let source_code = fs::read_to_string(&tests_dir.join("tokenize.pmxclang")).unwrap();

        let tokens = tokenize(Rc::new(source_code));

        let mut snapshot = vec![];
        for token in tokens.iter() {
            go(token, "", &mut snapshot).unwrap();
        }

        fs::write(&tests_dir.join("tokenize_snapshot.txt"), snapshot).unwrap();
    }

    pub(crate) fn snapshot_node(node: &NodeData, w: &mut Vec<u8>) -> io::Result<()> {
        fn on_token(token: &TokenData, depth: usize, w: &mut Vec<u8>) -> io::Result<()> {
            if !token.leading().is_empty() {
                write!(w, "{}v [\n", indent(depth))?;
                for trivia in token.leading() {
                    on_trivia(trivia, depth + 1, w)?;
                }
                write!(w, "{}]\n", indent(depth))?;
            }

            write!(
                w,
                "{}T({:?}) {:?}\n",
                indent(depth),
                token.token(),
                token.text()
            )?;

            if !token.trailing().is_empty() {
                write!(w, "{}^ [\n", indent(depth))?;
                for trivia in token.trailing() {
                    on_trivia(trivia, depth + 1, w)?;
                }
                write!(w, "{}]\n", indent(depth))?;
            }

            Ok(())
        }

        fn on_trivia(trivia: &Trivia, depth: usize, w: &mut Vec<u8>) -> io::Result<()> {
            match trivia {
                Trivia::Token(token) => on_token(token, depth, w)?,
                Trivia::Error(_) => {}
            }

            Ok(())
        }

        fn on_node(node: &NodeData, depth: usize, w: &mut Vec<u8>) -> io::Result<()> {
            write!(w, "{}N({:?}) [\n", indent(depth), node.node())?;

            for child in node.children() {
                on_element(child, depth + 1, w)?;
            }

            write!(w, "{}]\n", indent(depth))?;

            Ok(())
        }

        fn on_element(element: &Element, depth: usize, w: &mut Vec<u8>) -> io::Result<()> {
            match element {
                Element::Token(token) => on_token(token, depth, w)?,
                Element::Error(error) => {
                    write!(w, "{}E({:?})\n", indent(depth), error)?;
                }
                Element::Node(node) => on_node(node, depth, w)?,
            }

            Ok(())
        }

        on_node(node, 0, w)
    }

    #[test]
    fn test_parse() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tests_dir = root_dir.join("./tests");

        let source_code = fs::read_to_string(&tests_dir.join("parse.pmxclang")).unwrap();

        let root = parse(Rc::new(source_code));

        let mut cursor = TextCursor::default();
        let mut errors = vec![];
        parse::collect_errors(&root, &mut cursor, &mut errors);

        let mut snapshot = vec![];
        snapshot_node(&root, &mut snapshot).unwrap();

        if !errors.is_empty() {
            write!(snapshot, "\n{:#?}\n", errors).unwrap();
        }

        fs::write(&tests_dir.join("parse_snapshot.txt"), snapshot).unwrap();
    }

    #[test]
    fn test_ast() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tests_dir = root_dir.join("./tests");

        let source_code = fs::read_to_string(&tests_dir.join("ast.pmxclang")).unwrap();

        let root = Rc::new(parse(Rc::new(source_code)));
        let ast = ast_gen::gen_root(root);

        let mut snapshot = vec![];
        write!(snapshot, "{:#?}\n", ast).unwrap();

        fs::write(&tests_dir.join("ast_snapshot.txt"), snapshot).unwrap();
    }
}
