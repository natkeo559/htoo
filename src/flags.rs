use bitflags::bitflags;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[repr(transparent)]
#[derive(Debug, Default, PartialEq, Eq, IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct Flags(pub u8);

bitflags! {
    impl Flags: u8 {
        /// `0x00`: NONE (No flags)
        const NONE = 0x00;
        /// `0x01`: ACK
        const ACK = 0x01;
        /// `0x01`: END_STREAM
        const END_STREAM = 0x01;
        /// `0x04`: END_HEADERS
        const END_HEADERS = 0x04;
        /// `0x08`: PADDED
        const PADDED = 0x08;
        /// `0x20`: PRIORITY
        const PRIORITY = 0x20;
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self::from_bits_retain(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitflags() {
        let bytes = [0x01u8, 0x08, 0x00, 0x01].as_bytes();
        let flag = Flags::ref_from_prefix(bytes);
        let (flag, bytes) = flag.unwrap();

        let flag2 = Flags::ref_from_prefix(bytes);
        let (flag2, bytes) = flag2.unwrap();

        let flag3 = Flags::ref_from_prefix(bytes);
        let (flag3, bytes) = flag3.unwrap();

        let flag4 = Flags::ref_from_prefix(bytes);
        let (flag4, _bytes) = flag4.unwrap();

        assert_eq!(&Flags::END_STREAM, flag);
        assert_eq!(&Flags::PADDED, flag2);
        assert_eq!(&Flags::NONE, flag3);
        assert_eq!(&Flags::END_STREAM, flag4);
        assert_eq!(&Flags::ACK, flag4);
    }
}
