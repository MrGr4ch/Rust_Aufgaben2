use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // Keywords
    #[token("bool")]
    Bool,

    #[token("float")]
    Float,

    #[token("int")]
    Int,

    #[token("if")]
    If,

    #[token("otherwise")]
    Otherwise,

    #[token("do")]
    Do,

    #[token("while")]
    While,

    #[token("return")]
    Return,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("void")]
    Void,

    #[token("main")]
    Main,

    // Operators
    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("==")]
    Equal,

    #[token("!=")]
    NotEqual,

    #[token("!")]
    Not,

    #[token("+")]
    Plus,

    #[token("*")]
    Multiply,

    #[token("=")]
    Assign,

    // Identifiers (function names, variable names)
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", logos::skip)]
    Identifier,

    // Numeric literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    IntegerLiteral,

    // Float literals
    #[regex(r"[0-9]*\.[0-9]+([eE][+-]?[0-9]+)?", |lex| lex.slice().parse())]
    FloatLiteral,

    // Parentheses and braces
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    // C and C++ style comments
    #[regex(r"/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/")]
    Comment,

    #[regex(r"//.*", logos::skip)]
    CommentCpp,

    // Whitespaces
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    Whitespace,

    // Error handling
    #[error]
    Error,
}
