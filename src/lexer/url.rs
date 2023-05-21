use logos::{Lexer, Logos, Source};
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
    #[regex(r#"<a[ \n]*[^>]*href\s*=\s*""#, extract_link_info)]
    LinkStart,

    #[regex(r#"">"#)]
    LinkEnd,

    #[regex(r"[ \t\n\f\r]+", logos::skip)] // Ignore whitespaces
    Ignored,

    #[error]
    Error,
}


/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    let token_slice = lex.slice();

    let last_quote = token_slice.rfind('"').unwrap();
    let (url, text) = token_slice.split_at(last_quote + 1);  // +1 to exclude the quote itself from the URL
    let url = url.trim_end_matches('"'); // remove ending quote from URL
    let text = text.trim_start_matches(' '); // remove leading whitespace from text

    (LinkUrl(url.into()), LinkText(text.into()))
}
