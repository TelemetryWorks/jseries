use std::fmt;

pub const INFORMATION_BITS: u8 = 70;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InformationWord(u128);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BitRange {
    lsb: u8,
    width: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WordError {
    HighBitsSet,
    InvalidRange,
}

impl fmt::Display for WordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::HighBitsSet => "input has set bits above the 70-bit information word",
            Self::InvalidRange => "field range is empty or exceeds the 70-bit information word",
        })
    }
}

impl std::error::Error for WordError {}

pub(crate) const fn mask(width: u8) -> u128 {
    if width == 128 {
        u128::MAX
    } else {
        (1u128 << width) - 1
    }
}

impl InformationWord {
    pub fn new(value: u128) -> Result<Self, WordError> {
        if value & !mask(INFORMATION_BITS) != 0 {
            return Err(WordError::HighBitsSet);
        }
        Ok(Self(value))
    }

    pub const fn value(self) -> u128 {
        self.0
    }

    pub fn extract(self, range: BitRange) -> u128 {
        (self.0 >> range.lsb) & mask(range.width)
    }
}

impl BitRange {
    pub fn new(lsb: u8, width: u8) -> Result<Self, WordError> {
        if width == 0 || u16::from(lsb) + u16::from(width) > u16::from(INFORMATION_BITS) {
            return Err(WordError::InvalidRange);
        }
        Ok(Self { lsb, width })
    }

    pub const fn lsb(self) -> u8 {
        self.lsb
    }

    pub const fn width(self) -> u8 {
        self.width
    }
}
