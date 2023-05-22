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
    println!("gefunden!");
    let token_slice = lex.slice();
    println!("{:?}",token_slice);
    let last_quote = token_slice.rfind('"').unwrap();
    println!("{:?}",last_quote);
    let (url, text) = token_slice.split_at(last_quote + 1);  // +1 to exclude the quote itself from the URL
    let url = url.trim_end_matches('"'); // remove ending quote from URL
    println!("{:?}",url);
    let text = text.trim_start_matches(' '); // remove leading whitespace from text
    println!("{:?}",text);

    (LinkUrl(url.parse().unwrap()), LinkText(text.parse().unwrap()))
}