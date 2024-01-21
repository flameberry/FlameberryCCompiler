use std::fmt;

// Byte offset in the input buffer
#[derive(Copy, Clone)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    pub fn none() -> Self {
        Span { start: 0, end: 0 }
    }
}

// For a Span { start: 10, end: 16 } the debug string will be <10...16>
impl fmt::Debug for Span {
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
        write!(f, "{} {:?}", self.node, self.span)
    }
}
