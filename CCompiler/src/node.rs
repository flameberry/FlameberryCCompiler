use std::{fmt, ops::Add, usize};

#[derive(Debug, Copy, Clone)]
pub struct TokenPosition {
    line: usize,   // The line number in the source code
    column: usize, // The offset from the start of the line
}

impl fmt::Display for TokenPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Add for TokenPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        TokenPosition {
            line: self.line + rhs.line,
            column: self.column + rhs.column,
        }
    }
}

impl TokenPosition {
    pub fn none() -> Self {
        TokenPosition { line: 0, column: 0 }
    }

    pub fn new(line: usize, column: usize) -> Self {
        TokenPosition { line, column }
    }
}

// Byte offset in the input buffer
#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub start: TokenPosition,
    pub end: TokenPosition,
}

impl Span {
    pub fn new(start: TokenPosition, end: TokenPosition) -> Self {
        Span { start, end }
    }

    pub fn none() -> Self {
        Span {
            start: TokenPosition::none(),
            end: TokenPosition::none(),
        }
    }
}

// For a Span { start: 10, end: 16 } the debug string will be <10...16>
impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}..{}>", self.start, self.end)
    }
}

// Implementation of a basic node in the Abstract Syntax Tree
#[derive(Debug, Clone)]
pub struct Node<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Node<T> {
    /// Create new node
    pub fn new(node: T, span: Span) -> Node<T> {
        Node { node, span }
    }
}

impl<T> fmt::Display for Node<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.node, self.span)
    }
}
