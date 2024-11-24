use crate::lexer::{newline_callback, word_callback};

use logos::Logos;

/// All tokens of html
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(extras = (usize, usize))]
pub enum TokenType {
    // Special symbols
    #[token(" ", word_callback)]
    Space,

    #[token("\n", newline_callback)]
    NewLine,

    #[token("\0", word_callback)]
    Eof,

    // Punctuation
    #[token(".", word_callback)]
    Dot,

    #[token(",", word_callback)]
    Comma,

    #[token(":", word_callback)]
    Colon,

    #[token(";", word_callback)]
    SemiColon,

    // Delimiters
    #[token("<", word_callback)]
    LArrow,

    #[token(">", word_callback)]
    RArrow,

    #[token("<!", word_callback)]
    DeclarationTag,

    #[token("</", word_callback)]
    EndTag,

    // Symbols
    #[token("/", word_callback)]
    Slash,

    #[token("!", word_callback)]
    Exclamation,

    // Operators
    #[token("=", word_callback)]
    Assign,

    // Literals
    #[regex(r#"[\p{Alphabetic}][\p{Alphabetic}0-9]*"#, word_callback)]
    Identifier,

    #[regex(r#"[\p{Emoji}]"#, priority = 3, callback = word_callback)]
    Emoji,

    #[regex(r#"'([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*'"#, word_callback)]
    CharLiteral,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, word_callback)]
    StringLiteral,
}

/// A token struct who stores the token type, value and the position of the token
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
    pub line: usize,
    pub column: usize,
}
