use crate::Span;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: &'static str,
    pub location: Span,
}
