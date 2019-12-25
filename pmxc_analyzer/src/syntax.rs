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
pub(crate) mod text_position;
pub(crate) mod text_range;
pub(crate) mod token;
pub(crate) mod token_range_map;
pub(crate) mod tokenize;
pub(crate) mod tokenize_context;
pub(crate) mod tokenize_rules;
pub(crate) mod trivia;

pub(crate) use ast::*;
pub(crate) use element::Element;
pub(crate) use node::{Node, NodeData};
pub(crate) use parse_error::ParseError;
pub(crate) use text_cursor::TextCursor;
pub(crate) use text_position::TextPosition;
pub(crate) use text_range::TextRange;
pub(crate) use token::{Token, TokenData};
pub(crate) use token_range_map::TokenRangeMap;
pub(crate) use trivia::Trivia;

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

    #[test]
    fn test_tokenize() {
        fn write_token(token: &TokenData, prefix: &str, out: &mut Vec<u8>) -> io::Result<()> {
            write!(out, "{}{:?} {:?}\n", prefix, token.token(), token.text())
        }

        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tests_dir = root_dir.join("./tests");

        let source_code = fs::read_to_string(&tests_dir.join("tokenize.pmxclang")).unwrap();

        let tokens = tokenize::tokenize(Rc::new(source_code));

        let mut snapshot = vec![];
        for token in tokens.iter() {
            for trivia in token.leading() {
                write_token(trivia.as_token(), " v", &mut snapshot).unwrap();
            }

            write_token(token, "", &mut snapshot).unwrap();

            for trivia in token.trailing() {
                write_token(trivia.as_token(), " ^", &mut snapshot).unwrap();
            }
        }

        fs::write(&tests_dir.join("tokenize_snapshot.txt"), snapshot).unwrap();
    }

    pub(crate) fn snapshot_node(node: &NodeData, w: &mut Vec<u8>) -> io::Result<()> {
        fn on_token(token: &TokenData, depth: usize, w: &mut Vec<u8>) -> io::Result<()> {
            if !token.leading().is_empty() {
                write!(w, "{}v [\n", indent(depth))?;
                for trivia in token.leading() {
                    on_token(trivia.as_token(), depth + 1, w)?;
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
                    on_token(trivia.as_token(), depth + 1, w)?;
                }
                write!(w, "{}]\n", indent(depth))?;
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
                Element::Trivia(trivia) => on_token(trivia.as_token(), depth, w)?,
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

        let root = parse::parse(Rc::new(source_code));

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

        let root = Rc::new(parse::parse(Rc::new(source_code)));
        let ast = ast_gen::gen_root(root);

        let mut snapshot = vec![];
        write!(snapshot, "{:#?}\n", ast).unwrap();

        fs::write(&tests_dir.join("ast_snapshot.txt"), snapshot).unwrap();
    }
}
