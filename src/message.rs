use crate::{Header, Question};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message<'a> {
    pub header: Header,
    pub question: &'a [Question<'a>],
}

impl<'a> Message<'a> {
    pub fn new(header: Header, question: &'a [Question<'a>]) -> Self {
        Self { header, question }
    }
}
