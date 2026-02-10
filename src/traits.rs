use crate::{Additional, Answer, Authority, Question, question::Label};

pub trait GetQuestions<'a, GL: GetLabels<'a>> {
    fn get_questions(&'a self) -> &'a [Question<'a, GL>];
}

impl<'a, GL: GetLabels<'a>> GetQuestions<'a, GL> for Vec<Question<'a, GL>> {
    fn get_questions(&'a self) -> &'a [Question<'a, GL>] {
        self.as_slice()
    }
}

pub trait GetAnswers<'a> {
    fn get_answers(&'a self) -> &'a [Answer<'a>];
}

impl<'a> GetAnswers<'a> for Vec<Answer<'a>> {
    fn get_answers(&'a self) -> &'a [Answer<'a>] {
        self.as_slice()
    }
}

pub trait GetAuthorities<'a> {
    fn get_authorities(&'a self) -> &'a [Authority<'a>];
}

impl<'a> GetAuthorities<'a> for Vec<Authority<'a>> {
    fn get_authorities(&'a self) -> &'a [Authority<'a>] {
        self.as_slice()
    }
}

pub trait GetAdditionals<'a> {
    fn get_additionals(&'a self) -> &'a [Additional<'a>];
}

impl<'a> GetAdditionals<'a> for Vec<Additional<'a>> {
    fn get_additionals(&'a self) -> &'a [Additional<'a>] {
        self.as_slice()
    }
}

pub trait GetLabels<'a> {
    fn get_labels(&'a self) -> &'a [Label<'a>];
}

impl<'a> GetLabels<'a> for &'a [Label<'a>] {
    fn get_labels(&'a self) -> &'a [Label<'a>] {
        *self
    }
}

impl<'a> GetLabels<'a> for Vec<Label<'a>> {
    fn get_labels(&'a self) -> &'a [Label<'a>] {
        self.as_slice()
    }
}
