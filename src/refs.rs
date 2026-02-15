//! Offset-based references into a DNS message buffer.
//!
//! All structures operate only with offsets relative to the message start.
//! No slices or lifetimes — suitable for zero-copy parsing and compression tables.
//!
//! Name elements are explicitly distinguished: Label, Pointer, Root, Reserved.

use core::fmt;

/// Offset from the start of the DNS message buffer.
/// DNS messages are limited to 65535 bytes (RFC 1035).
pub type MsgOffset = u16;

/// Length of the DNS header in bytes.
pub const HEADER_LEN: MsgOffset = 12;

/// First byte of a pointer: top 2 bits must be `11`.
pub const POINTER_MASK: u8 = 0xC0;

/// Mask for the 14-bit offset in a pointer (full value).
pub const POINTER_OFFSET_MASK: u16 = 0x3FFF;

/// Mask for lower 6 bits of pointer's first byte (high part of offset).
pub const POINTER_HIGH_MASK: u8 = 0x3F;

/// Maximum label length (RFC 1035).
pub const LABEL_MAX_LEN: u8 = 63;

/// Root label: single zero byte marking end of name.
pub const ROOT_LABEL: u8 = 0;

// -----------------------------------------------------------------------------
// Name element kinds (wire format)
// -----------------------------------------------------------------------------

/// A single element in a domain name wire representation.
///
/// Distinguishes between:
/// - **Label**: `[length][data...]` — length 1–63, top 2 bits of length byte = 00
/// - **Pointer**: 2 bytes, top 2 bits = 11, lower 14 bits = offset from message start
/// - **Root**: single 0x00 byte
/// - **Reserved**: top 2 bits = 01 or 10 (RFC 1035 reserved for future use)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameElementRef {
    /// Normal label: offset points to the length byte.
    /// Wire: `[len][data...]` where len ∈ 1..=63.
    Label { offset: MsgOffset },

    /// Compression pointer: offset points to the first byte of the 2-byte pointer.
    /// Wire: `[0b11xxxxxx][xxxxxxxx]` — 14-bit offset from message start.
    Pointer { offset: MsgOffset },

    /// Root label: offset points to the 0x00 byte (end of name).
    Root { offset: MsgOffset },

    /// Reserved: top 2 bits = 01 or 10 (RFC 1035).
    Reserved { offset: MsgOffset },
}

impl NameElementRef {
    /// Returns the offset of this element in the message buffer.
    #[inline(always)]
    pub fn offset(&self) -> MsgOffset {
        match *self {
            NameElementRef::Label { offset }
            | NameElementRef::Pointer { offset }
            | NameElementRef::Root { offset }
            | NameElementRef::Reserved { offset } => offset,
        }
    }

    /// Classify a byte at `buf[offset]` as a name element kind.
    /// Does not validate bounds or pointer second byte.
    #[inline]
    pub fn classify_first_byte(byte: u8) -> NameElementKind {
        if byte == ROOT_LABEL {
            NameElementKind::Root
        } else if (byte & POINTER_MASK) == POINTER_MASK {
            NameElementKind::Pointer
        } else if (byte & POINTER_MASK) == 0 && byte <= LABEL_MAX_LEN {
            NameElementKind::Label
        } else {
            NameElementKind::Reserved
        }
    }

    /// Create a ref from offset, classifying using `buf[offset]`.
    #[inline]
    pub fn from_offset(buf: &[u8], offset: MsgOffset) -> Option<Self> {
        let off = offset as usize;
        if off >= buf.len() {
            return None;
        }
        let kind = Self::classify_first_byte(buf[off]);
        Some(match kind {
            NameElementKind::Label => NameElementRef::Label { offset },
            NameElementKind::Pointer => NameElementRef::Pointer { offset },
            NameElementKind::Root => NameElementRef::Root { offset },
            NameElementKind::Reserved => NameElementRef::Reserved { offset },
        })
    }
}

impl fmt::Debug for NameElementRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Label { offset } => f.debug_struct("Label").field("offset", offset).finish(),
            Self::Pointer { offset } => f.debug_struct("Pointer").field("offset", offset).finish(),
            Self::Root { offset } => f.debug_struct("Root").field("offset", offset).finish(),
            Self::Reserved { offset } => {
                f.debug_struct("Reserved").field("offset", offset).finish()
            }
        }
    }
}

/// Classification of the first byte of a name element (no allocation).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NameElementKind {
    Label,
    Pointer,
    Root,
    Reserved,
}

// -----------------------------------------------------------------------------
// Message section refs
// -----------------------------------------------------------------------------

/// Reference to the header: always at offset 0, length 12.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HeaderRef;

