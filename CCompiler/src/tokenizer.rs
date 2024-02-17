//! Module for performing lexical analysis on source code.
use crate::{
    errors::{CompilerError, CompilerErrorKind},
    node::FileLocation,
};

use regex::Regex;

#[allow(non_camel_case_types)]
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
    Restrict,
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
    _Alignas,
    _Alignof,
    _Atomic,
    _Bool,
    _Complex,
    _Generic,
    _Imaginary,
    _Noreturn,
    _Static_assert,
    _Thread_local,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IntegerType {
    Generic(i64),
    Signed(i32),
    SignedLong(i64),
    SignedLongLong(i128),
    Unsigned(u32),
    UnsignedLong(u64),
    UnsignedLongLong(u128),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FloatingPointType {
    Float(f32),
    Double(f64),
    LongDouble(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    None,
    Keyword(Keyword),
    Identifier(String),
    Integer(IntegerType),
    FloatingPoint(FloatingPointType),
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
    Equals,                    // =
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

// type Token = (TokenType, usize, usize);
//   ^^       ^^         ^^     ^^ Token End
//   Alias    Token      Token Start

type Token = (TokenType, FileLocation, FileLocation);
//   ^^       ^^         ^^             ^^
//   Alias    Token      Token Start      Token End

// struct Token {
//     tokentype: TokenType,
//     start: TokenPosition,
//     end: TokenPosition,
// }

/// Iterates the src string as long as the given predicate is satisfied.
/// Returns the sliced string which satisfied the predicate and also the number of characters in it
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

/// The tokenizer class
pub struct Tokenizer<'a> {
    cidx: usize,                   // Current index
    srcbuffer: &'a str,            // Remaining source buffer
    peekedtoken: Token,            // Store the peeked token, to be reused by self.next_token()
    peekedbytes: usize,            // The number of bytes that were peeked
    peeked_linerow: usize,         // The line row till which we have peeked
    peeked_linecol: usize,         // The line column till which we have peeked
    linerow: usize,                // Current line character row
    linecol: usize,                // Current line character column
    numeric_constant_regex: Regex, // Regular expression for a numeric constant in C
}

impl<'a> Default for Tokenizer<'a> {
    fn default() -> Self {
        Tokenizer {
            cidx: 0,
            srcbuffer: "",
            peekedtoken: (TokenType::None, FileLocation::none(), FileLocation::none()),
            peekedbytes: 0,
            peeked_linerow: 0,
            peeked_linecol: 0,
            linerow: 0,
            linecol: 0,
            numeric_constant_regex: Regex::new(r"").unwrap(),
        }
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Tokenizer {
            cidx: 0,
            srcbuffer: src,
            peekedtoken: (TokenType::None, FileLocation::none(), FileLocation::none()),
            peekedbytes: 0,
            peeked_linerow: 1,
            peeked_linecol: 1,
            linerow: 1,
            linecol: 1,
            numeric_constant_regex: Regex::new(
                r"^[+-]?(?P<number>\d+(?P<dot>\.\d+)?(?P<exp>[eE][+-]?\d+)?)((?i)(?P<suffix>(u|l|ul|ull|f|d)))?\b"
            ).unwrap()
        }
    }

    /// Returns the next token without advancing the tokenizer
    pub fn peek_token(&mut self) -> Result<Option<Token>, CompilerError> {
        // If multiple times the same token is peeked, then return the already peeked token
        // Instead of tokenizing it again
        if self.peekedbytes != 0 {
            return Ok(Some(self.peekedtoken.clone()));
        }

        // Set the peeked line row and column
        self.peeked_linerow = self.linerow;
        self.peeked_linecol = self.linecol;

        // let (mut temp_srcbuffer, mut skipped_bytes) = self.peekover_whitespace();

        // Equivalent to self.skip_whitespace() except we don't advance the tokenizer
        let (_, mut skipped_bytes) = iter_while(self.srcbuffer, |ch| {
            if ch == '\n' {
                self.peeked_linerow += 1;
                self.peeked_linecol = 1;
                return true;
            }
            if ch.is_whitespace() {
                self.peeked_linecol += 1;
                return true;
            }
            false
        });

        // Emulating advancement of the srcbuffer by skipping the whitespace
        let mut temp_srcbuffer = &self.srcbuffer[skipped_bytes..];

        // Emulating advancement of the srcbuffer by skipping all the consecutive single line comments
        while temp_srcbuffer.starts_with("//") {
            let (_, bytes) = iter_while(temp_srcbuffer, |ch| ch != '\n');
            temp_srcbuffer = &temp_srcbuffer[bytes..];

            // let (_, leading_wbytes) = iter_while(temp_srcbuffer, |ch| ch.is_whitespace());
            let (_, leading_wbytes) = iter_while(self.srcbuffer, |ch| {
                if ch == '\n' {
                    self.peeked_linerow += 1;
                    self.peeked_linecol = 1;
                    return true;
                }
                if ch.is_whitespace() {
                    self.peeked_linecol += 1;
                    return true;
                }
                false
            });

            temp_srcbuffer = &temp_srcbuffer[leading_wbytes..];
            skipped_bytes += bytes + leading_wbytes;
        }

        if temp_srcbuffer.is_empty() {
            Ok(None)
        } else {
            let (token, bytes) = self.tokenize(&temp_srcbuffer)?;

            // Store the peeked token info
            self.peeked_linecol += bytes;

            self.peekedbytes = skipped_bytes + bytes;
            self.peekedtoken = (
                token,
                FileLocation::new(self.peeked_linerow, self.peeked_linecol - bytes),
                FileLocation::new(self.peeked_linerow, self.peeked_linecol),
            );

            // Return the newly parsed token instead of parsing the srcbuffer again
            Ok(Some(self.peekedtoken.clone()))
        }
    }

    /// Tokenizes the next token and consumes it
    /// It moves forward the source pointers to the start of the next token
    pub fn next_token(&mut self) -> Result<Option<Token>, CompilerError> {
        // Check if the next token is already peeked/processed
        if self.peekedbytes != 0 {
            // If Yes then move the srcbuffer forward
            self.srcbuffer = &self.srcbuffer[self.peekedbytes..];
            self.cidx += self.peekedbytes;

            // Update location values
            self.linecol = self.peeked_linecol;
            self.linerow = self.peeked_linerow;

            // This is a temporary variable
            let peekedtoken = self.peekedtoken.clone(); // Is this optimal?

            // Reset the peeked token info
            self.peekedtoken = (TokenType::None, FileLocation::none(), FileLocation::none());
            self.peekedbytes = 0;

            // Return the already stored token instead of parsing the srcbuffer again
            return Ok(Some(peekedtoken));
        }

        // Read the next token and return it
        self.skip_whitespace();
        self.skip_comments();

        if self.srcbuffer.is_empty() {
            Ok(None)
        } else {
            let (token, bytes) = self.tokenize(self.srcbuffer)?;
            self.srcbuffer = &self.srcbuffer[bytes..];
            self.cidx += bytes;

            // Just update the line column
            // As the intended behaviour of `tokenize` function is to not include newline in any of the tokens
            self.linecol += bytes;

            // Return the newly parsed token with it's start and end information
            Ok(Some((
                token,
                FileLocation::new(self.linerow, self.linecol - bytes),
                FileLocation::new(self.linerow, self.linecol),
            )))
        }
    }

    pub fn get_cidx(&self) -> usize {
        self.cidx
    }

    pub fn get_lineinfo(&self) -> FileLocation {
        FileLocation::new(self.linerow, self.linecol)
    }

    fn skip_comments(&mut self) {
        while self.srcbuffer.starts_with("//") {
            let (_, bytes) = iter_while(self.srcbuffer, |ch| ch != '\n');

            //  Update the actual buffer and it's index
            self.cidx += bytes;
            self.srcbuffer = &self.srcbuffer[bytes..];

            // Skip the whitespace till the valid token of the next line
            self.skip_whitespace();
        }
    }

    fn skip_whitespace(&mut self) {
        let (_, bytes) = iter_while(self.srcbuffer, |ch| {
            if ch == '\n' {
                self.linerow += 1;
                self.linecol = 1;
                return true;
            }
            if ch.is_whitespace() {
                self.linecol += 1;
                return true;
            }
            false
        });

        //  Update the actual buffer and it's index
        self.cidx += bytes;
        self.srcbuffer = &self.srcbuffer[bytes..];
    }

    fn peekover_whitespace(&mut self) -> (&str, usize) {
        // Equivalent to self.skip_whitespace() except we don't advance the tokenizer
        let (_, skipped_bytes) = iter_while(self.srcbuffer, |ch| {
            if ch == '\n' {
                self.peeked_linerow += 1;
                self.peeked_linecol = 1;
            }
            let whitespace = ch.is_whitespace();
            if whitespace {
                self.peeked_linecol += 1;
            }
            whitespace
        });

        // Emulating advancement of the srcbuffer by skipping the whitespace
        let temp_srcbuffer = &self.srcbuffer[skipped_bytes..];

        (temp_srcbuffer, skipped_bytes)
    }

    fn tokenize(&self, src: &str) -> Result<(TokenType, usize), CompilerError> {
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
                _ => Ok((TokenType::Equals, 1)),
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
            '\'' => Ok(self.tokenize_char(src)?),
            '"' => Ok(self.tokenize_string(src)?),

            // Handle numbers and identifiers
            '0'..='9' => Ok(self.tokenize_number(src)?),
            next if next == '_' || next.is_alphabetic() => Ok(self.tokenize_identifier(src)?),

            // Handle unsupported characters
            _ => Err(CompilerError {
                kind: CompilerErrorKind::TokenizerError,
                message: format!("Unexpected token: {}", next),
                location: None,
            }),
        }
    }

    fn tokenize_number(&self, src: &str) -> Result<(TokenType, usize), CompilerError> {
        match self.numeric_constant_regex.captures(src) {
            Some(captures) => {
                match captures.name("number") {
                    Some(number) => {
                        // Check if it is a floating point constant
                        if captures.name("dot").is_some() || captures.name("exp").is_some() {
                            let fptype = match captures.name("suffix") {
                                Some(suffix) => match suffix.as_str() {
                                    "f" | "F" => FloatingPointType::Float(
                                        number.as_str().parse::<f32>().unwrap(),
                                    ),
                                    "l" | "L" => FloatingPointType::LongDouble(
                                        number.as_str().parse::<f64>().unwrap(),
                                    ),
                                    _ => {
                                        return Err(CompilerError {
                                            kind: CompilerErrorKind::TokenizerError,
                                            message: format!(
                                                "Invalid suffix: `{}` to a floating point constant",
                                                suffix.as_str()
                                            ),
                                            location: None, // TODO: Give the right location
                                        });
                                    }
                                },
                                None => FloatingPointType::Double(
                                    number.as_str().parse::<f64>().unwrap(),
                                ),
                            };
                            return Ok((TokenType::FloatingPoint(fptype), captures[0].len()));
                        } else {
                            // Else it is a integer constant
                            let inttype = match captures.name("suffix") {
                                Some(suffix) => match suffix.as_str() {
                                    "u" | "U" => IntegerType::Unsigned(
                                        number.as_str().parse::<u32>().unwrap(),
                                    ),
                                    "l" | "L" => IntegerType::SignedLong(
                                        number.as_str().parse::<i64>().unwrap(),
                                    ),
                                    "ul" | "uL" | "Ul" | "UL" => IntegerType::UnsignedLong(
                                        number.as_str().parse::<u64>().unwrap(),
                                    ),
                                    "ll" | "lL" | "Ll" | "LL" => IntegerType::SignedLongLong(
                                        number.as_str().parse::<i128>().unwrap(),
                                    ),
                                    "ull" | "ulL" | "uLl" | "uLL" | "Ull" | "UlL" | "ULl"
                                    | "ULL" => IntegerType::UnsignedLongLong(
                                        number.as_str().parse::<u128>().unwrap(),
                                    ),
                                    _ => {
                                        return Err(CompilerError {
                                            kind: CompilerErrorKind::TokenizerError,
                                            message: format!(
                                                "Expected an integer suffix, instead got `{}`",
                                                suffix.as_str()
                                            ),
                                            location: None, // TODO: Give the right location
                                        });
                                    }
                                },
                                None => {
                                    // TODO: Figure out the integer type
                                    IntegerType::Generic(number.as_str().parse::<i64>().unwrap())
                                }
                            };
                            return Ok((TokenType::Integer(inttype), captures[0].len()));
                        }
                    }
                    None => panic!("Internal Error: `number` part of the regex is not captured"),
                }
            }
            None => panic!("Internal Error: Numeric Pattern not found!"),
        }
    }

    fn tokenize_char(&self, src: &str) -> Result<(TokenType, usize), CompilerError> {
        let (ch, bytes) = iter_while(&src[1..], |ch| ch != '\'');

        match ch.len() {
            1 => Ok((TokenType::Character(ch.chars().next().unwrap()), bytes + 2)),
            _ => Err(CompilerError {
                kind: CompilerErrorKind::TokenizerError,
                message: format!(
                    "A single quoted literal can only have 1 character and not: {}",
                    ch
                ),
                location: None,
            }),
        }
    }

    fn tokenize_string(&self, src: &str) -> Result<(TokenType, usize), CompilerError> {
        let (stringliteral, bytes) = iter_while(&src[1..], |ch| ch != '"');

        match src.chars().nth(bytes + 1) {
            Some('"') => Ok((
                TokenType::StringLiteral(stringliteral.to_string()),
                bytes + 2,
            )),
            _ => Err(CompilerError {
                kind: CompilerErrorKind::TokenizerError,
                message: "Missing \" in a quoted string literal".to_string(),
                location: None,
            }),
        }
    }

    // This function never returns any error, should the return type be changed?
    // Or kept as it is to be consistent with other functions?
    fn tokenize_identifier(&self, src: &str) -> Result<(TokenType, usize), CompilerError> {
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
            "restrict" => TokenType::Keyword(Keyword::Restrict),
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
            "_Alignas" => TokenType::Keyword(Keyword::_Alignas),
            "_Alignof" => TokenType::Keyword(Keyword::_Alignof),
            "_Atomic" => TokenType::Keyword(Keyword::_Atomic),
            "_Bool" => TokenType::Keyword(Keyword::_Bool),
            "_Complex" => TokenType::Keyword(Keyword::_Complex),
            "_Generic" => TokenType::Keyword(Keyword::_Generic),
            "_Imaginary" => TokenType::Keyword(Keyword::_Imaginary),
            "_Noreturn" => TokenType::Keyword(Keyword::_Noreturn),
            "_Static_assert" => TokenType::Keyword(Keyword::_Static_assert),
            "_Thread_local" => TokenType::Keyword(Keyword::_Thread_local),

            // Else it is really an identifier
            _ => TokenType::Identifier(identifier.to_string()),
        };
        Ok((tokentype, bytes))
    }
}
