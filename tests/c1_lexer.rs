use cb_2::lexer::c1::C1Token;
use logos::Logos;
use std::error::Error;
use std::fs;

#[test]
fn scan_demorgan() -> Result<(), Box<dyn Error>> {
    let demorgan_input = fs::read_to_string("tests/resources/demorgan.c1")?;
    let demorgan_expected = fs::read_to_string("tests/resources/demorgan_tokens.txt")?;

    let mut lexer = C1Token::lexer(demorgan_input.as_str());
    // Parse all tokens
    let mut tokens = Vec::new();
    while let Some(val) = lexer.next() {
        let parse_result = format!("{:?}: {:?}", val, lexer.slice());
        println!("{}", parse_result);
        tokens.push(parse_result);
    }

    for (parsed, expected) in tokens.iter().zip(demorgan_expected.lines()) {
        assert_eq!(parsed, expected);
    }
    Ok(())
}
