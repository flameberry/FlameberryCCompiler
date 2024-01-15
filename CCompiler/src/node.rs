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
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<{}..{}>", self.start, self.end)
    }
}

// Implementation of a basic node in the Abstract Syntax Tree
#[derive(Debug, Clone)]
pub struct Node<T> {
    node: T,
    span: Span,
}

impl<T> Node<T> {
    /// Create new node
    pub fn new(node: T, span: Span) -> Node<T> {
        Node {
            node: node,
            span: span,
        }
    }
}
