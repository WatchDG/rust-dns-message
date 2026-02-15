use crate::{Header, Message, NameElement, Question, ResourceRecord};

pub fn header_wire_length(_header: &Header) -> usize {
    12
}

pub fn name_wire_length(name: &[NameElement<'_>]) -> usize {
    name.iter()
        .map(|e| match e {
            NameElement::Label(l) => 1 + l.data.len(),
            NameElement::Pointer(_) => 2,
            NameElement::Root => 1,
            NameElement::Reserved => 0,
        })
        .sum()
}

pub fn question_wire_length(questions: &[Question<'_>]) -> usize {
    questions
        .iter()
        .map(|q| name_wire_length(&q.q_name) + 4)
        .sum()
}

pub fn resource_record_wire_length(records: &[ResourceRecord<'_>]) -> usize {
    records
        .iter()
        .map(|r| name_wire_length(&r.rr_name) + 10 + r.rr_data.len())
        .sum()
}

pub fn message_wire_length<'a>(message: &Message<'a>) -> usize {
    let length = header_wire_length(&message.header);
    let length = length + question_wire_length(&message.question);
    let length = length + resource_record_wire_length(&message.answer);
    let length = length + resource_record_wire_length(&message.authority);
    let length = length + resource_record_wire_length(&message.additional);
    length
}
