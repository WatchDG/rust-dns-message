use crate::Question;

pub trait GetQuestions<'a> {
    fn get_questions(&'a self) -> &'a [Question<'a>];
}

impl<'a> GetQuestions<'a> for Vec<Question<'a>> {
    fn get_questions(&'a self) -> &'a [Question<'a>] {
        self.as_slice()
    }
}
