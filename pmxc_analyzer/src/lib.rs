mod match_exhaustivity;
mod syntax;

use monaco::*;
use std::rc::Rc;
use syntax::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub(crate) mod monaco {
    use serde::Serialize;

    pub mod editor {
        use super::*;

        #[derive(Serialize)]
        #[allow(unused)]
        pub enum MarkerSeverity {
            Hint = 1,
            Info = 2,
            Warning = 4,
            Error = 8,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct MarkerData {
            // code?: string;
            pub severity: MarkerSeverity,
            pub message: String,
            // source?: string;
            /// 1-based index
            pub start_line_number: usize,
            pub start_column: usize,
            pub end_line_number: usize,
            pub end_column: usize,
            // relatedInformation?: IRelatedInformation[];
            // tags?: MarkerTag[];
        }
    }

    pub mod languages {
        use super::*;

        #[derive(Serialize)]
        pub struct State;

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Token {
            pub start_index: usize,
            pub scopes: String,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct LineTokens {
            pub tokens: Vec<Token>,
            pub end_state: State,
        }
    }
}

#[wasm_bindgen]
pub fn tokenize(source_code: String) -> JsValue {
    let token_indices = syntax::tokenize::tokenize_with_utf16_indices(Rc::new(source_code));

    let tokens = token_indices
        .iter()
        .map(|&(token, start_index)| {
            let scopes = match token {
                Token::Comment => "comment",
                _ if token.is_control_keyword() => "keyword.control",
                _ if token.is_keyword() => "keyword",
                Token::Other => "invalid",
                _ => "none",
            };

            languages::Token {
                start_index,
                scopes: scopes.to_string(),
            }
        })
        .collect();

    let line_tokens = languages::LineTokens {
        tokens,
        end_state: languages::State,
    };

    JsValue::from_serde(&line_tokens).unwrap()
}

#[wasm_bindgen]
pub fn validate(source_code: String) -> JsValue {
    let root = syntax::parse::parse(Rc::new(source_code));

    let mut cursor = syntax::TextCursor::default();
    let mut errors = vec![];
    syntax::parse::collect_errors(&root, &mut cursor, &mut errors);

    let markers = errors
        .into_iter()
        .map(|(range, message)| editor::MarkerData {
            severity: editor::MarkerSeverity::Error,
            message,
            start_line_number: range.start().line(),
            start_column: range.start().character(),
            end_line_number: range.end().line(),
            end_column: range.end().character(),
        })
        .collect::<Vec<editor::MarkerData>>();
    if !markers.is_empty() {
        return JsValue::from_serde(&markers).unwrap();
    }

    let root = Rc::new(root);
    let token_range_map = syntax::TokenRangeMap::new(&root);
    let ast = syntax::ast_gen::gen_root(root);
    let mut model = match_exhaustivity::helpers::lower::from_ast(&ast, token_range_map);
    match_exhaustivity::helpers::lower::check(&mut model);

    let markers = model
        .errors
        .into_iter()
        .map(|(range, message)| editor::MarkerData {
            severity: editor::MarkerSeverity::Error,
            message,
            start_line_number: range.start().line(),
            start_column: range.start().character(),
            end_line_number: range.end().line(),
            end_column: range.end().character(),
        })
        .collect::<Vec<editor::MarkerData>>();

    JsValue::from_serde(&markers).unwrap()
}
