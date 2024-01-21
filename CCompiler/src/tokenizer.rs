//! Module for performing lexical analysis on source code.
use crate::errors::CompilerError;

#[derive(Debug, PartialEq, Clone)]
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
    Inline,
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

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    None,
    Keyword(Keyword),
    Identifier(String),
    Integer(i64),
    FloatingPoint(f64),
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

fn tokenize_number(src: &str) -> Result<(TokenType, usize), CompilerError> {
    let mut e = false;
    let mut dot = false;
    let mut minus = false;
    let (number, bytes) = iter_while(src, |ch| match ch {
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
        match number.parse::<f64>() {
            Ok(value) => Ok((TokenType::FloatingPoint(value), bytes)),
            Err(err) => Err(CompilerError::UnexpectedTokenError(format!(
                "Expected a valid C decimal value, instead got: {}",
                number
            ))),
        }
    } else {
        match number.parse::<i64>() {
            Ok(value) => Ok((TokenType::Integer(value), bytes)),
            Err(err) => Err(CompilerError::UnexpectedTokenError(format!(
                "Expected a valid C integer value, instead got: {}",
                number
            ))),
        }
    }
}

fn tokenize_char(src: &str) -> Result<(TokenType, usize), CompilerError> {
    let (ch, bytes) = iter_while(&src[1..], |ch| ch != '\'');

    match ch.len() {
        1 => Ok((TokenType::Character(ch.chars().next().unwrap()), bytes + 2)),
        _ => Err(CompilerError::UnexpectedTokenError(format!(
            "A single quoted literal can only have 1 character and not: {}",
            ch
        ))),
    }
}

fn tokenize_string(src: &str) -> Result<(TokenType, usize), CompilerError> {
    let (stringliteral, bytes) = iter_while(&src[1..], |ch| ch != '"');

    match src.chars().nth(bytes + 1) {
        Some('"') => Ok((
            TokenType::StringLiteral(stringliteral.to_string()),
            bytes + 2,
        )),
        _ => Err(CompilerError::UnexpectedTokenError(
            "Missing \" in a quoted string literal".to_string(),
        )),
    }
}

// This function never returns any error, should the return type be changed?
// Or kept as it is to be consistent with other functions?
fn tokenize_identifier(src: &str) -> Result<(TokenType, usize), CompilerError> {
    let (identifier, bytes) = iter_while(src, |ch| ch.is_alphanumeric() || ch == '_');

    let tokentype = match identifier {
        // Check if the so called `identifier` is actually a keyword
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
        "inline" => TokenType::Keyword(Keyword::Inline),
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

        // Else it is really an identifier
        _ => TokenType::Identifier(identifier.to_string()),
    };
    Ok((tokentype, bytes))
}

fn tokenize(src: &str) -> Result<(TokenType, usize), CompilerError> {
    let next = match src.chars().next() {
        Some(c) => c,
        None => panic!("Internal Error: Failed to get the next character from the src buffer, presumably it's empty!"),
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
        _ => Err(CompilerError::UnexpectedTokenError(format!(
            "Unexpected token: {}",
            next
        ))),
    }
}

pub struct Tokenizer<'a> {
    cidx: usize,            // Current index
    srcbuffer: &'a str,     // Remaining source buffer
    peekedtoken: TokenType, // Store the peeked token, to be reused by self.next_token()
    peekedbytes: usize,     // The number of bytes that were peeked
    linerow: usize,         // Current line character row
    linecol: usize,         // Current line character column
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &str) -> Tokenizer {
        Tokenizer {
            cidx: 0,
            srcbuffer: src,
            peekedtoken: TokenType::None,
            peekedbytes: 0,
            linerow: 0,
            linecol: 0,
        }
    }

    // TODO: Handle the case where if multiple times the same token is peeked...
    // Return the stored token instead of retokenizing it
    pub fn peek_token(&mut self) -> Result<Option<TokenType>, CompilerError> {
        let (_, whitespace_bytes) = iter_while(self.srcbuffer, |ch| ch.is_whitespace());

        if self.srcbuffer.is_empty() {
            Ok(None)
        } else {
            let (token, bytes) = tokenize(&self.srcbuffer[whitespace_bytes..])?;

            // Store the peeked token info
            self.peekedtoken = token.clone();
            self.peekedbytes = whitespace_bytes + bytes;

            // Return the newly parsed token instead of parsing the srcbuffer again
            Ok(Some(token))
        }
    }

    // This function parses the next token and consumes it
    // It moves forward the source pointers to the start of the next token
    pub fn next_token(&mut self) -> Result<Option<TokenType>, CompilerError> {
        // Check if the next token is already peeked/processed
        if self.peekedbytes != 0 {
            // If Yes then move the srcbuffer forward and return the stored token
            self.srcbuffer = &self.srcbuffer[self.peekedbytes..];
            self.cidx += self.peekedbytes;

            // This is a temporary variable
            let peekedtoken = self.peekedtoken.clone(); // Is this optimal?

            // Reset the peeked token info
            self.peekedtoken = TokenType::None;
            self.peekedbytes = 0;

            // Return the already stored token instead of parsing the srcbuffer again
            return Ok(Some(peekedtoken));
        };

        // Read the next token and return it
        self.skip_whitespace();

        if self.srcbuffer.is_empty() {
            Ok(None)
        } else {
            let (token, bytes) = tokenize(self.srcbuffer)?;
            self.srcbuffer = &self.srcbuffer[bytes..];
            self.cidx += bytes;

            // Just update the line column
            // As the intended behaviour of `tokenize` function is to not include newline in any of the tokens
            self.linecol += bytes;

            Ok(Some(token))

            // Err(error) => {
            //     panic!(
            //         "Error tokenizing at line:{}:{}: {:?}",
            //         self.linerow, self.linecol, error
            //     );
            // }
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
