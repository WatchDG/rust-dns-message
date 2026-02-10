#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Question<'a> {
    pub q_name: &'a [u8],
    pub q_type: QType,
    pub q_class: QClass,
}

impl<'a> Question<'a> {
    pub fn new(q_name: &'a [u8], q_type: QType, q_class: QClass) -> Self {
        Self {
            q_name,
            q_type,
            q_class,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QType {
    A,
    NS,
    CNAME,
    SOA,
    PTR,
    MX,
    TXT,
    AAAA,
    SRV,
    ANY,
    Unknown(u16),
}

impl QType {
    pub fn from_u16(value: u16) -> Self {
        match value {
            1 => QType::A,
            2 => QType::NS,
            5 => QType::CNAME,
            6 => QType::SOA,
            12 => QType::PTR,
            15 => QType::MX,
            16 => QType::TXT,
            28 => QType::AAAA,
            33 => QType::SRV,
            255 => QType::ANY,
            other => QType::Unknown(other),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            QType::A => 1,
            QType::NS => 2,
            QType::CNAME => 5,
            QType::SOA => 6,
            QType::PTR => 12,
            QType::MX => 15,
            QType::TXT => 16,
            QType::AAAA => 28,
            QType::SRV => 33,
            QType::ANY => 255,
            QType::Unknown(v) => v,
        }
    }

    pub fn from_question_bytes(h: u8, l: u8) -> Self {
        let value = ((h as u16) << 8) | (l as u16);
        QType::from_u16(value)
    }

    pub fn to_question_bytes(&self) -> (u8, u8) {
        let value = self.to_u16();
        let h = (value >> 8) as u8;
        let l = (value & 0xFF) as u8;
        (h, l)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QClass {
    IN,
    CS,
    CH,
    HS,
    ANY,
    Unknown(u16),
}

impl QClass {
    pub fn from_u16(value: u16) -> Self {
        match value {
            1 => QClass::IN,
            2 => QClass::CS,
            3 => QClass::CH,
            4 => QClass::HS,
            255 => QClass::ANY,
            other => QClass::Unknown(other),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            QClass::IN => 1,
            QClass::CS => 2,
            QClass::CH => 3,
            QClass::HS => 4,
            QClass::ANY => 255,
            QClass::Unknown(v) => v,
        }
    }

    pub fn from_question_bytes(h: u8, l: u8) -> Self {
        let value = ((h as u16) << 8) | (l as u16);
        QClass::from_u16(value)
    }

    pub fn to_question_bytes(&self) -> (u8, u8) {
        let value = self.to_u16();
        let h = (value >> 8) as u8;
        let l = (value & 0xFF) as u8;
        (h, l)
    }
}
