use std::marker::PhantomData;

use crate::{Header, traits::GetQuestions};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message<'a, GQ: GetQuestions<'a>> {
    pub header: Header,
    pub questions: GQ,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, GQ: GetQuestions<'a>> Message<'a, GQ> {
    pub fn new(header: Header, questions: GQ) -> Self {
        Self {
            header,
            questions,
            _phantom: PhantomData,
        }
    }
}
