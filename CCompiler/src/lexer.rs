#[derive(Debug)]
pub enum Keyword {
    Return,
    Void,
    Int,
}

#[derive(Debug)]
pub enum TokenType {
    Keyword(Keyword),
    Identifier(String),
    Numeric(String),
    Operator(char),
    Parenthesis(char),
    Semicolon(char),
}

fn is_numeric_start(ch: char) -> bool {
    ch.is_numeric()
}

// Currently only decimal numbers are recognized
fn is_numeric(ch: char) -> bool {
    ch.is_numeric()
}

fn is_symbol_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_symbol(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

pub fn lex(source: &str) -> Vec<TokenType> {
    let mut cursor = 0;
    let mut tokens: Vec<TokenType> = Vec::new();

    // Iterate over the source characters one by one and determine the tokens
    while cursor < source.len() {
        // Grab the current character
        let ch = source.chars().nth(cursor).unwrap();

        if ch.is_whitespace() {
            cursor += 1;
        } else {
            match ch {
                '(' | ')' | '{' | '}' | '[' | ']' => {
                    tokens.push(TokenType::Parenthesis(ch));
                    cursor += 1;
                }
                '+' | '-' | '*' | '/' | '%' => {
                    tokens.push(TokenType::Operator(ch));
                    cursor += 1;
                }
                ';' => {
                    tokens.push(TokenType::Semicolon(ch));
                    cursor += 1;
                }
                _ => {
                    // Identify Symbols and Keywords
                    if is_symbol_start(ch) {
                        let last = cursor;

                        // Read the entire symbol
                        while cursor < source.len()
                            && is_symbol(source.chars().nth(cursor).unwrap())
                        {
                            cursor += 1;
                        }

                        // Get the symbol string
                        let symbol = &source[last..cursor];

                        // Check if the symbol is a keyword
                        match symbol {
                            "return" => {
                                tokens.push(TokenType::Keyword(Keyword::Return));
                            }
                            "void" => {
                                tokens.push(TokenType::Keyword(Keyword::Void));
                            }
                            "int" => {
                                tokens.push(TokenType::Keyword(Keyword::Int));
                            }
                            _ => {
                                tokens.push(TokenType::Identifier(symbol.to_string()));
                            }
                        }
                    }
                    // Identify Numerics
                    else if is_numeric_start(ch) {
                        let last = cursor;

                        // Read the entire symbol
                        while cursor < source.len()
                            && is_numeric(source.chars().nth(cursor).unwrap())
                        {
                            cursor += 1;
                        }

                        // Get the symbol string
                        let numeric = &source[last..cursor];
                        tokens.push(TokenType::Numeric(numeric.to_string()));
                    } else {
                        panic!("Couldn't recognize token: {}", ch);
                    }
                }
            }
        }
    }
    // Return the sequence of tokens
    tokens
}
