use logos::{Logos, Skip};

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex

    //Whitespace
    #[regex(r"[ \t\n\f\r]+", logos::skip)]

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
    //#[regex("[0-9]")]
    //Digit,
    //#[regex("[:Digit:]+")]
    //Integer,
    //#[regex("[:Integer:].[:Integer:]|.[:Integer:]")]
    //Float,
    //#[regex("[a-zA-z]")]
    //Letter,

    //termvariables
    #[regex("[0-9]+")]
    ConstInt,
    #[regex("([0-9]* [.] [0-9]+) ( [eE] ([-+])? [0-9]+)? | [0-9]+ [eE] ([+-])? [0-9]+")]
    ConstFloat,
    #[regex("true|false")]
    ConstBool,
    #[regex("\" [^\n\"]* \"")]
    ConstString,
    #[regex("([a-zA-z])+ ([0-9] | [a-zA-z])*")]
    Id,

    //comments
    #[regex("([/][*])(.|\n)([*][/])", |_| Skip)]
    #[regex("([/][*])(.)(\n)", |_| Skip)]

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
