use std::marker::PhantomData;

use crate::{
    Header,
    traits::{GetAdditionals, GetAnswers, GetAuthorities, GetQuestions},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message<
    'a,
    GQS: GetQuestions<'a>,
    GAN: GetAnswers<'a>,
    GNS: GetAuthorities<'a>,
    GAD: GetAdditionals<'a>,
> {
    pub header: Header,
    pub questions: GQS,
    pub answers: GAN,
    pub authorities: GNS,
    pub additionals: GAD,
    _phantom: PhantomData<&'a ()>,
}

impl<
    'a,
    GQ: GetQuestions<'a>,
    GANS: GetAnswers<'a>,
    GNS: GetAuthorities<'a>,
    GAD: GetAdditionals<'a>,
> Message<'a, GQ, GANS, GNS, GAD>
{
    pub fn new(
        header: Header,
        questions: GQ,
        answers: GANS,
        authorities: GNS,
        additionals: GAD,
    ) -> Self {
        Self {
            header,
            questions,
            answers,
            authorities,
            additionals,
            _phantom: PhantomData,
        }
    }
}
