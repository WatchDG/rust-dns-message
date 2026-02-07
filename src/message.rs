use crate::{Header, Question};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message<'a> {
    pub header: Header,
    pub questions: &'a [Question<'a>],
}

impl<'a> Message<'a> {
    pub fn new(header: Header, questions: &'a [Question<'a>]) -> Self {
        Self { header, questions }
    }
}
