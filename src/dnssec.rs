use crate::question::Name;

pub const RR_TYPE_DS: u16 = 43;
pub const RR_TYPE_RRSIG: u16 = 46;
pub const RR_TYPE_NSEC: u16 = 47;
pub const RR_TYPE_DNSKEY: u16 = 48;
pub const RR_TYPE_NSEC3: u16 = 50;
pub const RR_TYPE_NSEC3PARAM: u16 = 51;
pub const RR_TYPE_CDS: u16 = 59;
pub const RR_TYPE_CDNSKEY: u16 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnssecRrType {
    DS,
    RRSIG,
    NSEC,
    DNSKEY,
    NSEC3,
    NSEC3PARAM,
    CDS,
    CDNSKEY,
    Unknown(u16),
}

impl DnssecRrType {
    pub fn from_u16(value: u16) -> Self {
        match value {
            RR_TYPE_DS => Self::DS,
            RR_TYPE_RRSIG => Self::RRSIG,
            RR_TYPE_NSEC => Self::NSEC,
            RR_TYPE_DNSKEY => Self::DNSKEY,
            RR_TYPE_NSEC3 => Self::NSEC3,
            RR_TYPE_NSEC3PARAM => Self::NSEC3PARAM,
            RR_TYPE_CDS => Self::CDS,
            RR_TYPE_CDNSKEY => Self::CDNSKEY,
            other => Self::Unknown(other),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            Self::DS => RR_TYPE_DS,
            Self::RRSIG => RR_TYPE_RRSIG,
            Self::NSEC => RR_TYPE_NSEC,
            Self::DNSKEY => RR_TYPE_DNSKEY,
            Self::NSEC3 => RR_TYPE_NSEC3,
            Self::NSEC3PARAM => RR_TYPE_NSEC3PARAM,
            Self::CDS => RR_TYPE_CDS,
            Self::CDNSKEY => RR_TYPE_CDNSKEY,
            Self::Unknown(v) => v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnssecAlgorithm {
    Rsamd5,
    Dh,
    Dsa,
    RsaSha1,
    DsaNsec3Sha1,
    RsaSha1Nsec3Sha1,
    RsaSha256,
    RsaSha512,
    EccGost,
    EcdsaP256Sha256,
    EcdsaP384Sha384,
    Ed25519,
    Ed448,
    Indirect,
    PrivateDns,
    PrivateOid,
    Unknown(u8),
}

impl DnssecAlgorithm {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Rsamd5,
            2 => Self::Dh,
            3 => Self::Dsa,
            5 => Self::RsaSha1,
            6 => Self::DsaNsec3Sha1,
            7 => Self::RsaSha1Nsec3Sha1,
            8 => Self::RsaSha256,
            10 => Self::RsaSha512,
            12 => Self::EccGost,
            13 => Self::EcdsaP256Sha256,
            14 => Self::EcdsaP384Sha384,
            15 => Self::Ed25519,
            16 => Self::Ed448,
            252 => Self::Indirect,
            253 => Self::PrivateDns,
            254 => Self::PrivateOid,
            other => Self::Unknown(other),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match *self {
            Self::Rsamd5 => 1,
            Self::Dh => 2,
            Self::Dsa => 3,
            Self::RsaSha1 => 5,
            Self::DsaNsec3Sha1 => 6,
            Self::RsaSha1Nsec3Sha1 => 7,
            Self::RsaSha256 => 8,
            Self::RsaSha512 => 10,
            Self::EccGost => 12,
            Self::EcdsaP256Sha256 => 13,
            Self::EcdsaP384Sha384 => 14,
            Self::Ed25519 => 15,
            Self::Ed448 => 16,
            Self::Indirect => 252,
            Self::PrivateDns => 253,
            Self::PrivateOid => 254,
            Self::Unknown(v) => v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DsDigestType {
    Sha1,
    Sha256,
    GostR3411_94,
    Sha384,
    Unknown(u8),
}

impl DsDigestType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Sha1,
            2 => Self::Sha256,
            3 => Self::GostR3411_94,
            4 => Self::Sha384,
            other => Self::Unknown(other),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match *self {
            Self::Sha1 => 1,
            Self::Sha256 => 2,
            Self::GostR3411_94 => 3,
            Self::Sha384 => 4,
            Self::Unknown(v) => v,
        }
    }
}

pub const DNSKEY_FLAG_ZONE: u16 = 0x0100;
pub const DNSKEY_FLAG_REVOKE: u16 = 0x0080;
pub const DNSKEY_FLAG_SEP: u16 = 0x0001;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnskeyRdata<'a> {
    pub flags: u16,
    pub protocol: u8,
    pub algorithm: DnssecAlgorithm,
    pub public_key: &'a [u8],
}

impl<'a> DnskeyRdata<'a> {
    pub fn new(flags: u16, protocol: u8, algorithm: DnssecAlgorithm, public_key: &'a [u8]) -> Self {
        Self {
            flags,
            protocol,
            algorithm,
            public_key,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DsRdata<'a> {
    pub key_tag: u16,
    pub algorithm: DnssecAlgorithm,
    pub digest_type: DsDigestType,
    pub digest: &'a [u8],
}

impl<'a> DsRdata<'a> {
    pub fn new(
        key_tag: u16,
        algorithm: DnssecAlgorithm,
        digest_type: DsDigestType,
        digest: &'a [u8],
    ) -> Self {
        Self {
            key_tag,
            algorithm,
            digest_type,
            digest,
        }
    }
}

pub type CdsRdata<'a> = DsRdata<'a>;
pub type CdnskeyRdata<'a> = DnskeyRdata<'a>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RrsigRdata<'a> {
    pub type_covered: u16,
    pub algorithm: DnssecAlgorithm,
    pub labels: u8,
    pub original_ttl: u32,
    pub signature_expiration: u32,
    pub signature_inception: u32,
    pub key_tag: u16,
    pub signer_name: Name<'a>,
    pub signature: &'a [u8],
}

impl<'a> RrsigRdata<'a> {
    pub fn new(
        type_covered: u16,
        algorithm: DnssecAlgorithm,
        labels: u8,
        original_ttl: u32,
        signature_expiration: u32,
        signature_inception: u32,
        key_tag: u16,
        signer_name: Name<'a>,
        signature: &'a [u8],
    ) -> Self {
        Self {
            type_covered,
            algorithm,
            labels,
            original_ttl,
            signature_expiration,
            signature_inception,
            key_tag,
            signer_name,
            signature,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NsecRdata<'a> {
    pub next_domain_name: Name<'a>,
    pub type_bit_maps: &'a [u8],
}

impl<'a> NsecRdata<'a> {
    pub fn new(next_domain_name: Name<'a>, type_bit_maps: &'a [u8]) -> Self {
        Self {
            next_domain_name,
            type_bit_maps,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nsec3Rdata<'a> {
    pub hash_algorithm: u8,
    pub flags: u8,
    pub iterations: u16,
    pub salt: &'a [u8],
    pub next_hashed_owner_name: &'a [u8],
    pub type_bit_maps: &'a [u8],
}

impl<'a> Nsec3Rdata<'a> {
    pub fn new(
        hash_algorithm: u8,
        flags: u8,
        iterations: u16,
        salt: &'a [u8],
        next_hashed_owner_name: &'a [u8],
        type_bit_maps: &'a [u8],
    ) -> Self {
        Self {
            hash_algorithm,
            flags,
            iterations,
            salt,
            next_hashed_owner_name,
            type_bit_maps,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nsec3paramRdata<'a> {
    pub hash_algorithm: u8,
    pub flags: u8,
    pub iterations: u16,
    pub salt: &'a [u8],
}

impl<'a> Nsec3paramRdata<'a> {
    pub fn new(hash_algorithm: u8, flags: u8, iterations: u16, salt: &'a [u8]) -> Self {
        Self {
            hash_algorithm,
            flags,
            iterations,
            salt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dnssec_rr_type_roundtrip_to_from_u16() {
        let cases = [
            DnssecRrType::DS,
            DnssecRrType::RRSIG,
            DnssecRrType::NSEC,
            DnssecRrType::DNSKEY,
            DnssecRrType::NSEC3,
            DnssecRrType::NSEC3PARAM,
            DnssecRrType::CDS,
            DnssecRrType::CDNSKEY,
            DnssecRrType::Unknown(0),
            DnssecRrType::Unknown(65280),
        ];

        for rr_type in cases {
            assert_eq!(DnssecRrType::from_u16(rr_type.to_u16()), rr_type);
        }
    }

    #[test]
    fn dnssec_algorithm_roundtrip_to_from_u8() {
        let cases = [
            DnssecAlgorithm::Rsamd5,
            DnssecAlgorithm::Dh,
            DnssecAlgorithm::Dsa,
            DnssecAlgorithm::RsaSha1,
            DnssecAlgorithm::DsaNsec3Sha1,
            DnssecAlgorithm::RsaSha1Nsec3Sha1,
            DnssecAlgorithm::RsaSha256,
            DnssecAlgorithm::RsaSha512,
            DnssecAlgorithm::EccGost,
            DnssecAlgorithm::EcdsaP256Sha256,
            DnssecAlgorithm::EcdsaP384Sha384,
            DnssecAlgorithm::Ed25519,
            DnssecAlgorithm::Ed448,
            DnssecAlgorithm::Indirect,
            DnssecAlgorithm::PrivateDns,
            DnssecAlgorithm::PrivateOid,
            DnssecAlgorithm::Unknown(0),
            DnssecAlgorithm::Unknown(255),
        ];

        for alg in cases {
            assert_eq!(DnssecAlgorithm::from_u8(alg.to_u8()), alg);
        }
    }

    #[test]
    fn ds_digest_type_roundtrip_to_from_u8() {
        let cases = [
            DsDigestType::Sha1,
            DsDigestType::Sha256,
            DsDigestType::GostR3411_94,
            DsDigestType::Sha384,
            DsDigestType::Unknown(0),
            DsDigestType::Unknown(255),
        ];

        for dt in cases {
            assert_eq!(DsDigestType::from_u8(dt.to_u8()), dt);
        }
    }
}
