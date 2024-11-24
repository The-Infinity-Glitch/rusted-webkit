use crate::tokens::{Token, TokenType};

use logos::{Lexer, Logos};

/// Update the line count and the char index.
pub fn newline_callback(lex: &mut Lexer<TokenType>) {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
}

/// Compute the line and column position for the current word.
pub fn word_callback(lex: &mut Lexer<TokenType>) {
    let _line = lex.extras.0;
    let _column = lex.span().start - lex.extras.1;
}

struct Position {
    pub line: usize,
    pub column: usize,
}

pub fn lex_html(input: &str) -> Vec<Token> {
    let mut lex = TokenType::lexer(input);
    let mut token_position = Position { line: 0, column: 0 };
    let mut result: Vec<Token> = vec![];

    let mut error_count: usize = 0;

    while let Some(token_type) = lex.next() {
        match token_type {
            Ok(token) => {
                // If the token type is a new line token, so update the token position
                match token {
                    TokenType::NewLine => token_position.line += 1,
                    _ => token_position.column = lex.span().start - lex.extras.1,
                }

                // A new token
                let tk = Token {
                    token_type: token.clone(),
                    token_value: lex.slice().to_string(),
                    line: token_position.line,
                    column: token_position.column,
                };

                // Insert the new token in the result vector
                result.insert(result.len(), tk);
            }
            Err(_) => {
                error_count += 1;
            }
        }
    }

    result.insert(
        result.len(),
        Token {
            token_type: TokenType::Eof,
            token_value: String::from("EOF"),
            line: token_position.line.clone(),
            column: token_position.column.clone() + 1,
        },
    );

    return result;
}
