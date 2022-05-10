use cb_2::lexer::url::URLToken;
use cb_2::lexer::url::URLToken::Link;
use logos::Logos;
use std::error::Error;
use std::fs;

#[test]
fn url_lexer() -> Result<(), Box<dyn Error>> {
    let html_text = fs::read_to_string("tests/resources/urls.html")?;
    let expected_tokens = fs::read_to_string("tests/resources/url_tokens.txt")?;

    let mut lexer = URLToken::lexer(html_text.as_str());
    // Parse all tokens
    let mut tokens = Vec::new();
    while let Some(Link((url, text))) = lexer.next() {
        let parse_result = format!("{:?}: {:?}", url, text);
        println!("{:?}: {:?}", url, text);
        tokens.push(parse_result);
    }

    assert_eq!(tokens.len(), 2);

    for (parsed, expected) in tokens.iter().zip(expected_tokens.lines()) {
        assert_eq!(parsed, expected);
    }
    Ok(())
}
