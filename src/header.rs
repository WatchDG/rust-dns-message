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
    pub opcode: OpCode,
    pub aa: AuthoritativeAnswer,
    pub tc: Truncation,
    pub rd: RecursionDesired,
    pub ra: RecursionAvailable,
    pub z: ReservedZ,
    pub r_code: u8,
}

impl Flags {
    pub fn new(
        qr: QueryResponse,
        opcode: OpCode,
        aa: AuthoritativeAnswer,
        tc: Truncation,
        rd: RecursionDesired,
        ra: RecursionAvailable,
        z: ReservedZ,
        r_code: u8,
    ) -> Self {
        Self {
            qr,
            opcode,
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

impl QueryResponse {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & (1 << 7)) != 0 {
            QueryResponse::Response
        } else {
            QueryResponse::Query
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AuthoritativeAnswer {
    NonAuthoritative = 0 << 2,
    Authoritative = 1 << 2,
}

impl AuthoritativeAnswer {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & (1 << 2)) != 0 {
            AuthoritativeAnswer::Authoritative
        } else {
            AuthoritativeAnswer::NonAuthoritative
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Truncation {
    NotTruncated = 0 << 1,
    Truncated = 1 << 1,
}

impl Truncation {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & (1 << 1)) != 0 {
            Truncation::Truncated
        } else {
            Truncation::NotTruncated
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RecursionDesired {
    RecursionNotDesired = 0,
    RecursionDesired = 1,
}

impl RecursionDesired {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & 1) != 0 {
            RecursionDesired::RecursionDesired
        } else {
            RecursionDesired::RecursionNotDesired
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RecursionAvailable {
    RecursionNotAvailable = 0 << 7,
    RecursionAvailable = 1 << 7,
}

impl RecursionAvailable {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & (1 << 7)) != 0 {
            RecursionAvailable::RecursionAvailable
        } else {
            RecursionAvailable::RecursionNotAvailable
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReservedZ {
    Reserved,
    NonStandard(u8),
}

impl ReservedZ {
    pub fn from_flags_byte(byte: u8) -> Self {
        let v = (byte >> 4) & 0b111;
        if v == 0 {
            ReservedZ::Reserved
        } else {
            ReservedZ::NonStandard(v)
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        let z = match self {
            ReservedZ::Reserved => 0,
            ReservedZ::NonStandard(v) => v & 0b111,
        };
        z << 4
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Query,
    IQuery,
    Status,
    Notify,
    Update,
    Unassigned(u8),
}

impl OpCode {
    pub fn from_flags_byte(byte: u8) -> Self {
        let v = (byte >> 3) & 0b1111;
        match v {
            0 => OpCode::Query,
            1 => OpCode::IQuery,
            2 => OpCode::Status,
            4 => OpCode::Notify,
            5 => OpCode::Update,
            other => OpCode::Unassigned(other),
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        let v = match self {
            OpCode::Query => 0,
            OpCode::IQuery => 1,
            OpCode::Status => 2,
            OpCode::Notify => 4,
            OpCode::Update => 5,
            OpCode::Unassigned(x) => x & 0b1111,
        };
        v << 3
    }
}
