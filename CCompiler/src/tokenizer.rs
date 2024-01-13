//! Module for performing lexical analysis on source code.

use std::io::{self, Error, ErrorKind};

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
    Integer(usize),
    Decimal(f64),
    QuotedString(String),
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    OpenSquareBracket,
    CloseSquareBracket,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
}

#[must_use = "This function is useless without the return value"]
fn iter_while<F>(src: &str, mut predicate: F) -> (&str, usize)
where
    F: FnMut(char) -> bool,
{
    let mut cidx = 0;
    for ch in src.chars() {
        if !predicate(ch) {
            break;
        }
        cidx += 1;
    }
    (&src[..cidx], cidx)
}

fn tokenize_number(src: &str) -> io::Result<(TokenType, usize)> {
    let mut e = false;
    let mut dot = false;
    let mut minus = false;
    let (decimal, bytes) = iter_while(src, |ch| match ch {
        'e' => {
            if e {
                return false;
            }
            e = true;
            true
        }
        '.' => {
            if dot {
                return false;
            }
            dot = true;
            true
        }
        '-' => {
            if minus {
                return false;
            }
            minus = true;
            true
        }
        _ => ch.is_numeric(),
    });

    if dot {
        let value: f64 = decimal.parse().unwrap();
        Ok((TokenType::Decimal(value), bytes))
    } else {
        let value: usize = decimal.parse().unwrap();
        Ok((TokenType::Integer(value), bytes))
    }
}

fn tokenize_identifier(src: &str) -> Result<(TokenType, usize), Error> {
    let (identifier, bytes) = iter_while(src, |ch| ch.is_alphanumeric() || ch == '_');

    match identifier {
        "return" => Ok((TokenType::Keyword(Keyword::Return), 6)),
        "void" => Ok((TokenType::Keyword(Keyword::Void), 4)),
        "int" => Ok((TokenType::Keyword(Keyword::Int), 3)),
        _ => Ok((TokenType::Identifier(identifier.to_string()), bytes)),
    }
}

fn tokenize(src: &str) -> Result<(TokenType, usize), Error> {
    let next = match src.chars().next() {
        Some(c) => c,
        None => panic!("Unexpected EOF!"),
    };

    match next {
        ';' => Ok((TokenType::Semicolon, 1)),
        '(' => Ok((TokenType::OpenParenthesis, 1)),
        ')' => Ok((TokenType::CloseParenthesis, 1)),
        '{' => Ok((TokenType::OpenBrace, 1)),
        '}' => Ok((TokenType::CloseBrace, 1)),
        '[' => Ok((TokenType::OpenSquareBracket, 1)),
        ']' => Ok((TokenType::CloseSquareBracket, 1)),
        '+' => Ok((TokenType::Plus, 1)),
        '-' => Ok((TokenType::Minus, 1)),
        '*' => Ok((TokenType::Asterisk, 1)),
        '/' => Ok((TokenType::Slash, 1)),
        '%' => Ok((TokenType::Percent, 1)),
        '0'..='9' => Ok(tokenize_number(src)?),
        next @ '_' | next if next.is_alphabetic() => Ok(tokenize_identifier(src)?),
        _ => Err(Error::from(ErrorKind::Unsupported)),
    }
}

pub struct Tokenizer<'a> {
    cidx: usize,        // Current index
    srcbuffer: &'a str, // Remaining source buffer
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &str) -> Tokenizer {
        Tokenizer {
            cidx: 0,
            srcbuffer: src,
        }
    }

    pub fn next_token(&mut self) -> Result<Option<TokenType>, Error> {
        self.skip_whitespace();

        if self.srcbuffer.is_empty() {
            Ok(None)
        } else {
            match tokenize(self.srcbuffer) {
                Ok((token, bytes)) => {
                    self.srcbuffer = &self.srcbuffer[bytes..];
                    self.cidx += bytes;
                    Ok(Some(token))
                }
                Err(error) => {
                    panic!("Error: {:?}", error);
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        let (_, bytes) = iter_while(self.srcbuffer, |ch| ch.is_whitespace());
        self.cidx += bytes;
        self.srcbuffer = &self.srcbuffer[bytes..];
    }
}
