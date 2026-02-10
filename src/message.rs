use std::marker::PhantomData;

use crate::traits::GetLabels;
use crate::{
    Header,
    traits::{GetAdditionals, GetAnswers, GetAuthorities, GetQuestions},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message<
    'a,
    GQS: GetQuestions<'a, GL>,
    GAN: GetAnswers<'a>,
    GNS: GetAuthorities<'a>,
    GAD: GetAdditionals<'a>,
    GL: GetLabels<'a>,
> {
    pub header: Header,
    pub questions: GQS,
    pub answers: GAN,
    pub authorities: GNS,
    pub additionals: GAD,
    _phantom: PhantomData<&'a GL>,
}

impl<
    'a,
    GL: GetLabels<'a>,
    GQ: GetQuestions<'a, GL>,
    GANS: GetAnswers<'a>,
    GNS: GetAuthorities<'a>,
    GAD: GetAdditionals<'a>,
> Message<'a, GQ, GANS, GNS, GAD, GL>
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
