#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub id: u16,
    pub flags: Flags,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

impl Header {
    pub fn new(
        id: u16,
        flags: Flags,
        qd_count: u16,
        an_count: u16,
        ns_count: u16,
        ar_count: u16,
    ) -> Self {
        Self {
            id,
            flags,
            qd_count,
            an_count,
            ns_count,
            ar_count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags {
    pub qr: bool,
    pub op_code: u8,
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub z: u8,
    pub r_code: u8,
}

impl Flags {
    pub fn new(
        qr: bool,
        op_code: u8,
        aa: bool,
        tc: bool,
        rd: bool,
        ra: bool,
        z: u8,
        r_code: u8,
    ) -> Self {
        Self {
            qr,
            op_code,
            aa,
            tc,
            rd,
            ra,
            z,
            r_code,
        }
    }
}
