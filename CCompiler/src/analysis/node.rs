use std::{fmt, ops::Add, usize};

#[derive(Debug, Copy, Clone)]
pub struct FileLocation {
    line: usize,   // The line number in the source code
    column: usize, // The offset from the start of the line
}

impl fmt::Display for FileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Add for FileLocation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        FileLocation {
            line: self.line + rhs.line,
            column: self.column + rhs.column,
        }
    }
}

impl FileLocation {
    pub fn none() -> Self {
        FileLocation { line: 0, column: 0 }
    }

    pub fn new(line: usize, column: usize) -> Self {
        FileLocation { line, column }
    }
}

// Byte offset in the input buffer
#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub start: FileLocation,
    pub end: FileLocation,
}

impl Span {
    pub fn new(start: FileLocation, end: FileLocation) -> Self {
        Span { start, end }
    }

    pub fn none() -> Self {
        Span {
            start: FileLocation::none(),
            end: FileLocation::none(),
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
