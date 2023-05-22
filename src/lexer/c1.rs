use logos::{Logos, Skip};

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    //Whitespace
    #[regex(r"[ \t\n\r]+", |_| Skip)]

    //keywords
    #[token("bool")]
    KwBoolean,
    #[token("do")]
    KwDo,
    #[token("while")]
    KwWhile,
    #[token("else")]
    KwElse,
    #[token("float")]
    KwFloat,
    #[token("for")]
    KwFor,
    #[token("if")]
    KwIf,
    #[token("int")]
    KwInt,
    #[token("printf")]
    KwPrintf,
    #[token("return")]
    KwReturn,
    #[token("void")]
    KwVoid,

    //operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("<")]
    Lss,
    #[token(">")]
    Grt,
    #[token("<=")]
    Leq,
    #[token(">=")]
    Geq,
    #[token("&&")]
    And,
    #[token("||")]
    Or,

    //other tokens
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

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
    #[regex("([0-9]*[.][0-9]+)([eE]([-+])?[0-9]+)?|[0-9]+[eE]([+-])?[0-9]+")]
    ConstFloat,
    #[regex("true|false")]
    ConstBoolean,
    #[regex("\"[^\n\"]*\"")]
    ConstString,
    #[regex("([a-zA-Z_])+([0-9]|[a-zA-Z_])*")]
    Id,

    //comments
    #[regex("(?s)[/][*][^(/*)]*[*][/]", |_| Skip)]
    CComment,
    #[regex("//.*", |_| Skip)]
    CplusplusComment,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
