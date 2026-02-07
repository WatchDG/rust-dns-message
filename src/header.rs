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
    pub qr: QueryResponse,
    pub op_code: u8,
    pub aa: AuthoritativeAnswer,
    pub tc: Truncation,
    pub rd: RecursionDesired,
    pub ra: bool,
    pub z: u8,
    pub r_code: u8,
}

impl Flags {
    pub fn new(
        qr: QueryResponse,
        op_code: u8,
        aa: AuthoritativeAnswer,
        tc: Truncation,
        rd: RecursionDesired,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QueryResponse {
    Query = 0 << 7,
    Response = 1 << 7,
}

impl From<u8> for QueryResponse {
    fn from(b: u8) -> Self {
        if (b & (1 << 7)) != 0 {
            QueryResponse::Response
        } else {
            QueryResponse::Query
        }
    }
}

impl From<QueryResponse> for u8 {
    fn from(qr: QueryResponse) -> Self {
        qr as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AuthoritativeAnswer {
    NonAuthoritative = 0 << 2,
    Authoritative = 1 << 2,
}

impl From<u8> for AuthoritativeAnswer {
    fn from(v: u8) -> Self {
        if (v & (1 << 2)) != 0 {
            AuthoritativeAnswer::Authoritative
        } else {
            AuthoritativeAnswer::NonAuthoritative
        }
    }
}

impl From<AuthoritativeAnswer> for u8 {
    fn from(aa: AuthoritativeAnswer) -> Self {
        aa as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Truncation {
    NotTruncated = 0 << 1,
    Truncated = 1 << 1,
}

impl From<u8> for Truncation {
    fn from(v: u8) -> Self {
        if (v & (1 << 1)) != 0 {
            Truncation::Truncated
        } else {
            Truncation::NotTruncated
        }
    }
}

impl From<Truncation> for u8 {
    fn from(tc: Truncation) -> Self {
        tc as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RecursionDesired {
    RecursionNotDesired = 0,
    RecursionDesired = 1,
}

impl From<u8> for RecursionDesired {
    fn from(v: u8) -> Self {
        if (v & 1) != 0 {
            RecursionDesired::RecursionDesired
        } else {
            RecursionDesired::RecursionNotDesired
        }
    }
}

impl From<RecursionDesired> for u8 {
    fn from(rd: RecursionDesired) -> Self {
        rd as u8
    }
}
