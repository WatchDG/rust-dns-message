#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Question<'a> {
    pub q_name: &'a [u8],
    pub q_type: u16,
    pub q_class: u16,
}

impl<'a> Question<'a> {
    pub fn new(q_name: &'a [u8], q_type: u16, q_class: u16) -> Self {
        Self {
            q_name,
            q_type,
            q_class,
        }
    }
}
