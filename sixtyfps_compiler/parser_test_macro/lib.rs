extern crate proc_macro;
use core::iter::IntoIterator;
use core::str::FromStr;
use proc_macro::{TokenStream, TokenTree};

fn error(e: &str) -> String {
    format!("::core::compile_error!{{\"{}\"}}", e)
}

fn generate_test(fn_name: &str, doc: &str) -> String {
    if fn_name.is_empty() {
        return error("Could not parse function name");
    }

    if doc.is_empty() {
        return error("doc comments not found");
    }
    let idx = match doc.find("```test\n") {
        Some(idx) => idx,
        None => return error("test not found"),
    };
    let doc = &doc[(idx + 8)..];
    let idx = match doc.find("```\n") {
        Some(idx) => idx,
        None => return error("end of test not found"),
    };
    let doc = &doc[..idx];

    let mut tests = String::new();
    for (i, line) in doc.split("\n").enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        tests += &format!(r#"
        #[test] fn parser_test_{fn}_{i}()
        {{
            let mut p = Parser::new("{source}");
            {fn}(&mut p);
            assert_eq!(p.diags.inner, Vec::new());
        }}
        "#, fn = fn_name, i = i, source = line)
    }
    tests
}

#[proc_macro_attribute]
pub fn parser_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut result = item.clone(); // The original function

    let mut doc = String::new();
    let mut item = item.into_iter();

    let mut fn_name = String::new();

    // Extract the doc comment.
    // Bail out once we find a token that does not fit the doc comment pattern
    loop {
        match item.next() {
            Some(TokenTree::Punct(p)) => {
                if p.as_char() != '#' {
                    break;
                }
            }
            Some(TokenTree::Ident(i)) => {
                if i.to_string() == "fn" {
                    fn_name = item.next().map_or_else(String::default, |i| i.to_string());
                }
                break;
            }
            _ => break,
        }
        if let Some(TokenTree::Group(g)) = item.next() {
            if g.delimiter() != proc_macro::Delimiter::Bracket {
                break;
            }
            let mut attr = g.stream().into_iter();
            if let Some(TokenTree::Ident(i)) = attr.next() {
                if i.to_string() != "doc" {
                    break;
                }
            } else {
                break;
            }
            if let Some(TokenTree::Punct(p)) = attr.next() {
                if p.as_char() != '=' {
                    break;
                }
            } else {
                break;
            }
            if let Some(TokenTree::Literal(lit)) = attr.next() {
                let s = lit.to_string();
                doc += s.trim_matches('"');
                doc += "\n";
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if fn_name.is_empty() {
        while let Some(tt) = item.next() {
            if tt.to_string() == "fn" {
                fn_name = item.next().map_or_else(String::default, |i| i.to_string());
                break;
            }
        }
    }

    let test_function = TokenStream::from_str(&generate_test(&fn_name, &doc)).unwrap_or_else(|e| {
        TokenStream::from_str(&error(&format!("Lex error in generated test: {:?}", e))).unwrap()
    });

    result.extend(test_function);
    result
}