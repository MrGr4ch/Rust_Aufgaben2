use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex

    //keywords
    #[token("bool")]
    C1bool,
    #[token("do")]
    C1do,
    #[token("while")]
    C1while,
    #[token("else")]
    C1else,
    #[token("float")]
    C1float,
    #[token("for")]
    C1for,
    #[token("if")]
    C1if,
    #[token("int")]
    C1int,
    #[token("printf")]
    C1printf,
    #[token("return")]
    C1return,
    #[token("void")]
    C1void,

    //operators
    #[token("+")]
    C1plus,
    #[token("-")]
    C1minus,
    #[token("*")]
    C1asterisk,
    #[token("/")]
    C1slash,
    #[token("=")]
    C1assign,
    #[token("==")]
    C1equal,
    #[token("!=")]
    C1notequal,
    #[token("<")]
    C1lessthan,
    #[token(">")]
    C1greaterthan,
    #[token("<=")]
    C1lessequal,
    #[token(">=")]
    C1greaterequal,
    #[token("&&")]
    C1and,
    #[token("||")]
    C1or,

    //other tokens
    #[token(",")]
    C1comma,
    #[token(";")]
    C1semicolon,
    #[token("(")]
    C1leftparen,
    #[token(")")]
    C1rightparen,
    #[token("{")]
    C1leftbrace,
    #[token("}")]
    C1rightbrace,

    //pseudotoken
    #[regex("[0-9]")]
    Digit,
    #[regex("[:Digit:]+")]
    Integer,
    #[regex("[:Integer:].[:Integer:]|.[:Integer:]")]
    Float,
    #[regex("[a-zA-z]")]
    Letter,

    //termvariables
    #[regex("[:Integer:]")]
    ConstInt,
    #[regex("[:Float:]")]




    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
