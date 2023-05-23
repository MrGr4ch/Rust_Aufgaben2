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
    #[regex(r#"<a[ \n\t\f\r]*[^h]*href="([^"]*)"[^>]*>([^<]*)</a[ \n\t\f\r]*>"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    #[regex(r"[ \t\n\f\r]+|[^<]+|<[^a]|<a[ \n\t\f\r]*[^h]*", logos::skip)] // Ignore whitespaces
    Ignored,

    #[error]
    Error,
}


/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    println!("extract_link_info ausgeführt");
    let caps = lex.slice().match_indices('"').collect::<Vec<_>>();
    /*for cap in &caps{
        println!("({},{})", cap.0, cap.1);
    }*/
    let url;
    let text;

    if lex.slice().find("name") < lex.slice().find("href") {
        let url_start = caps[2].0 + 1;  // start after first quote
        let url_end = caps[3].0;  // end at second quote
        url = &lex.slice()[url_start..url_end];
    } else {
    let url_start = caps[0].0 + 1;  // start after first quote
    let url_end = caps[1].0;  // end at second quote
    url = &lex.slice()[url_start..url_end];
    }
    println!("url_done, url = {}", url);

    let close = lex.slice().match_indices('>').collect::<Vec<_>>();
    let open = lex.slice().match_indices('<').collect::<Vec<_>>();

    /*println!("printing close");
    for clo in &close{
        println!("({},{})", clo.0, clo.1);
    }
    println!("printing open");
    for ope in &open{
        println!("({},{})", ope.0, ope.1);
    }*/

    let text_start = close[close.len()-2].0 + 1;  // start after first quote
    let text_end = open[open.len()-1].0;  // end at second quote
    text = &lex.slice()[text_start..text_end];

    println!("text_done, text = {}", text);

    println!("URl = {}, Text = {}", url, text);
    (LinkUrl(url.to_string()), LinkText(text.to_string()))
}