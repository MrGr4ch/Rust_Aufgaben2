se logos::{Lexer, Logos, Source};
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
    #[regex(r#"<a[ \n]*[^>]*href\s*=\s*"([^"]*)"[^>]*>([^<]*)</a>"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    #[regex(r"[ \t\n\f\r]+", logos::skip)] // Ignore whitespaces
    Ignored,

    #[error]
    Error,
}


/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    let caps = lex.extras.captures.as_ref().unwrap(); // Get the captures

    let url = caps.get(1).unwrap().as_str(); // Get the URL
    let text = caps.get(2).unwrap().as_str(); // Get the link text

    (LinkUrl(url.into()), LinkText(text.into()))
}
