use crate::{Additional, Answer, Authority, Header, Question};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message<'a> {
    pub header: Header,
    pub question: Vec<Question<'a>>,
    pub answer: Vec<Answer<'a>>,
    pub authority: Vec<Authority<'a>>,
    pub additional: Vec<Additional<'a>>,
}

impl<'a> Message<'a> {
    pub fn new(
        header: Header,
        question: Vec<Question<'a>>,
        answer: Vec<Answer<'a>>,
        authority: Vec<Authority<'a>>,
        additional: Vec<Additional<'a>>,
    ) -> Self {
        Self {
            header,
            question,
            answer,
            authority,
            additional,
        }
    }
}
