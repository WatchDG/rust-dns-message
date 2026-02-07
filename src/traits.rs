use crate::{Additional, Answer, Authority, Question};

pub trait GetQuestions<'a> {
    fn get_questions(&'a self) -> &'a [Question<'a>];
}

impl<'a> GetQuestions<'a> for Vec<Question<'a>> {
    fn get_questions(&'a self) -> &'a [Question<'a>] {
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
