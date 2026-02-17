use crate::header::{Flags, Header};
use crate::message::Message;
use crate::question::{Label, NameElement, Question};
use crate::resource_record::ResourceRecord;
use mt::WireLength;

impl WireLength<usize> for Header {
    fn wire_length(&self) -> usize {
        12
    }
}

impl WireLength<usize> for &Flags {
    fn wire_length(&self) -> usize {
        2
    }
}

impl<'a> WireLength<usize> for &Label<'a> {
    fn wire_length(&self) -> usize {
        1 + self.data.len()
    }
}

impl<'a> WireLength<usize> for NameElement<'a> {
    fn wire_length(&self) -> usize {
        match self {
            NameElement::Label(l) => l.wire_length(),
            NameElement::Pointer(_) => 2,
            NameElement::Root => 1,
            NameElement::Reserved => 0,
        }
    }
}

impl<'a> WireLength<usize> for Question<'a> {
    fn wire_length(&self) -> usize {
        self.q_name.wire_length() + 4
    }
}

impl<'a> WireLength<usize> for ResourceRecord<'a> {
    fn wire_length(&self) -> usize {
        self.rr_name.wire_length() + 10 + self.rr_data.len()
    }
}

impl<'a> WireLength<usize> for Message<'a> {
    fn wire_length(&self) -> usize {
        self.header.wire_length()
            + self.question.wire_length()
            + self.answer.wire_length()
            + self.authority.wire_length()
            + self.additional.wire_length()
    }
}
