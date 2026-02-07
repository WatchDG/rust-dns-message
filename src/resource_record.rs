#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceRecord<'a> {
    pub rr_name: &'a [u8],
    pub rr_type: u16,
    pub rr_class: u16,
    pub rr_ttl: u32,
    pub rr_rd_length: u16,
    pub rr_data: &'a [u8],
}

impl<'a> ResourceRecord<'a> {
    pub fn new(
        rr_name: &'a [u8],
        rr_type: u16,
        rr_class: u16,
        rr_ttl: u32,
        rr_rd_length: u16,
        rr_data: &'a [u8],
    ) -> Self {
        Self {
            rr_name,
            rr_type,
            rr_class,
            rr_ttl,
            rr_rd_length,
            rr_data,
        }
    }
}
