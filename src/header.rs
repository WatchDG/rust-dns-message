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
    pub qr: QR,
    pub op_code: u8,
    pub aa: AA,
    pub tc: TC,
    pub rd: RD,
    pub ra: bool,
    pub z: u8,
    pub r_code: u8,
}

impl Flags {
    pub fn new(qr: QR, op_code: u8, aa: AA, tc: TC, rd: RD, ra: bool, z: u8, r_code: u8) -> Self {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QR {
    Query = 0 << 7,
    Response = 1 << 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AA {
    False = 0 << 2,
    True = 1 << 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TC {
    False = 0 << 1,
    True = 1 << 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RD {
    False = 0,
    True = 1,
}
