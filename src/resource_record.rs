use crate::question::NameElement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceRecord<'a> {
    pub rr_name: Vec<NameElement<'a>>,
    pub rr_type: RRType,
    pub rr_class: RRClass,
    pub rr_ttl: u32,
    pub rr_rd_length: u16,
    pub rr_data: &'a [u8],
}

impl<'a> ResourceRecord<'a> {
    pub fn new(
        rr_name: Vec<NameElement<'a>>,
        rr_type: RRType,
        rr_class: RRClass,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RRType {
    A,
    NS,
    CNAME,
    SOA,
    PTR,
    MX,
    TXT,
    AAAA,
    SRV,
    Unknown(u16),
}

impl RRType {
    pub fn from_u16(value: u16) -> Self {
        match value {
            1 => RRType::A,
            2 => RRType::NS,
            5 => RRType::CNAME,
            6 => RRType::SOA,
            12 => RRType::PTR,
            15 => RRType::MX,
            16 => RRType::TXT,
            28 => RRType::AAAA,
            33 => RRType::SRV,
            other => RRType::Unknown(other),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            RRType::A => 1,
            RRType::NS => 2,
            RRType::CNAME => 5,
            RRType::SOA => 6,
            RRType::PTR => 12,
            RRType::MX => 15,
            RRType::TXT => 16,
            RRType::AAAA => 28,
            RRType::SRV => 33,
            RRType::Unknown(v) => v,
        }
    }

    pub fn from_rr_bytes(h: u8, l: u8) -> Self {
        let value = ((h as u16) << 8) | (l as u16);
        RRType::from_u16(value)
    }

    pub fn to_rr_bytes(&self) -> (u8, u8) {
        let value = self.to_u16();
        let h = (value >> 8) as u8;
        let l = (value & 0xFF) as u8;
        (h, l)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RRClass {
    IN,
    CS,
    CH,
    HS,
    Unknown(u16),
}

impl RRClass {
    pub fn from_u16(value: u16) -> Self {
        match value {
            1 => RRClass::IN,
            2 => RRClass::CS,
            3 => RRClass::CH,
            4 => RRClass::HS,
            other => RRClass::Unknown(other),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            RRClass::IN => 1,
            RRClass::CS => 2,
            RRClass::CH => 3,
            RRClass::HS => 4,
            RRClass::Unknown(v) => v,
        }
    }

    pub fn from_rr_bytes(h: u8, l: u8) -> Self {
        let value = ((h as u16) << 8) | (l as u16);
        RRClass::from_u16(value)
    }

    pub fn to_rr_bytes(&self) -> (u8, u8) {
        let value = self.to_u16();
        let h = (value >> 8) as u8;
        let l = (value & 0xFF) as u8;
        (h, l)
    }
}
