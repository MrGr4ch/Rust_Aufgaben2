use logos::{Lexer, Logos};
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    #[regex(r#"<a[ \n\t\f\r]*[^>]*href\s*=\s*"([^"]*)"[^>]*>([^<]*)</a[ \n\t\f\r]*>"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    #[regex(r"[ \t\n\f\r]+", logos::skip)] // Ignore whitespaces
    Ignored,

    #[error]
    Error,
}


/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    println!("extract_link_info ausgeführt");
    let caps: Vec<_> = lex.slice().match_indices('"').collect();

    let url_start = caps[0].0 + 1;  // start after first quote
    let url_end = caps[1].0;  // end at second quote
    let url = &lex.slice()[url_start..url_end];

    let text_start = caps[1].0 + 2;  // start after the second quote and a '>'
    let text_end = lex.slice().rfind('<').unwrap();  // end at the last '<'
    let text = &lex.slice()[text_start..text_end];

    (LinkUrl(url.to_string()), LinkText(text.to_string()))
}