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
    pub op_code: OpCode,
    pub aa: AA,
    pub tc: TC,
    pub rd: RD,
    pub ra: RA,
    pub z: Z,
    pub ad: AD,
    pub cd: CD,
    pub r_code: RCode,
}

impl Flags {
    pub fn new(
        qr: QR,
        op_code: OpCode,
        aa: AA,
        tc: TC,
        rd: RD,
        ra: RA,
        z: Z,
        ad: AD,
        cd: CD,
        r_code: RCode,
    ) -> Self {
        Self {
            qr,
            op_code,
            aa,
            tc,
            rd,
            ra,
            z,
            ad,
            cd,
            r_code,
        }
    }

    pub fn from_flags_bytes(h: u8, l: u8) -> Self {
        Flags::new(
            QR::from_flags_byte(h),
            OpCode::from_flags_byte(h),
            AA::from_flags_byte(h),
            TC::from_flags_byte(h),
            RD::from_flags_byte(h),
            RA::from_flags_byte(l),
            Z::from_flags_byte(l),
            AD::from_flags_byte(l),
            CD::from_flags_byte(l),
            RCode::from_flags_byte(l),
        )
    }

    pub fn to_flags_bytes(&self) -> (u8, u8) {
        let h: u8 = self.qr.to_flags_byte_bits()
            | self.op_code.to_flags_byte_bits()
            | self.aa.to_flags_byte_bits()
            | self.tc.to_flags_byte_bits()
            | self.rd.to_flags_byte_bits();

        let l = self.ra.to_flags_byte_bits()
            | self.z.to_flags_byte_bits()
            | self.ad.to_flags_byte_bits()
            | self.cd.to_flags_byte_bits()
            | self.r_code.to_flags_byte_bits();

        (h, l)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QR {
    Query = 0 << 7,
    Response = 1 << 7,
}

impl QR {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & (1 << 7)) != 0 {
            QR::Response
        } else {
            QR::Query
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AA {
    NonAuthoritative = 0 << 2,
    Authoritative = 1 << 2,
}

impl AA {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & (1 << 2)) != 0 {
            AA::Authoritative
        } else {
            AA::NonAuthoritative
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TC {
    NotTruncated = 0 << 1,
    Truncated = 1 << 1,
}

impl TC {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & (1 << 1)) != 0 {
            TC::Truncated
        } else {
            TC::NotTruncated
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RD {
    RecursionNotDesired = 0,
    RecursionDesired = 1,
}

impl RD {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & 1) != 0 {
            RD::RecursionDesired
        } else {
            RD::RecursionNotDesired
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RA {
    RecursionNotAvailable = 0 << 7,
    RecursionAvailable = 1 << 7,
}

impl RA {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & (1 << 7)) != 0 {
            RA::RecursionAvailable
        } else {
            RA::RecursionNotAvailable
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RCode {
    NoError,
    FormErr,
    ServFail,
    NXDomain,
    NotImp,
    Refused,
    YXDomain,
    YXRRSet,
    NXRRSet,
    NotAuth,
    NotZone,
    Unassigned(u8),
}

impl RCode {
    pub fn from_flags_byte(byte: u8) -> Self {
        let v = byte & 0b1111;
        match v {
            0 => RCode::NoError,
            1 => RCode::FormErr,
            2 => RCode::ServFail,
            3 => RCode::NXDomain,
            4 => RCode::NotImp,
            5 => RCode::Refused,
            6 => RCode::YXDomain,
            7 => RCode::YXRRSet,
            8 => RCode::NXRRSet,
            9 => RCode::NotAuth,
            10 => RCode::NotZone,
            other => RCode::Unassigned(other),
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        let v = match self {
            RCode::NoError => 0,
            RCode::FormErr => 1,
            RCode::ServFail => 2,
            RCode::NXDomain => 3,
            RCode::NotImp => 4,
            RCode::Refused => 5,
            RCode::YXDomain => 6,
            RCode::YXRRSet => 7,
            RCode::NXRRSet => 8,
            RCode::NotAuth => 9,
            RCode::NotZone => 10,
            RCode::Unassigned(x) => x & 0b1111,
        };
        v
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Z {
    Reserved = 0 << 6,
}

impl Z {
    pub fn from_flags_byte(_byte: u8) -> Self {
        Z::Reserved
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AD {
    DataNotAuthenticated = 0 << 5,
    DataAuthenticated = 1 << 5,
}

impl AD {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & (1 << 5)) != 0 {
            AD::DataAuthenticated
        } else {
            AD::DataNotAuthenticated
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CD {
    CheckingEnabled = 0 << 4,
    CheckingDisabled = 1 << 4,
}

impl CD {
    pub fn from_flags_byte(byte: u8) -> Self {
        if (byte & (1 << 4)) != 0 {
            CD::CheckingDisabled
        } else {
            CD::CheckingEnabled
        }
    }

    pub fn to_flags_byte_bits(self) -> u8 {
        self as u8
    }
}
