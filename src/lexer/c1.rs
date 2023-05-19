use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex
    #[token("bool")]
    c1bool,
    #[token("do")]
    c1do,
    #[token("while")]
    c1while,
    #[token("else")]
    c1else,
    #[token("float")]
    c1float,
    #[token("for")]
    c1for,
    #[token("if")]
    c1if,
    #[token("int")]
    c1int,
    #[token("printf")]
    c1printf,
    #[token("return")]
    c1return,
    #[token("void")]
    c1void,
    #[token("+")]
    c1plus,
    #[token("-")]
    c1minus,
    #[token("*")]
    c1asterisk,
    #[token("/")]
    c1slash,
    #[token("=")]
    c1assign,
    #[token("==")]
    c1equal,
    #[token("!=")]
    c1notequal,
    #[token("<")]
    c1lessthan,
    #[token(">")]
    c1greaterthan,
    #[token("<=")]
    c1lessequal,
    #[token]



    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
