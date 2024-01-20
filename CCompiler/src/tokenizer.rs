//! Module for performing lexical analysis on source code.

use std::io::{self, Error, ErrorKind};

#[derive(Debug)]
pub enum Keyword {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    Register,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

#[derive(Debug)]
pub enum TokenType {
    Keyword(Keyword),
    Identifier(String),
    Integer(usize),
    Decimal(f64),
    Character(char),
    StringLiteral(String),
    OpenBrace,                 // {
    CloseBrace,                // }
    OpenParenthesis,           // (
    CloseParenthesis,          // )
    OpenSquareBracket,         // [
    CloseSquareBracket,        // ]
    Semicolon,                 // ;
    Plus,                      // +
    PlusEquals,                // +=
    Minus,                     // -
    MinusEquals,               // -=
    Asterisk,                  // *
    AsteriskEquals,            // *=
    Slash,                     // /
    SlashEquals,               // /=
    Percent,                   // %
    PercentEquals,             // %=
    AssignmentOperator,        // =
    EqualityOperator,          // ==
    NotEqualsOperator,         // !=
    IncrementOperator,         // ++
    DecrementOperator,         // --
    LessThanOperator,          // <
    LessThanEqualsOperator,    // <=
    GreaterThanOperator,       // >
    GreaterThanEqualsOperator, // >=
    LogicalNotOperator,        // !
    LeftShiftOperator,         // <<
    LeftShiftEqualsOperator,   // <<=
    RightShiftOperator,        // >>
    RightShiftEqualsOperator,  // >>=
    BitwiseAndOperator,        // &
    BitwiseAndEqualsOperator,  // &=
    LogicalAndOperator,        // &&
    BitwiseOrOperator,         // |
    BitwiseOrEqualsOperator,   // |=
    LogicalOrOperator,         // ||
    ExclusiveOrOperator,       // ^
    ExclusiveOrEqualsOperator, // ^=
    BitwiseComplimentOperator, // ~
    QuestionMark,              // ?
    DotOperator,               // .
    ArrowOperator,             // ->
    Colon,                     // :
    ScopeOperator,             // ::
    Comma,                     // ,
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

fn tokenize_char(src: &str) -> Result<(TokenType, usize), Error> {
    let (ch, bytes) = iter_while(&src[1..], |ch| ch != '\'');

    match ch.len() {
        1 => Ok((TokenType::Character(ch.chars().next().unwrap()), bytes + 2)),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

fn tokenize_string(src: &str) -> Result<(TokenType, usize), Error> {
    let (stringliteral, bytes) = iter_while(&src[1..], |ch| ch != '"');

    match src.chars().nth(bytes + 1) {
        Some('"') => Ok((
            TokenType::StringLiteral(stringliteral.to_string()),
            bytes + 2,
        )),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

fn tokenize_identifier(src: &str) -> Result<(TokenType, usize), Error> {
    let (identifier, bytes) = iter_while(src, |ch| ch.is_alphanumeric() || ch == '_');

    let tokentype = match identifier {
        "auto" => TokenType::Keyword(Keyword::Auto),
        "break" => TokenType::Keyword(Keyword::Break),
        "case" => TokenType::Keyword(Keyword::Case),
        "char" => TokenType::Keyword(Keyword::Char),
        "const" => TokenType::Keyword(Keyword::Const),
        "continue" => TokenType::Keyword(Keyword::Continue),
        "default" => TokenType::Keyword(Keyword::Default),
        "do" => TokenType::Keyword(Keyword::Do),
        "double" => TokenType::Keyword(Keyword::Double),
        "else" => TokenType::Keyword(Keyword::Else),
        "enum" => TokenType::Keyword(Keyword::Enum),
        "extern" => TokenType::Keyword(Keyword::Extern),
        "float" => TokenType::Keyword(Keyword::Float),
        "for" => TokenType::Keyword(Keyword::For),
        "goto" => TokenType::Keyword(Keyword::Goto),
        "if" => TokenType::Keyword(Keyword::If),
        "int" => TokenType::Keyword(Keyword::Int),
        "long" => TokenType::Keyword(Keyword::Long),
        "register" => TokenType::Keyword(Keyword::Register),
        "return" => TokenType::Keyword(Keyword::Return),
        "short" => TokenType::Keyword(Keyword::Short),
        "signed" => TokenType::Keyword(Keyword::Signed),
        "sizeof" => TokenType::Keyword(Keyword::Sizeof),
        "static" => TokenType::Keyword(Keyword::Static),
        "struct" => TokenType::Keyword(Keyword::Struct),
        "switch" => TokenType::Keyword(Keyword::Switch),
        "typedef" => TokenType::Keyword(Keyword::Typedef),
        "union" => TokenType::Keyword(Keyword::Union),
        "unsigned" => TokenType::Keyword(Keyword::Unsigned),
        "void" => TokenType::Keyword(Keyword::Void),
        "volatile" => TokenType::Keyword(Keyword::Volatile),
        "while" => TokenType::Keyword(Keyword::While),

        _ => TokenType::Identifier(identifier.to_string()),
    };
    Ok((tokentype, bytes))
}

fn tokenize(src: &str) -> Result<(TokenType, usize), Error> {
    let next = match src.chars().next() {
        Some(c) => c,
        None => panic!("Unexpected EOF!"),
    };

    // Required to check multicharacter operators like ++, --, +=, -=, &&, ||
    let next2next = src.chars().nth(1);
    // Required to check multicharacter operators like <<=, >>=
    let next2next2next = src.chars().nth(2);

    match next {
        // Handle Single Character Operators
        ';' => Ok((TokenType::Semicolon, 1)),
        '(' => Ok((TokenType::OpenParenthesis, 1)),
        ')' => Ok((TokenType::CloseParenthesis, 1)),
        '{' => Ok((TokenType::OpenBrace, 1)),
        '}' => Ok((TokenType::CloseBrace, 1)),
        '[' => Ok((TokenType::OpenSquareBracket, 1)),
        ']' => Ok((TokenType::CloseSquareBracket, 1)),
        '?' => Ok((TokenType::QuestionMark, 1)),
        '.' => Ok((TokenType::DotOperator, 1)),
        ',' => Ok((TokenType::Comma, 1)),
        '~' => Ok((TokenType::BitwiseComplimentOperator, 1)),

        // Handle Multicharacter Operators
        '+' => match next2next {
            Some('+') => Ok((TokenType::IncrementOperator, 2)),
            Some('=') => Ok((TokenType::PlusEquals, 2)),
            _ => Ok((TokenType::Plus, 1)),
        },
        '-' => match next2next {
            Some('-') => Ok((TokenType::DecrementOperator, 2)),
            Some('=') => Ok((TokenType::MinusEquals, 2)),
            Some('>') => Ok((TokenType::ArrowOperator, 2)),
            _ => Ok((TokenType::Minus, 1)),
        },
        '*' => match next2next {
            Some('=') => Ok((TokenType::AsteriskEquals, 2)),
            _ => Ok((TokenType::Asterisk, 1)),
        },
        '/' => match next2next {
            Some('=') => Ok((TokenType::SlashEquals, 2)),
            _ => Ok((TokenType::Slash, 1)),
        },
        '%' => match next2next {
            Some('=') => Ok((TokenType::PercentEquals, 2)),
            _ => Ok((TokenType::Percent, 1)),
        },
        '=' => match next2next {
            Some('=') => Ok((TokenType::EqualityOperator, 2)),
            _ => Ok((TokenType::AssignmentOperator, 1)),
        },
        '!' => match next2next {
            Some('=') => Ok((TokenType::NotEqualsOperator, 2)),
            _ => Ok((TokenType::LogicalNotOperator, 1)),
        },
        '<' => match next2next {
            // Handle <<=
            Some('<') => match next2next2next {
                Some('=') => Ok((TokenType::LeftShiftEqualsOperator, 3)),
                _ => Ok((TokenType::LeftShiftOperator, 2)),
            },
            Some('=') => Ok((TokenType::LessThanEqualsOperator, 2)),
            _ => Ok((TokenType::LessThanOperator, 1)),
        },
        '>' => match next2next {
            // Handle >>=
            Some('>') => match next2next2next {
                Some('=') => Ok((TokenType::RightShiftEqualsOperator, 3)),
                _ => Ok((TokenType::RightShiftOperator, 2)),
            },
            Some('=') => Ok((TokenType::GreaterThanEqualsOperator, 2)),
            _ => Ok((TokenType::GreaterThanOperator, 1)),
        },
        '&' => match next2next {
            Some('&') => Ok((TokenType::LogicalAndOperator, 2)),
            Some('=') => Ok((TokenType::BitwiseAndEqualsOperator, 2)),
            _ => Ok((TokenType::BitwiseAndOperator, 1)),
        },
        '|' => match next2next {
            Some('|') => Ok((TokenType::LogicalOrOperator, 2)),
            Some('=') => Ok((TokenType::BitwiseOrEqualsOperator, 2)),
            _ => Ok((TokenType::BitwiseOrOperator, 1)),
        },
        '^' => match next2next {
            Some('=') => Ok((TokenType::ExclusiveOrEqualsOperator, 2)),
            _ => Ok((TokenType::ExclusiveOrOperator, 1)),
        },
        ':' => match next2next {
            Some(':') => Ok((TokenType::ScopeOperator, 2)),
            _ => Ok((TokenType::Colon, 1)),
        },

        // Handle quoted values like char and string
        '\'' => Ok(tokenize_char(src)?),
        '"' => Ok(tokenize_string(src)?),

        // Handle numbers and identifiers
        '0'..='9' => Ok(tokenize_number(src)?),
        next @ '_' | next if next.is_alphabetic() => Ok(tokenize_identifier(src)?),

        // Handle unsupported characters
        _ => Err(Error::from(ErrorKind::Unsupported)),
    }
}

pub struct Tokenizer<'a> {
    cidx: usize,        // Current index
    srcbuffer: &'a str, // Remaining source buffer
    linerow: usize,     // Current line character row
    linecol: usize,     // Current line character column
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &str) -> Tokenizer {
        Tokenizer {
            cidx: 0,
            srcbuffer: src,
            linerow: 0,
            linecol: 0,
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

                    // Just update the line column
                    // As the intended behaviour of `tokenize` function is to not include newline in any of the tokens
                    self.linecol += bytes;

                    Ok(Some(token))
                }
                Err(error) => {
                    panic!(
                        "Error tokenizing at line:{}:{}: {:?}",
                        self.linerow, self.linecol, error
                    );
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        let (_, bytes) = iter_while(self.srcbuffer, |ch| {
            if ch == '\n' {
                self.linerow += 1;
                self.linecol = 0;
            }
            let whitespace = ch.is_whitespace();
            if whitespace {
                self.linecol += 1;
            }
            return whitespace;
        });

        //  Update the actual buffer and it's index
        self.cidx += bytes;
        self.srcbuffer = &self.srcbuffer[bytes..];
    }
}