impl HeaderRef {
    pub const OFFSET: MsgOffset = 0;
    pub const LEN: MsgOffset = HEADER_LEN;

    #[inline(always)]
    pub fn offset(&self) -> MsgOffset {
        Self::OFFSET
    }

    #[inline(always)]
    pub fn end(&self) -> MsgOffset {
        Self::OFFSET + Self::LEN
    }
}

/// Reference to a question record.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuestionRef {
    /// Offset to the start of the question (first byte of QNAME).
    pub offset: MsgOffset,
    /// Length in bytes (QNAME + QTYPE + QCLASS).
    pub len: MsgOffset,
}

impl QuestionRef {
    #[inline]
    pub fn new(offset: MsgOffset, len: MsgOffset) -> Self {
        Self { offset, len }
    }

    #[inline]
    pub fn end(&self) -> MsgOffset {
        self.offset.saturating_add(self.len)
    }
}

/// Reference to a resource record.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ResourceRecordRef {
    /// Offset to the start of the RR (first byte of NAME).
    pub offset: MsgOffset,
    /// Length in bytes (NAME + TYPE + CLASS + TTL + RDLENGTH + RDATA).
    pub len: MsgOffset,
}

impl ResourceRecordRef {
    #[inline]
    pub fn new(offset: MsgOffset, len: MsgOffset) -> Self {
        Self { offset, len }
    }

    #[inline]
    pub fn end(&self) -> MsgOffset {
        self.offset.saturating_add(self.len)
    }
}

/// Reference to a domain name (sequence of name elements).
/// Stores only the offset where the name starts; elements are parsed on demand.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NameRef {
    /// Offset to the first byte of the name (label, pointer, or root).
    pub offset: MsgOffset,
}

impl NameRef {
    #[inline]
    pub fn new(offset: MsgOffset) -> Self {
        Self { offset }
    }
}

// -----------------------------------------------------------------------------
// Pointer helpers
// -----------------------------------------------------------------------------

/// Decode the 14-bit target offset from a pointer at `buf[offset..offset+2]`.
/// Pointer format: [0b11xxxxxx][xxxxxxxx] — lower 6 bits of first byte + 8 bits of second.
#[inline]
pub fn decode_pointer_offset(buf: &[u8], offset: MsgOffset) -> Option<MsgOffset> {
    let off = offset as usize;
    if off + 2 > buf.len() {
        return None;
    }
    let b0 = buf[off];
    let b1 = buf[off + 1] as u16;
    if (b0 & POINTER_MASK) != POINTER_MASK {
        return None;
    }
    Some(((b0 & POINTER_HIGH_MASK) as u16) << 8 | b1)
}

/// Encode a pointer at `buf[offset..offset+2]` pointing to `target`.
#[inline]
pub fn encode_pointer(buf: &mut [u8], offset: MsgOffset, target: MsgOffset) -> Option<()> {
    let off = offset as usize;
    if off + 2 > buf.len() || target > POINTER_OFFSET_MASK {
        return None;
    }
    buf[off] = ((target >> 8) | (POINTER_MASK as u16)) as u8;
    buf[off + 1] = (target & 0xFF) as u8;
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_label() {
        assert_eq!(
            NameElementRef::classify_first_byte(0),
            NameElementKind::Root
        );
        assert_eq!(
            NameElementRef::classify_first_byte(1),
            NameElementKind::Label
        );
        assert_eq!(
            NameElementRef::classify_first_byte(63),
            NameElementKind::Label
        );
        assert_eq!(
            NameElementRef::classify_first_byte(0xC0),
            NameElementKind::Pointer
        );
        assert_eq!(
            NameElementRef::classify_first_byte(0xFF),
            NameElementKind::Pointer
        );
        assert_eq!(
            NameElementRef::classify_first_byte(0x40),
            NameElementKind::Reserved
        );
        assert_eq!(
            NameElementRef::classify_first_byte(0x80),
            NameElementKind::Reserved
        );
    }

    #[test]
    fn decode_encode_pointer() {
        let mut buf = [0u8; 4];
        encode_pointer(&mut buf, 0, 12).unwrap();
        assert_eq!(buf[0], 0xC0);
        assert_eq!(buf[1], 12);
        assert_eq!(decode_pointer_offset(&buf, 0), Some(12));

        encode_pointer(&mut buf, 2, 0x3FFF).unwrap();
        assert_eq!(decode_pointer_offset(&buf, 2), Some(0x3FFF));
    }

    #[test]
    fn header_ref() {
        let h = HeaderRef;
        assert_eq!(h.offset(), 0);
        assert_eq!(h.end(), 12);
    }
}
