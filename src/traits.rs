use crate::Question;

pub trait GetQuestions<'a> {
    fn get_questions(&self) -> &'a [Question<'a>];
}
