use super::parse_context::ParseContext;
use super::parse_stmts::parse_root;
use super::*;
use std::rc::Rc;

pub(crate) fn parse_tokens(tokens: Rc<[TokenData]>) -> NodeData {
    let mut p = ParseContext::new(tokens);
    let mut root = parse_root(&mut p);
    p.finish(&mut root);
    root
}

pub(crate) fn parse(source_code: Rc<String>) -> NodeData {
    let tokens = Rc::from(tokenize::tokenize(source_code));
    parse_tokens(tokens)
}

pub(crate) fn collect_errors(
    node: &NodeData,
    cursor: &mut TextCursor,
    errors: &mut Vec<(TextRange, String)>,
) {
    fn on_token(token: &TokenData, cursor: &mut TextCursor, errors: &mut Vec<(TextRange, String)>) {
        for trivia in token.leading() {
            let start = cursor.current();
            let range = TextRange::new(start, start);
            on_trivia(trivia, range, cursor, errors);
        }

        let start = cursor.current();
        cursor.advance(token.text());
        let end = cursor.current();
        let range = TextRange::new(start, end);

        for trivia in token.trailing() {
            // let start = cursor.current();
            // let range = TextRange::new(start, start);
            on_trivia(trivia, range, cursor, errors);
        }
    }

    fn on_error(error: ParseError, range: TextRange, errors: &mut Vec<(TextRange, String)>) {
        errors.push((range, format!("{:?}", error)));
    }

    fn on_trivia(
        trivia: &Trivia,
        parent_range: TextRange,
        cursor: &mut TextCursor,
        errors: &mut Vec<(TextRange, String)>,
    ) {
        match trivia {
            Trivia::Token(token) => on_token(token, cursor, errors),
            Trivia::Error(error) => on_error(*error, parent_range, errors),
        }
    }

    fn on_node(node: &NodeData, cursor: &mut TextCursor, errors: &mut Vec<(TextRange, String)>) {
        for child in node.children() {
            on_element(child, cursor, errors);
        }
    }

    fn on_element(
        element: &Element,
        cursor: &mut TextCursor,
        errors: &mut Vec<(TextRange, String)>,
    ) {
        match element {
            Element::Token(token) => on_token(token, cursor, errors),
            Element::Error(error) => {
                let start = cursor.current();
                let range = TextRange::new(start, start);
                on_error(*error, range, errors);
            }
            Element::Node(node) => on_node(node, cursor, errors),
        }
    }

    on_node(node, cursor, errors);
}
